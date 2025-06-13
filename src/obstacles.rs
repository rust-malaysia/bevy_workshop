use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::GameState;

pub fn obstacle_plugin(app: &mut App) {
    app.insert_resource(SpawnInterval(Timer::new(
        Duration::from_secs(4),
        TimerMode::Repeating,
    )))
    .add_systems(
        Update,
        (tick_interval, (spawn_obstacles, move_obstacles))
            .chain()
            .run_if(in_state(GameState::InGame)),
    );
}

fn move_obstacles(
    mut q_obstacles: Query<&mut Position, With<Obstacle>>,
) {
    for mut position in q_obstacles.iter_mut() {}
}

/// Despawn when it's out of frame.
fn despawn_obstacles(
    mut commands: Commands,
    q_obstacles: Query<(&Position, Entity), With<Obstacle>>,
) {
}

fn spawn_obstacles(
    mut commands: Commands,
    interval: Res<SpawnInterval>,
) {
    if interval.just_finished() {
        // spawn
        commands.spawn((Sprite::default(), Obstacle));
    }
}

fn tick_interval(
    mut interval: ResMut<SpawnInterval>,
    time: Res<Time>,
) {
    interval.tick(time.delta());
}

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnInterval(Timer);
