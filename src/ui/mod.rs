use bevy::prelude::*;

mod resources;
mod systems;

pub use resources::*;
use systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, spawn_score)
            .add_systems(Update, score);
    }
} 