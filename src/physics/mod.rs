use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod systems;
use systems::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        
        #[cfg(debug_assertions)]
        app.add_plugins(RapierDebugRenderPlugin::default());

        app.add_systems(Update, (
            apply_wind,
            spawn_wind_particles,
            update_wind_particles,
        ));
    }
} 