use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
}

impl Paddle {
    pub fn player1() -> Self {
        Self {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        }
    }

    pub fn player2() -> Self {
        Self {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        }
    }
} 