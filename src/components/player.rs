use bevy::{
    prelude::*,
    color::palettes::css::{GREEN, RED},
};
use bevy_rapier2d::prelude::Velocity;

use super::constants::BALL_SPEED;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn start_speed(&self) -> Velocity {
        match self {
            Player::Player1 => Velocity::linear(Vec2::new(0., BALL_SPEED)),
            Player::Player2 => Velocity::linear(Vec2::new(0., -BALL_SPEED)),
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Player::Player1 => RED.into(),
            Player::Player2 => GREEN.into(),
        }
    }
} 