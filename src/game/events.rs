use bevy::prelude::*;
use crate::components::Player;

#[derive(Event)]
pub enum GameEvents {
    ResetBall(Player),
    GainPoint(Player),
    ResetScore,
} 