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
    match b {
        Some(b) => {
            let min = a.min(b);
            let max = a.max(b);
            min + (pyxel::Pyxel::random_float(0.0, 1.0) * (max - min))
        },
        None => pyxel::Pyxel::random_float(0.0, a),
    }
}

/// Random integer between a and b
pub fn rnd_int(a: i32, b: Option<i32>) -> i32 {
    match b {
        Some(b) => {
            let min = a.min(b);
            let max = a.max(b);
            pyxel::Pyxel::random_int(min, max)
        },
        None => pyxel::Pyxel::random_int(0, a),
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
