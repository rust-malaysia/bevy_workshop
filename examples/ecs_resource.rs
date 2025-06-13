use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        bevy_framepace::FramepacePlugin,
    ))
    .insert_resource(Speed(500.0))
    .add_systems(Startup, setup)
    .add_systems(Update, move_player);

    app.run();
}

/// Spawn the camera and player.
fn setup(mut commands: Commands) {
    const SIZE: f32 = 20.0;
    commands.spawn(Camera2d);

    // Spawn player.
    commands.spawn((
        Player,
        Sprite {
            rect: Some(Rect::default().inflate(SIZE)),
            color: CYAN_500.into(),
            ..default()
        },
    ));
}

/// Move player based on WASD key inputs.
fn move_player(
    mut q_speeds: Query<&mut Transform, With<Player>>,
    kb_inputs: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    speed: Res<Speed>,
) {
    let mut direction = Vec2::ZERO;

    let key_direction = [
        (KeyCode::KeyW, Vec2::Y),
        (KeyCode::KeyS, -Vec2::Y),
        (KeyCode::KeyA, -Vec2::X),
        (KeyCode::KeyD, Vec2::X),
    ];

    // Determine direction based on key inputs.
    for (key, dir) in key_direction {
        if kb_inputs.pressed(key) {
            direction += dir;
        }
    }

    let Some(direction) = direction.try_normalize() else {
        return;
    };

    // Move the player transform based on speed and direction.
    for mut transform in q_speeds.iter_mut() {
        // Delta time is used to perform consistent movement
        // across varying refresh rates.
        transform.translation.x +=
            speed.0 * direction.x * time.delta_secs();
        transform.translation.y +=
            speed.0 * direction.y * time.delta_secs();
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct Speed(f32);
