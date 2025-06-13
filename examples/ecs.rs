#![allow(dead_code)] // Prevent warnings to unused systems.

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(PreStartup, setup);

    // app.add_systems(Startup, print_entities);
    // app.add_systems(Startup, print_players);
    // app.add_systems(Startup, print_non_players);

    app.run();
}

fn print_entities(q_entities: Query<(&Speed, &Position)>) {
    info!("All entities:");
    for (speed, position) in q_entities.iter() {
        info!("{speed:?}, {position:?}");
    }
}

fn print_players(
    q_players: Query<(&Speed, &Position), With<Player>>,
) {
    info!("Players:");
    for (speed, position) in q_players.iter() {
        info!("{speed:?}, {position:?}");
    }
}

fn print_non_players(
    q_players: Query<(&Speed, &Position), Without<Player>>,
) {
    info!("Non players:");
    for (speed, position) in q_players.iter() {
        info!("{speed:?}, {position:?}");
    }
}

fn setup(mut commands: Commands) {
    // Spawn an entity with Speed & Position
    // but without the Player component.
    commands.spawn((
        Speed { x: 1.0, y: 2.0 },
        Position { x: 10.0, y: 20.0 },
    ));

    // Spawn an entity with Speed & Position
    // with the Player component.
    commands.spawn((
        Speed { x: 3.0, y: 4.0 },
        Position { x: 30.0, y: 40.0 },
        Player,
    ));
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
