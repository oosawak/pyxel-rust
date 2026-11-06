//! Bake the official finite MML effects for the browser sample ports.
use pyxel_rust::Sound;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args()
        .nth(1)
        .unwrap_or("docs/examples/cube/audio".into());
    std::fs::create_dir_all(&output)?;
    let sounds: serde_json::Value = serde_json::from_str(include_str!(
        "../src/games/cube_examples/assets/sounds.json"
    ))?;
    for entry in sounds.as_array().unwrap() {
        let sound = Sound::new();
        let mut sound = sound.lock().unwrap();
        sound.set_mml(entry["mml"].as_str().unwrap())?;
        let duration = sound
            .total_seconds()
            .ok_or("Effect must have a finite duration")?;
        let name = format!(
            "{output}/c{}-{}.wav",
            entry["example"].as_u64().unwrap() + 1,
            entry["sound"]
        );
        sound.save(&name, duration, Some(false))?;
        println!("{name}: {duration:.3} seconds");
    }
    Ok(())
}
