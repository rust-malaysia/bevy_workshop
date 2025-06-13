use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        bevy_framepace::FramepacePlugin,
        MovementPlugin,
    ));

    app.run();
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (PlayerSet::Movement, PlayerSet::Camera).chain(),
        )
        .insert_resource(Speed(500.0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player.in_set(PlayerSet::Movement),
                move_camera.in_set(PlayerSet::Camera),
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    const SIZE: f32 = 20.0;
    const TILE_SIZE: f32 = 10.0;
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

    // Spawn background.
    for x in 0..5 {
        for y in 0..5 {
            commands.spawn((
                Sprite {
                    rect: Some(
                        Rect::default().inflate(TILE_SIZE),
                    ),
                    color: RED_500.into(),
                    ..default()
                },
                Transform::from_xyz(
                    50.0 * x as f32 - 100.0,
                    50.0 * y as f32 - 100.0,
                    -1.0,
                ),
            ));
        }
    }
}

/// Move player based on WASD key inputs.
fn move_player(
    mut q_player: Query<&mut Transform, With<Player>>,
    kb_inputs: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    speed: Res<Speed>,
) -> Result {
    let mut transform = q_player.single_mut()?;
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
        return Ok(());
    };

    // Move the player transform based on speed and direction.
    transform.translation.x +=
        speed.0 * direction.x * time.delta_secs();
    transform.translation.y +=
        speed.0 * direction.y * time.delta_secs();

    Ok(())
}

fn move_camera(
    mut q_camera: Query<
        &mut Transform,
        (With<Camera2d>, Without<Player>),
    >,
    q_player: Query<
        &Transform,
        (With<Player>, Without<Camera2d>),
    >,
) -> Result {
    let player_transform = q_player.single()?;
    let mut camera_transform = q_camera.single_mut()?;

    camera_transform.translation = player_transform.translation;
    Ok(())
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct Speed(f32);

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlayerSet {
    Movement,
    Camera,
}
