use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::components::{Ball, BallBundle, Paddle, Player};
use crate::components::constants::*;
use super::events::GameEvents;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn spawn_border(mut commands: Commands) {
    // Top wall
    commands.spawn((
        Transform::from_translation(Vec3::new(0., WINDOW_HIGHT / 2., 0.)),
        Visibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(WINDOW_WIDTH / 2., 3.),
    ));

    // Bottom wall
    commands.spawn((
        Transform::from_translation(Vec3::new(0., -WINDOW_HIGHT / 2., 0.)),
        Visibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(WINDOW_WIDTH / 2., 3.),
    ));

    // Right goal
    commands.spawn((
        Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2., 0., 0.)),
        Visibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(3., WINDOW_HIGHT / 2.),
        Player::Player1,
        Sensor,
    ));

    // Left goal
    commands.spawn((
        Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2., 0., 0.)),
        Visibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(3., WINDOW_HIGHT / 2.),
        Player::Player2,
        Sensor,
    ));
}

pub fn spawn_players(mut commands: Commands) {
    // Left paddle (Player 1)
    commands.spawn((
        Sprite {
            color: Player::Player1.get_color(),
            custom_size: Some(Vec2::new(PADDLE_WIDTH * 2.0, PADDLE_HEIGHT)),
            flip_x: false,
            ..default()
        },
        Transform {
            translation: Vec3::new(-WINDOW_WIDTH / 2. + PADDLE_WIDTH * 2.0, 0., 0.),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
            ..default()
        },
        Player::Player1,
        Paddle::player1(),
        RigidBody::KinematicPositionBased,
        Collider::triangle(
            Vec2::new(-PADDLE_WIDTH, -PADDLE_HEIGHT/2.),
            Vec2::new(-PADDLE_WIDTH, PADDLE_HEIGHT/2.),
            Vec2::new(PADDLE_WIDTH, 0.0),
        ),
    ));

    // Right paddle (Player 2)
    commands.spawn((
        Sprite {
            color: Player::Player2.get_color(),
            custom_size: Some(Vec2::new(PADDLE_WIDTH * 2.0, PADDLE_HEIGHT)),
            flip_x: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(WINDOW_WIDTH / 2. - PADDLE_WIDTH * 2.0, 0., 0.),
            rotation: Quat::from_rotation_z(-std::f32::consts::PI / 2.0),
            ..default()
        },
        Player::Player2,
        Paddle::player2(),
        RigidBody::KinematicPositionBased,
        Collider::triangle(
            Vec2::new(PADDLE_WIDTH, -PADDLE_HEIGHT/2.),
            Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT/2.),
            Vec2::new(-PADDLE_WIDTH, 0.0),
        ),
    ));
}

pub fn spawn_ball(mut commands: Commands) {
    commands.spawn(BallBundle::default());
}

pub fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += PADDLE_SPEED * time.delta_secs();
            pos.translation.y = pos
                .translation
                .y
                .clamp((-WINDOW_HIGHT / 2.) + PADDLE_HEIGHT/2., (WINDOW_HIGHT / 2.) - PADDLE_HEIGHT/2.);
        }

        if input.pressed(settings.move_down) {
            pos.translation.y -= PADDLE_SPEED * time.delta_secs();
            pos.translation.y = pos
                .translation
                .y
                .clamp((-WINDOW_HIGHT / 2.) + PADDLE_HEIGHT/2., (WINDOW_HIGHT / 2.) - PADDLE_HEIGHT/2.);
        }
    }
}

pub fn ball_hit(
    paddles: Query<&Player, With<Paddle>>,
    mut balls: Query<(&CollidingEntities, &mut Sprite, &mut Velocity, &mut Ball), With<Ball>>,
) {
    for (hits, mut sprite, mut velocity, mut ball) in &mut balls {
        for hit in hits.iter() {
            if let Ok(player) = paddles.get(hit) {
                sprite.color = player.get_color();
                velocity.angvel = 5.0;
                let random_y = rand::thread_rng().gen_range(-100.0..100.0);
                velocity.linvel.y = random_y;
                velocity.linvel.x = if player == &Player::Player1 { BALL_SPEED } else { -BALL_SPEED };
                ball.time_since_hit = 0.0;
                return;
            }
        }
    }
}

pub fn detect_reset(
    input: Res<ButtonInput<KeyCode>>,
    balls: Query<&CollidingEntities, With<Ball>>,
    goals: Query<&Player, With<Sensor>>,
    mut game_events: EventWriter<GameEvents>,
) {
    if input.just_pressed(KeyCode::Space) {
        let player = if rand::thread_rng().gen::<bool>() {
            Player::Player1
        } else {
            Player::Player2
        };
        game_events.send(GameEvents::ResetBall(player));
        return;
    }

    if input.just_pressed(KeyCode::KeyR) {
        if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
            game_events.send(GameEvents::ResetScore);
            game_events.send(GameEvents::ResetBall(Player::Player1));
        } else {
            game_events.send(GameEvents::ResetBall(Player::Player1));
        }
        return;
    }

    for ball in &balls {
        for hit in ball.iter() {
            if let Ok(player) = goals.get(hit) {
                game_events.send(GameEvents::ResetBall(*player));
                game_events.send(GameEvents::GainPoint(*player));
            }
        }
    }
}

pub fn reset_ball(
    mut balls: Query<(&mut Transform, &mut Velocity, &mut Ball), With<Ball>>,
    mut game_events: EventReader<GameEvents>,
) {
    for events in game_events.read() {
        match events {
            GameEvents::ResetBall(player) => {
                for (mut transform, mut velocity, mut ball) in &mut balls {
                    transform.translation = Vec3::ZERO;
                    *velocity = player.start_speed();
                    ball.time_since_hit = 0.0;
                }
            }
            _ => {}
        }
    }
} 