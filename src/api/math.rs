/// Math API - Utility functions for game logic
/// 
/// Common mathematical functions for game development.

/// Return the sign of x: -1, 0, or 1
pub fn sgn(x: f32) -> i32 {
    if x > 0.0 {
        1
    } else if x < 0.0 {
        -1
    } else {
        0
    }
}

/// Absolute value
pub fn abs(x: f32) -> f32 {
    x.abs()
}

// xorshift32 PRNG — no SystemTime, works on WASM
static RNG_STATE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(12345);

fn next_rand() -> f32 {
    use std::sync::atomic::Ordering;
    let mut x = RNG_STATE.load(Ordering::Relaxed);
    if x == 0 { x = 12345; }
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    RNG_STATE.store(x, Ordering::Relaxed);
    (x as f32) / (u32::MAX as f32)
}

/// Seed the RNG (call with a timestamp or frame counter)
pub fn rseed(seed: u32) {
    use std::sync::atomic::Ordering;
    RNG_STATE.store(if seed == 0 { 1 } else { seed }, Ordering::Relaxed);
}

/// Random float between a and b
pub fn rnd(a: f32, b: Option<f32>) -> f32 {
    let r = next_rand();
    match b {
        Some(b) => { let lo = a.min(b); let hi = a.max(b); lo + r * (hi - lo) },
        None => r * a,
    }
}

/// Random integer between a and b
pub fn rnd_int(a: i32, b: Option<i32>) -> i32 {
    let f = rnd(0.0, None);
    match b {
        Some(b) => { let lo = a.min(b); let hi = a.max(b); lo + (f * (hi - lo + 1) as f32) as i32 },
        None => (f * a as f32) as i32,
    }
}

/// Clamp value between min and max
pub fn clamp(x: f32, a: f32, b: f32) -> f32 {
    x.max(a).min(b)
}

/// Mid value (minimum of max(a, b) and min(c, d) relative to three values)
/// Returns the value closest to b among a, b, c
pub fn mid(a: f32, b: f32, c: f32) -> f32 {
    if a < b {
        if b < c {
            b
        } else if a < c {
            c
        } else {
            a
        }
    } else if a < c {
        a
    } else if b < c {
        c
    } else {
        b
    }
}

/// Maximum of two values
pub fn max(a: f32, b: f32) -> f32 {
    a.max(b)
}

/// Minimum of two values
pub fn min(a: f32, b: f32) -> f32 {
    a.min(b)
}

/// Random float between a and b (alias for rnd with both bounds)
pub fn rndf(a: f32, b: f32) -> f32 {
    rnd(a, Some(b))
}

/// Sine of angle in degrees (Pyxel-compatible: pyxel.sin(deg))
pub fn sin(deg: f32) -> f32 {
    (deg * std::f32::consts::PI / 180.0).sin()
}

/// Cosine of angle in degrees
pub fn cos(deg: f32) -> f32 {
    (deg * std::f32::consts::PI / 180.0).cos()
}

/// Square root
pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

/// Floor division (floor of x)
pub fn floor(x: f32) -> i32 {
    x.floor() as i32
}

