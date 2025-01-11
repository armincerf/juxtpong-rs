use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::Player;

#[derive(Default, Resource)]
pub struct Score(pub HashMap<Player, i32>); 