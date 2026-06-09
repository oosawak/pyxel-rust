use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
struct InputDoc {
    title: String,
    body: String,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Clone)]
struct Doc {
    title: String,
    body: String,
    source: Option<String>,
    tags: Vec<String>,
}

#[derive(Serialize)]
struct RankedDoc {
    title: String,
    score: f32,
    snippet: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    tags: Vec<String>,
}

#[derive(Serialize)]
struct QueryResult {
    query: String,
    answer: String,
    prompt: String,
    system_prompt: String,
    document_count: usize,
    matches: Vec<RankedDoc>,
}

thread_local! {
    static DOCS: RefCell<Vec<Doc>> = const { RefCell::new(Vec::new()) };
    static SYSTEM_PROMPT: RefCell<String> = RefCell::new(String::from(
        "あなたは観光案内のアシスタントです。与えられた観光データだけを優先して簡潔に答えてください。"
    ));
    static CONTEXT_LIMIT: RefCell<usize> = RefCell::new(1200);
}

fn normalize(text: &str) -> Vec<char> {
    text.chars()
        .filter(|c| !c.is_whitespace() && !matches!(c, '、' | '。' | '，' | '．' | ',' | '.' | '!' | '?' | '！' | '？'))
        .collect()
}

fn ngrams(text: &str, n: usize) -> Vec<String> {
    let chars = normalize(text);
    if chars.is_empty() {
        return Vec::new();
    }
    if chars.len() <= n {
        return vec![chars.iter().collect()];
    }
    chars
        .windows(n)
        .map(|w| w.iter().collect::<String>())
        .collect()
}

fn unique_tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_alphanumeric() || ('\u{3040}'..='\u{30ff}').contains(&ch) || ('\u{4e00}'..='\u{9fff}').contains(&ch) {
            current.push(ch);
        } else if !current.is_empty() {
            tokens.push(current.clone());
            current.clear();
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens.sort();
    tokens.dedup();
    tokens
}

fn score(query: &str, body: &str) -> f32 {
    let qgrams = ngrams(query, 2);
    let bgrams = ngrams(body, 2);
    if qgrams.is_empty() || bgrams.is_empty() {
        return 0.0;
    }

    let mut matched = 0.0;
    for q in &qgrams {
        if bgrams.iter().any(|b| b == q) {
            matched += 1.0;
        }
    }
    let gram_score = matched / qgrams.len() as f32;

    let q_tokens = unique_tokens(query);
    let b_tokens = unique_tokens(body);
    let token_hits = q_tokens
        .iter()
        .filter(|q| b_tokens.iter().any(|b| b.contains(*q) || q.contains(b)))
        .count() as f32;
    let token_score = if q_tokens.is_empty() { 0.0 } else { token_hits / q_tokens.len() as f32 };

    (gram_score * 0.65) + (token_score * 0.35)
}

fn snippet(text: &str, max_chars: usize) -> String {
    let mut out = String::new();
    for ch in text.chars().take(max_chars) {
        out.push(ch);
    }
    if text.chars().count() > max_chars {
        out.push('…');
    }
    out
}

fn system_prompt() -> String {
    SYSTEM_PROMPT.with(|prompt| prompt.borrow().clone())
}

fn read_context_limit() -> usize {
    CONTEXT_LIMIT.with(|limit| *limit.borrow())
}

fn compose_context(matches: &[RankedDoc]) -> String {
    let mut context = String::new();
    let limit = read_context_limit();
    for (i, item) in matches.iter().enumerate() {
        let block = format!(
            "[{}] {}{}\n{}\n\n",
            i + 1,
            item.title,
            item.source.as_ref().map(|s| format!(" ({s})")).unwrap_or_default(),
            item.snippet
        );
        if context.len() + block.len() > limit {
            break;
        }
        context.push_str(&block);
    }
    context
}

fn build_prompt(query: &str, context: &str) -> String {
    format!(
        "{}\n\n### 観光コンテキスト\n{}\n### 質問\n{}\n\n### 回答方針\n- 断定しすぎず、観光案内として自然に答える\n- 必要なら候補を箇条書きで出す\n- コンテキストにない内容は推測しない",
        system_prompt(),
        context,
        query
    )
}

fn compose_answer(query: &str, matches: &[RankedDoc]) -> String {
    if matches.is_empty() {
        return format!("「{query}」に一致する観光情報はまだ登録されていません。");
    }

    let best = &matches[0];
    let mut lines = vec![format!("「{query}」に近い候補は「{}」です。", best.title)];
    lines.push(best.snippet.clone());
    if let Some(source) = &best.source {
        lines.push(format!("出典: {source}"));
    }
    lines.join("\n")
}

fn rank(query: &str) -> Vec<RankedDoc> {
    let mut ranked: Vec<RankedDoc> = DOCS.with(|docs| {
        docs.borrow()
            .iter()
            .map(|doc| RankedDoc {
                title: doc.title.clone(),
                score: score(query, &doc.body),
                snippet: snippet(&doc.body, 96),
                source: doc.source.clone(),
                tags: doc.tags.clone(),
            })
            .collect()
    });

    ranked.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    ranked
        .into_iter()
        .filter(|doc| doc.score > 0.0)
        .take(5)
        .collect()
}

#[wasm_bindgen]
pub fn clear_documents() {
    DOCS.with(|docs| docs.borrow_mut().clear());
}

#[wasm_bindgen]
pub fn add_document(title: String, body: String) {
    DOCS.with(|docs| docs.borrow_mut().push(Doc {
        title,
        body,
        source: None,
        tags: Vec::new(),
    }));
}

#[wasm_bindgen]
pub fn load_documents(json_text: &str) -> Result<(), JsValue> {
    let parsed: Vec<InputDoc> = serde_json::from_str(json_text)
        .map_err(|e| JsValue::from_str(&format!("invalid corpus json: {e}")))?;

    DOCS.with(|docs| {
        let mut target = docs.borrow_mut();
        target.clear();
        target.extend(parsed.into_iter().map(|doc| Doc {
            title: doc.title,
            body: doc.body,
            source: doc.source,
            tags: doc.tags,
        }));
    });
    Ok(())
}

#[wasm_bindgen]
pub fn document_count() -> usize {
    DOCS.with(|docs| docs.borrow().len())
}

#[wasm_bindgen]
pub fn set_system_prompt(prompt: &str) {
    SYSTEM_PROMPT.with(|p| {
        let mut value = p.borrow_mut();
        value.clear();
        value.push_str(prompt);
    });
}

#[wasm_bindgen]
pub fn system_prompt_text() -> String {
    system_prompt()
}

#[wasm_bindgen]
pub fn set_context_limit(limit: usize) {
    CONTEXT_LIMIT.with(|v| *v.borrow_mut() = limit.max(200));
}

#[wasm_bindgen]
pub fn context_limit() -> usize {
    read_context_limit()
}

#[wasm_bindgen]
pub fn build_prompt_text(query: &str) -> String {
    let matches = rank(query);
    let context = compose_context(&matches);
    build_prompt(query, &context)
}

#[wasm_bindgen]
pub fn query_guide(query: &str) -> String {
    let matches = rank(query);
    let context = compose_context(&matches);
    let document_count = DOCS.with(|docs| docs.borrow().len());
    let payload = QueryResult {
        query: query.to_string(),
        answer: compose_answer(query, &matches),
        prompt: build_prompt(query, &context),
        system_prompt: system_prompt(),
        document_count,
        matches,
    };

    serde_json::to_string(&payload).unwrap_or_else(|e| {
        format!(
            "{{\"query\":{},\"answer\":\"failed to serialize result: {}\",\"document_count\":0,\"matches\":[]}}",
            serde_json::to_string(query).unwrap_or_else(|_| "\"\"".to_string()),
            e
        )
    })
}
