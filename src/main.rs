mod game;
mod ui;
mod physics;
mod components;

use bevy::{
    prelude::*,
    window::{WindowResolution, WindowMode},
};
use components::{constants::*, camera::*};

fn main() {
    let mut app = App::new();
    
    // Core plugins and window setup
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        }),
        ..Default::default()
    }));

    // Add camera scaling resource and systems
    app.init_resource::<CameraScaling>()
        .add_systems(Update, (update_camera_scaling, update_camera));

    // Add game systems
    app.add_plugins(game::GamePlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(ui::UiPlugin);

    app.run();
}
