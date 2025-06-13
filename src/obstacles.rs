use std::time::Duration;

use avian2d::prelude::*;
use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

use crate::{
    GameState, WINDOW_HALF_HEIGHT, WINDOW_HALF_WIDTH,
    WINDOW_HEIGHT,
};

const OBSTACLE_WIDTH: f32 = 50.0;
const OBSTACLE_SPEED: f32 = 200.0;
const SPAWN_START_X: f32 = WINDOW_HALF_WIDTH + OBSTACLE_WIDTH;

pub fn obstacle_plugin(app: &mut App) {
    app.insert_resource(SpawnInterval(Timer::new(
        Duration::from_secs(2),
        TimerMode::Repeating,
    )))
    .add_systems(
        Update,
        (
            tick_interval,
            (spawn_obstacles, move_obstacles, despawn_obstacles),
        )
            .chain()
            .run_if(in_state(GameState::InGame)),
    );
}

fn move_obstacles(
    mut q_obstacles: Query<&mut Position, With<Obstacle>>,
    time: Res<Time>,
) {
    for mut position in q_obstacles.iter_mut() {
        position.x -= OBSTACLE_SPEED * time.delta_secs();
    }
}

/// Despawn when it's out of frame.
fn despawn_obstacles(
    mut commands: Commands,
    q_obstacles: Query<(&Position, Entity), With<Obstacle>>,
) {
    for (position, entity) in q_obstacles.iter() {
        if position.x < -SPAWN_START_X {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_obstacles(
    mut commands: Commands,
    interval: Res<SpawnInterval>,
) {
    const GAP: f32 = 400.0;

    let top_height = rand::random_range(400.0..=1000.0);
    let bottom_height = WINDOW_HEIGHT * 2.0 - top_height - GAP;

    let obstacle_bundle = (
        Sprite {
            color: ZINC_600.into(),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::rectangle(1.0, 1.0),
        Obstacle,
    );

    if interval.just_finished() {
        // spawn
        commands.spawn((
            Transform::from_xyz(
                SPAWN_START_X,
                WINDOW_HALF_HEIGHT,
                -1.0,
            )
            .with_scale(Vec3::new(
                OBSTACLE_WIDTH,
                top_height,
                1.0,
            )),
            obstacle_bundle.clone(),
        ));

        commands.spawn((
            Transform::from_xyz(
                SPAWN_START_X,
                -WINDOW_HALF_HEIGHT,
                -1.0,
            )
            .with_scale(Vec3::new(
                OBSTACLE_WIDTH,
                bottom_height,
                1.0,
            )),
            obstacle_bundle,
        ));
    }
}

fn tick_interval(
    mut interval: ResMut<SpawnInterval>,
    time: Res<Time>,
) {
    interval.tick(time.delta());
}

#[derive(Component, Clone, Copy)]
#[require(StateScoped::<GameState>(GameState::InGame))]
pub struct Obstacle;

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnInterval(Timer);
