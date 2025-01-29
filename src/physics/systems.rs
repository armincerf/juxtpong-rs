use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::components::{Ball, WindParticle};
use crate::components::constants::*;

pub fn apply_wind(
    time: Res<Time>,
    mut balls: Query<(&Transform, &mut ExternalForce, &mut Ball, &mut Velocity), With<Ball>>,
) {
    for (transform, mut external_force, mut ball, mut velocity) in &mut balls {
        // Increment time since last hit
        ball.time_since_hit += time.delta().as_secs_f32();
        
        let y = transform.translation.y;
        let direction = if y > 0.0 { 1.0 } else { -1.0 };
        
        // Calculate exponential force based on time since last hit
        let force = WIND_BASE_FORCE * 
            (WIND_GROWTH_RATE.powf(ball.time_since_hit / WIND_TIME_SCALE))
                .min(WIND_MAX_MULTIPLIER);
        
        external_force.force = Vec2::new(0.0, direction * force);

        // Cap the ball's velocity while preserving direction
        if velocity.linvel.length() > MAX_BALL_SPEED {
            velocity.linvel = velocity.linvel.normalize() * MAX_BALL_SPEED;
        }

        // Add minimum vertical velocity in wind direction to prevent pure horizontal bouncing
        if velocity.linvel.y.abs() < MIN_BALL_SPEED {
            velocity.linvel.y = direction * MIN_BALL_SPEED;
        }
    }
}

pub fn spawn_wind_particles(
    mut commands: Commands,
    ball_query: Query<&Transform, (With<Ball>, Without<WindParticle>)>,
) {
    // Get wind direction based on ball position
    let direction = if let Some(ball_transform) = ball_query.iter().next() {
        if ball_transform.translation.y > 0.0 { 1.0 } else { -1.0 }
    } else {
        return; // No ball found
    };
    
    // Spawn particles across the whole screen
    if rand::thread_rng().gen::<f32>() < 0.1 { // Control particle spawn rate
        let random_x = rand::thread_rng().gen_range(-VIRTUAL_WIDTH/2.0..VIRTUAL_WIDTH/2.0);
        let random_y = rand::thread_rng().gen_range(-VIRTUAL_HEIGHT/2.0..VIRTUAL_HEIGHT/2.0);
        
        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 0.8, 0.2, 1.0),
                custom_size: Some(Vec2::new(1.0, 5.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(random_x, random_y, 0.0))
                .with_rotation(Quat::from_rotation_z(if direction > 0.0 { std::f32::consts::PI * 0.5 } else { -std::f32::consts::PI * 0.5 })),
            WindParticle::new(direction),
        ));
    }
}

pub fn update_wind_particles(
    mut commands: Commands,
    time: Res<Time>,
    ball_query: Query<&Transform, (With<Ball>, Without<WindParticle>)>,
    mut particles: Query<(Entity, &mut WindParticle, &mut Transform, &mut Sprite), With<WindParticle>>,
) {
    // Get current wind direction from ball position
    let direction = if let Some(ball_transform) = ball_query.iter().next() {
        if ball_transform.translation.y > 0.0 { 1.0 } else { -1.0 }
    } else {
        return; // No ball found
    };

    for (entity, mut particle, mut transform, mut sprite) in &mut particles {
        particle.lifetime.tick(time.delta());
        
        // Update particle direction based on current wind
        particle.direction = direction;
        
        // Move particle
        let speed = 200.0;
        transform.translation.y += particle.direction * speed * time.delta().as_secs_f32();
        transform.rotation = Quat::from_rotation_z(if direction > 0.0 { std::f32::consts::PI * 0.5 } else { -std::f32::consts::PI * 0.5 });
        
        // Fade out
        let alpha = 1.0 - particle.lifetime.elapsed_secs() / particle.lifetime.duration().as_secs_f32();
        sprite.color = Color::srgba(0.5, 0.8, 1.0, alpha);
        
        // Remove if lifetime is over or particle moves off screen
        if particle.lifetime.finished() || 
           transform.translation.y.abs() > VIRTUAL_HEIGHT/2.0 {
            commands.entity(entity).despawn();
        }
    }
} 