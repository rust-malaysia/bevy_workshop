#![allow(dead_code)] // Prevent warnings to unused systems.

use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_systems(Startup, setup);

    // app.add_systems(Update, move_all);
    // app.add_systems(Update, move_player);
    // app.add_systems(Update, move_non_player);

    app.run();
}

fn setup(mut commands: Commands) {
    const SIZE: f32 = 20.0;
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Sprite {
            rect: Some(Rect::default().inflate(SIZE)),
            color: CYAN_500.into(),
            ..default()
        },
        Transform::from_xyz(50.0, 0.0, 0.0),
    ));

    commands.spawn((
        Speed { x: -1.0, y: -1.0 },
        Sprite {
            rect: Some(Rect::default().inflate(SIZE)),
            color: RED_500.into(),
            ..default()
        },
    ));
}

fn move_all(mut q_speeds: Query<(&Speed, &mut Transform)>) {
    for (speed, mut transform) in q_speeds.iter_mut() {
        transform.translation.x += speed.x;
        transform.translation.y += speed.y;
    }
}

fn move_player(
    mut q_speeds: Query<(&Speed, &mut Transform), With<Player>>,
) {
    for (speed, mut transform) in q_speeds.iter_mut() {
        transform.translation.x += speed.x;
        transform.translation.y += speed.y;
    }
}

fn move_non_player(
    mut q_speeds: Query<
        (&Speed, &mut Transform),
        Without<Player>,
    >,
) {
    for (speed, mut transform) in q_speeds.iter_mut() {
        transform.translation.x += speed.x;
        transform.translation.y += speed.y;
    }
}

#[derive(Component)]
#[require(Speed {x: 1.0, y: 1.0})]
pub struct Player;

#[derive(Component)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}
