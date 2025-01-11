use bevy::prelude::*;

mod events;
mod systems;

pub use events::*;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvents>()
            .add_systems(Startup, (
                spawn_camera,
                spawn_players,
                spawn_ball,
                spawn_border,
            ))
            .add_systems(Update, (
                move_paddle,
                detect_reset,
                ball_hit,
            ))
            .add_systems(PostUpdate, reset_ball);
    }
} 