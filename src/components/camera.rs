use bevy::prelude::*;
use super::constants::*;

#[derive(Component)]
pub struct GameCamera;

#[derive(Resource)]
pub struct CameraScaling {
    pub scale_factor: f32,
}

impl Default for CameraScaling {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
        }
    }
}

pub fn update_camera_scaling(
    mut scaling: ResMut<CameraScaling>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let window_aspect = window.width() / window.height();
    let virtual_aspect = VIRTUAL_WIDTH / VIRTUAL_HEIGHT;

    scaling.scale_factor = if window_aspect > virtual_aspect {
        // Window is wider than virtual aspect - scale by height
        window.height() / VIRTUAL_HEIGHT
    } else {
        // Window is taller than virtual aspect - scale by width
        window.width() / VIRTUAL_WIDTH
    };
}

pub fn update_camera(
    scaling: Res<CameraScaling>,
    mut camera_query: Query<&mut OrthographicProjection, With<GameCamera>>,
) {
    if let Ok(mut projection) = camera_query.get_single_mut() {
        projection.scale = 1.0 / scaling.scale_factor;
    }
} 