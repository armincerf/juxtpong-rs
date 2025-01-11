mod game;
mod ui;
mod physics;
mod components;

use bevy::{
    prelude::*,
    window::WindowResolution,
};
use components::constants::*;

fn main() {
    let mut app = App::new();
    
    // Core plugins and window setup
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HIGHT),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }));

    // Add game systems
    app.add_plugins(game::GamePlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(ui::UiPlugin);

    app.run();
}
