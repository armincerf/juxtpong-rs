use bevy::prelude::*;

#[derive(Component)]
pub struct WindParticle {
    pub lifetime: Timer,
    pub direction: f32, // 1.0 for right, -1.0 for left
}

impl WindParticle {
    pub fn new(direction: f32) -> Self {
        Self {
            lifetime: Timer::from_seconds(1.0, TimerMode::Once),
            direction,
        }
    }
} 