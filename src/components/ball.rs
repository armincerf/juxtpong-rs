use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::constants::*;

#[derive(Component)]
pub struct Ball {
    pub time_since_hit: f32,
}

#[derive(Bundle)]
pub struct BallBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub ball: Ball,
    pub body: RigidBody,
    pub colliding_entities: CollidingEntities,
    pub active_events: ActiveEvents,
    pub collider: Collider,
    pub velocity: Velocity,
    pub restitution: Restitution,
    pub ccd: Ccd,
    pub gravity: GravityScale,
    pub damping: Damping,
    pub external_force: ExternalForce,
}

impl Default for BallBundle {
    fn default() -> Self {
        Self {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(BALL_RADIUS * 2., BALL_RADIUS * 2.)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ball: Ball { time_since_hit: 0.0 },
            body: RigidBody::Dynamic,
            colliding_entities: CollidingEntities::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::ball(BALL_RADIUS),
            velocity: Velocity::linear(Vec2::new(BALL_SPEED, 0.)),
            restitution: Restitution {
                coefficient: 0.8,
                combine_rule: CoefficientCombineRule::Max,
            },
            ccd: Ccd::enabled(),
            gravity: GravityScale(0.0),
            damping: Damping { 
                linear_damping: 0.1,
                angular_damping: 0.2,
            },
            external_force: ExternalForce::default(),
        }
    }
} 