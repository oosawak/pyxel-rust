//! Playback of imported node-transform animations. No Python callbacks are needed.
use crate::cube::{Mat4, RcMesh};

/// A player owns a model's rest pose and restores it when changing clips.
/// Advance with game time in seconds; `fps` must match the GLB import rate.
/// Keep the mesh hierarchy and clip list unchanged while its player is active.
pub struct MeshAnimationPlayer {
    mesh: RcMesh,
    rest: Vec<Mat4>,
    clip: Option<usize>,
    frame: f32,
    fps: f32,
    speed: f32,
    playing: bool,
    looping: bool,
}

impl MeshAnimationPlayer {
    pub fn new(mesh: RcMesh, fps: f32) -> Result<Self, String> {
        if !fps.is_finite() || fps <= 0.0 {
            return Err("Animation fps must be finite and positive".into());
        }
        mesh.borrow().validate()?;
        let rest = mesh
            .borrow()
            .transforms
            .iter()
            .map(|m| *m.borrow())
            .collect();
        Ok(Self {
            mesh,
            rest,
            clip: None,
            frame: 0.0,
            fps,
            speed: 1.0,
            playing: false,
            looping: true,
        })
    }

    pub fn clips(&self) -> Vec<(String, f32)> {
        self.mesh
            .borrow()
            .motions
            .iter()
            .map(|m| {
                let m = m.borrow();
                (m.name.clone(), m.length / self.fps)
            })
            .collect()
    }

    pub fn selected_clip(&self) -> Option<usize> {
        self.clip
    }
    pub fn elapsed(&self) -> f32 {
        self.frame / self.fps
    }
    pub fn is_playing(&self) -> bool {
        self.playing
    }
    pub fn is_looping(&self) -> bool {
        self.looping
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }

    fn length(&self) -> f32 {
        self.clip
            .map_or(0.0, |i| self.mesh.borrow().motions[i].borrow().length)
    }

    pub fn duration(&self) -> f32 {
        self.length() / self.fps
    }

    /// Select None for the original rest pose. A new clip starts from its beginning.
    pub fn select_clip(&mut self, clip: Option<usize>) -> Result<(), String> {
        if let Some(index) = clip {
            let mesh = self.mesh.borrow();
            let motion = mesh
                .motions
                .get(index)
                .ok_or("Animation clip is out of range")?;
            let motion = motion.borrow();
            if !motion.length.is_finite()
                || motion.length < 0.0
                || !(motion.length / self.fps).is_finite()
            {
                return Err("Animation duration must be finite and nonnegative".into());
            }
            if motion
                .channels
                .iter()
                .any(|channel| channel.part_index >= self.rest.len())
            {
                return Err("Animation targets a missing mesh part".into());
            }
        }
        self.clip = clip;
        self.frame = 0.0;
        self.playing = clip.is_some() && self.length() > 0.0;
        self.apply_pose();
        Ok(())
    }

    pub fn set_playing(&mut self, playing: bool) {
        self.playing = playing && self.clip.is_some() && self.length() > 0.0;
        if self.playing && !self.looping && self.frame >= self.length() {
            self.frame = 0.0;
            self.apply_pose();
        }
    }

    pub fn set_looping(&mut self, looping: bool) {
        self.looping = looping;
    }

    pub fn set_speed(&mut self, speed: f32) -> Result<(), String> {
        if !speed.is_finite() || speed <= 0.0 {
            return Err("Animation speed must be finite and positive".into());
        }
        self.speed = speed;
        Ok(())
    }

    /// Seek clamps to the clip endpoints and preserves the play/pause state.
    pub fn seek(&mut self, seconds: f32) -> Result<(), String> {
        if !seconds.is_finite() {
            return Err("Animation seek time must be finite".into());
        }
        self.frame = (seconds.clamp(0.0, self.duration()) * self.fps).min(self.length());
        self.apply_pose();
        Ok(())
    }

    pub fn advance(&mut self, seconds: f32) -> Result<(), String> {
        if !seconds.is_finite() || seconds < 0.0 {
            return Err("Animation delta time must be finite and nonnegative".into());
        }
        let length = self.length();
        if !self.playing || length <= 0.0 {
            return Ok(());
        }
        let next = f64::from(self.frame)
            + f64::from(seconds) * f64::from(self.speed) * f64::from(self.fps);
        if self.looping {
            self.frame = next.rem_euclid(f64::from(length)) as f32;
        } else {
            self.frame = next.min(f64::from(length)) as f32;
            if self.frame >= length {
                self.playing = false;
            }
        }
        self.apply_pose();
        Ok(())
    }

    fn apply_pose(&self) {
        let mesh = self.mesh.borrow();
        for (target, rest) in mesh.transforms.iter().zip(&self.rest) {
            *target.borrow_mut() = *rest;
        }
        if let Some(index) = self.clip {
            // Boundary handling belongs to advance/seek. Sampling without wrapping
            // also makes an explicit seek to the last frame display that endpoint.
            for (part, pose) in mesh.motions[index].borrow().sample(self.frame, false) {
                if let Some(target) = mesh.transforms.get(part) {
                    *target.borrow_mut() = pose;
                }
            }
        }
    }
}
