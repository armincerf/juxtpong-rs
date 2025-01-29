use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::components::{Ball, BallBundle, Paddle, Player, GameCamera};
use crate::components::constants::*;
use super::events::GameEvents;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        GameCamera,
    ));
}

pub fn spawn_border(mut commands: Commands) {
    // Left wall
    commands.spawn((
        Transform::from_translation(Vec3::new(-VIRTUAL_WIDTH / 2., 0., 0.)),
        Visibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(3., VIRTUAL_HEIGHT / 2.),
    ));

    // Right wall
    commands.spawn((
        Transform::from_translation(Vec3::new(VIRTUAL_WIDTH / 2., 0., 0.)),
        Visibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(3., VIRTUAL_HEIGHT / 2.),
    ));

    // Top goal (Player 1)
    commands.spawn((
        Transform::from_translation(Vec3::new(0., VIRTUAL_HEIGHT / 2., 0.)),
        Visibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(VIRTUAL_WIDTH / 2., 3.),
        Player::Player1,
        Sensor,
    ));

    // Bottom goal (Player 2)
    commands.spawn((
        Transform::from_translation(Vec3::new(0., -VIRTUAL_HEIGHT / 2., 0.)),
        Visibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(VIRTUAL_WIDTH / 2., 3.),
        Player::Player2,
        Sensor,
    ));
}

pub fn spawn_players(mut commands: Commands) {
    // Helper function to create paddle vertices based on player
    let create_paddle_vertices = |is_player1: bool| {
        let (base_y, tip_y) = if is_player1 {
            (PADDLE_WIDTH, -PADDLE_WIDTH)
        } else {
            (-PADDLE_WIDTH, PADDLE_WIDTH)
        };
        
        vec![
            Vec2::new(-PADDLE_HEIGHT/2., base_y),
            Vec2::new(PADDLE_HEIGHT/2., base_y),
            Vec2::new(0.0, tip_y),
        ]
    };

    // Helper function to spawn a paddle
    let spawn_paddle = |commands: &mut Commands, player: Player| {
        let is_player1 = matches!(player, Player::Player1);
        let vertices = create_paddle_vertices(is_player1);
        let y_pos = if is_player1 {
            VIRTUAL_HEIGHT / 2. - PADDLE_WIDTH * 2.0
        } else {
            -VIRTUAL_HEIGHT / 2. + PADDLE_WIDTH * 2.0
        };

        commands.spawn((
            Sprite {
                color: player.get_color(),
                custom_size: None, // Remove custom_size to use mesh instead
                flip_y: !is_player1,
                ..default()
            },
            Transform {
                translation: Vec3::new(0., y_pos, 0.),
                ..default()
            },
            player,
            if is_player1 { Paddle::player1() } else { Paddle::player2() },
            RigidBody::KinematicPositionBased,
            Collider::triangle(vertices[0], vertices[1], vertices[2]),
            Friction::coefficient(0.8), // Add significant friction to affect ball spin
            Restitution::coefficient(1.0), // Make sure the ball bounces fully
        ));
    };

    // Spawn both paddles
    spawn_paddle(&mut commands, Player::Player1);
    spawn_paddle(&mut commands, Player::Player2);
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
        if input.pressed(settings.move_right) {
            pos.translation.x += PADDLE_SPEED * time.delta_secs();
            pos.translation.x = pos
                .translation
                .x
                .clamp((-VIRTUAL_WIDTH / 2.) + PADDLE_HEIGHT/2., (VIRTUAL_WIDTH / 2.) - PADDLE_HEIGHT/2.);
        }

        if input.pressed(settings.move_left) {
            pos.translation.x -= PADDLE_SPEED * time.delta_secs();
            pos.translation.x = pos
                .translation
                .x
                .clamp((-VIRTUAL_WIDTH / 2.) + PADDLE_HEIGHT/2., (VIRTUAL_WIDTH / 2.) - PADDLE_HEIGHT/2.);
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
                // Add spin but preserve the existing velocity
                velocity.angvel = 10.0; // Increased spin
                // Add some random horizontal variation while preserving the current speed
                let current_speed = velocity.linvel.length();
                let random_factor = rand::thread_rng().gen_range(0.8..1.2);
                velocity.linvel = velocity.linvel * random_factor;
                // Normalize and maintain the original speed
                velocity.linvel = velocity.linvel.normalize() * current_speed;
                // Ensure minimum vertical velocity after bounce while preserving direction
                let min_y_vel = BALL_SPEED * 0.5;
                if velocity.linvel.y.abs() < min_y_vel {
                    velocity.linvel.y = if velocity.linvel.y > 0.0 { min_y_vel } else { -min_y_vel };
                    // Re-normalize to maintain speed after adjusting y velocity
                    velocity.linvel = velocity.linvel.normalize() * current_speed;
                }
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