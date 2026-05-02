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

/// Random float between a and b
pub fn rnd(a: f32, b: Option<f32>) -> f32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;
    let mut h = DefaultHasher::new();
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().subsec_nanos().hash(&mut h);
    let r = (h.finish() as f32) / (u64::MAX as f32); // 0.0..1.0
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
