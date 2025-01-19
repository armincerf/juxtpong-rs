use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle {
    pub move_left: KeyCode,
    pub move_right: KeyCode,
}

impl Paddle {
    pub fn player1() -> Self {
        Self {
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
        }
    }

    pub fn player2() -> Self {
        Self {
            move_left: KeyCode::ArrowLeft,
            move_right: KeyCode::ArrowRight,
        }
    }
} 