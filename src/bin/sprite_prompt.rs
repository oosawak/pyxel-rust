//! sprite-prompt — AI画像生成プロンプト出力ツール
//!
//! スプライトシート用のAI画像生成プロンプトをキャラクター説明付きで出力します。
//!
//! Usage:
//!   sprite-prompt "a cute pink rabbit warrior"
//!   sprite-prompt "red dragon boss" --cols 6 --frame-size 256 --anims "Idle;Walk;Attack;Death"
//!   sprite-prompt "ninja cat" --format json

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "sprite-prompt",
    about = "スプライトシート生成用AIプロンプトを出力するツール",
    long_about = "キャラクター/オブジェクトの説明を受け取り、\
                  ゲームエンジンで直接使用できるスプライトシート生成プロンプトを出力します。",
    version,
)]
struct Args {
    /// スプライトの説明（プロンプト冒頭の生成対象の説明文）
    ///
    /// 例: "a cute pink rabbit warrior"
    ///     "red fire dragon boss monster"
    ///     "sci-fi space marine soldier"
    description: String,

    /// 1フレームの縦横サイズ（ピクセル）
    #[arg(short = 'f', long, default_value = "128")]
    frame_size: u32,

    /// 1行あたりのフレーム数（アニメーションのフレーム数）
    #[arg(short = 'c', long, default_value = "8")]
    cols: u32,

    /// アニメーション行のリスト（セミコロン区切り）
    ///
    /// 省略時はデフォルトの12アニメーション行を使用。
    /// 例: "Idle;Walk;Run;Jump;Attack;Death"
    #[arg(short = 'a', long)]
    anims: Option<String>,

    /// 背景色
    #[arg(long, default_value = "magenta")]
    bg: BackgroundColor,

    /// キャラクターの向き
    #[arg(long, default_value = "right")]
    facing: FacingDirection,

    /// 出力フォーマット
    #[arg(long, default_value = "text")]
    format: OutputFormat,
}

#[derive(ValueEnum, Clone, Debug)]
enum BackgroundColor {
    /// #FF00FF マゼンタ（クロマキー対応）
    Magenta,
    /// #00FF00 グリーン
    Green,
    /// #0000FF ブルー
    Blue,
}

impl BackgroundColor {
    fn hex(&self) -> &'static str {
        match self {
            Self::Magenta => "#FF00FF",
            Self::Green   => "#00FF00",
            Self::Blue    => "#0000FF",
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Magenta => "solid pure magenta (#FF00FF)",
            Self::Green   => "solid pure green (#00FF00)",
            Self::Blue    => "solid pure blue (#0000FF)",
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum FacingDirection {
    /// 右向き（3/4サイドビュー）
    Right,
    /// 左向き（3/4サイドビュー）
    Left,
    /// 正面
    Front,
}

impl FacingDirection {
    fn description(&self) -> &'static str {
        match self {
            Self::Right => "Character must ALWAYS face RIGHT - Use a 3/4 side view (face must always be visible) - NO front-facing views - NO back-facing views",
            Self::Left  => "Character must ALWAYS face LEFT - Use a 3/4 side view (face must always be visible) - NO front-facing views - NO back-facing views",
            Self::Front => "Character must ALWAYS face FRONT - Use a front-facing view with slight 3/4 tilt for depth - NO back-facing views",
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// そのままコピペできるテキスト形式
    Text,
    /// JSON形式（プログラムから扱いやすい）
    Json,
    /// Markdown形式
    Markdown,
}

const DEFAULT_ANIMS: &[(&str, &str)] = &[
    ("Idle",           "subtle breathing loop, duplicate frames allowed"),
    ("Walk",           "loop, repeat frames if needed"),
    ("Run",            "loop, repeat frames if needed"),
    ("Jump",           "squat → rise → peak → fall → land → settle → fill to frame count"),
    ("Melee attack",   "physical motion only, no effects"),
    ("Ranged attack",  "shooting pose only, no projectile"),
    ("Damage",         "hit reaction, recover, loop or hold"),
    ("Turn in place",  "simulate turning using side motion only, NO back view"),
    ("Special attack", "charge and release pose only, no effects"),
    ("Singing",        "body motion only, no symbols"),
    ("Resting",        "sit or lie down, loop/hold"),
    ("Victory pose",   "celebration loop"),
];

fn parse_anims(s: &str) -> Vec<(String, String)> {
    s.split(';')
        .enumerate()
        .map(|(i, name)| {
            let name = name.trim().to_string();
            // デフォルトの説明があれば流用、なければ空
            let desc = DEFAULT_ANIMS.get(i)
                .map(|(_, d)| d.to_string())
                .unwrap_or_default();
            (name, desc)
        })
        .filter(|(n, _)| !n.is_empty())
        .collect()
}

fn build_anim_list(anims: &[(String, String)], cols: u32) -> String {
    anims.iter().enumerate().map(|(i, (name, desc))| {
        if desc.is_empty() {
            format!("{}. {} ({} frames)", i + 1, name, cols)
        } else {
            format!("{}. {} ({})", i + 1, name, desc)
        }
    }).collect::<Vec<_>>().join("\n")
}

fn build_prompt(args: &Args) -> String {
    let anims: Vec<(String, String)> = match &args.anims {
        Some(s) => parse_anims(s),
        None    => DEFAULT_ANIMS.iter().map(|(n, d)| (n.to_string(), d.to_string())).collect(),
    };
    let rows = anims.len() as u32;
    let anim_list = build_anim_list(&anims, args.cols);
    let total_w = args.frame_size * args.cols;
    let total_h = args.frame_size * rows;

    format!(
r#"A production-ready 2D sprite sheet of {description}.

STRICT TECHNICAL REQUIREMENTS:
- Each frame must be exactly {fs}x{fs} pixels
- The entire sheet must be a perfect grid: {cols} columns × {rows} rows
- Total sheet size: {tw}x{th} pixels
- Frames must be tightly packed with NO spacing or padding
- Each animation row must start from the LEFTMOST column
- Remaining frames on the right must still exist but follow the fixed {cols}-frame structure
- Character must be centered horizontally in every frame
- Character feet must be consistently positioned 2 pixels above the bottom edge
- Character scale must remain identical across all frames

BACKGROUND RULE:
- Background must be {bg}
- No gradients, no patterns, no transparency
- No checkerboard transparency

VISUAL CLEANLINESS:
- NO grid lines
- NO guides
- NO borders
- NO text
- NO labels
- NO UI elements
- NO annotations

ORIENTATION RULE (CRITICAL):
- {facing}
- NO camera rotation between frames

ANIMATION STRUCTURE (ALL MUST BE EXACTLY {cols} FRAMES PER ROW):
{anim_list}

IMPORTANT ANIMATION RULES:
- ALL animations must contain exactly {cols} frames
- If fewer frames are needed, duplicate frames to fill to {cols}
- Motion must remain readable and consistent

EFFECT RULE:
- NO visual effects at all
- No glow, no particles, no beams, no magic, no dust

STYLE:
- Clean, game-ready sprite style
- Slight chibi proportions
- Stable silhouette
- Consistent lighting and shading
- Designed for real-time game use

FINAL OUTPUT REQUIREMENT:
- This must be a clean sprite atlas ready for direct use in a game engine"#,
        description = args.description,
        fs          = args.frame_size,
        cols        = args.cols,
        rows        = rows,
        tw          = total_w,
        th          = total_h,
        bg          = args.bg.name(),
        facing      = args.facing.description(),
        anim_list   = anim_list,
    )
}

fn main() {
    let args = Args::parse();

    let prompt = build_prompt(&args);

    match args.format {
        OutputFormat::Text => {
            println!("{}", prompt);
        }
        OutputFormat::Json => {
            let anims: Vec<(String, String)> = match &args.anims {
                Some(s) => parse_anims(s),
                None    => DEFAULT_ANIMS.iter().map(|(n, d)| (n.to_string(), d.to_string())).collect(),
            };
            let rows = anims.len() as u32;
            let anim_names: Vec<&str> = anims.iter().map(|(n, _)| n.as_str()).collect();
            // シンプルな手書きJSON（serde_jsonを使わずに）
            println!("{{");
            println!("  \"description\": {:?},", args.description);
            println!("  \"frame_size\": {},", args.frame_size);
            println!("  \"cols\": {},", args.cols);
            println!("  \"rows\": {},", rows);
            println!("  \"total_width\": {},", args.frame_size * args.cols);
            println!("  \"total_height\": {},", args.frame_size * rows);
            println!("  \"background\": {:?},", args.bg.hex());
            println!("  \"animations\": {:?},", anim_names);
            println!("  \"prompt\": {:?}", prompt);
            println!("}}");
        }
        OutputFormat::Markdown => {
            println!("## Sprite Sheet Prompt\n");
            println!("**Description:** {}\n", args.description);
            println!("**Grid:** {}×{} @ {}px/frame\n", args.cols, {
                let rows = match &args.anims {
                    Some(s) => parse_anims(s).len(),
                    None    => DEFAULT_ANIMS.len(),
                };
                rows
            }, args.frame_size);
            println!("```");
            println!("{}", prompt);
            println!("```");
        }
    }
}
