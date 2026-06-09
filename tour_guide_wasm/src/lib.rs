use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::cell::RefCell;

#[derive(Clone, Serialize, Deserialize)]
struct InputDoc {
    title: String,
    body: String,
}

#[derive(Clone)]
struct Doc {
    title: String,
    body: String,
}

#[derive(Serialize)]
struct RankedDoc {
    title: String,
    score: f32,
    snippet: String,
}

#[derive(Serialize)]
struct QueryResult {
    query: String,
    answer: String,
    document_count: usize,
    matches: Vec<RankedDoc>,
}

thread_local! {
    static DOCS: RefCell<Vec<Doc>> = const { RefCell::new(Vec::new()) };
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
    matched / qgrams.len() as f32
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

fn compose_answer(query: &str, matches: &[RankedDoc]) -> String {
    if matches.is_empty() {
        return format!("「{query}」に一致する観光情報はまだ登録されていません。");
    }

    let best = &matches[0];
    format!(
        "「{query}」に近い候補は「{}」です。{}\n\nGemma4 の推論レイヤーはここに差し込めるようにしてあります。",
        best.title, best.snippet
    )
}

fn rank(query: &str) -> Vec<RankedDoc> {
    let mut ranked: Vec<RankedDoc> = DOCS.with(|docs| {
        docs.borrow()
            .iter()
            .map(|doc| RankedDoc {
                title: doc.title.clone(),
                score: score(query, &doc.body),
                snippet: snippet(&doc.body, 96),
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
    DOCS.with(|docs| docs.borrow_mut().push(Doc { title, body }));
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
        }));
    });
    Ok(())
}

#[wasm_bindgen]
pub fn document_count() -> usize {
    DOCS.with(|docs| docs.borrow().len())
}

#[wasm_bindgen]
pub fn query_guide(query: &str) -> String {
    let matches = rank(query);
    let document_count = DOCS.with(|docs| docs.borrow().len());
    let payload = QueryResult {
        query: query.to_string(),
        answer: compose_answer(query, &matches),
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
