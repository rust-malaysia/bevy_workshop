use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        bevy_framepace::FramepacePlugin,
        GamePlugin,
    ));

    app.run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Speed(500.0))
            // Initialize the states.
            .init_state::<GameState>()
            .add_sub_state::<PlayerState>()
            .add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(GameState::MainMenu), simple_ui)
            .add_systems(
                OnEnter(PlayerState::Loading),
                spawn_player,
            )
            .add_systems(
                OnExit(GameState::InGame),
                despawn_player,
            )
            .add_systems(
                Update,
                (
                    move_player
                        // Only run if the player has been spawned in.
                        .run_if(in_state(PlayerState::Loaded)),
                    exit_game
                        // Only run if we are not in main menu.
                        .run_if(in_state(GameState::InGame)),
                ),
            );
    }
}

fn exit_game(
    kb_inputs: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if kb_inputs.just_pressed(KeyCode::Escape) {
        info!("Return to MainMenu.");
        next_state.set(GameState::MainMenu);
    }
}

/// Despawns the player.
fn despawn_player(
    mut commands: Commands,
    q_player: Query<Entity, With<Player>>,
) -> Result {
    // There should be only 1 player in the world.
    let player = q_player.single()?;

    commands.entity(player).despawn();

    Ok(())
}

/// Spawns the player.
fn spawn_player(
    mut commands: Commands,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    const SIZE: f32 = 20.0;

    // Spawn player.
    commands.spawn((
        Player,
        Sprite {
            rect: Some(Rect::default().inflate(SIZE)),
            color: CYAN_500.into(),
            ..default()
        },
    ));

    info!("Spawned player.");
    // Move to loaded state.
    next_state.set(PlayerState::Loaded);
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

fn simple_ui(mut commands: Commands) {
    // Creates a simple UI with a start button at the center.
    commands
        .spawn((
            StateScoped(GameState::MainMenu),
            Node {
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Button,
            BackgroundColor(Color::BLACK),
            BorderRadius::all(Val::Px(10.0)),
            children![Text::new("Start")],
        ))
        // Adds a targeted observer.
        .observe(enter_game);

    /// Observer to see if there's any mouse click on the
    /// targeted entity (node).
    ///
    /// If so, switch to GameState::InGame.
    fn enter_game(
        _: Trigger<Pointer<Click>>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        info!("Enter InGame");
        next_state.set(GameState::InGame);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(
    States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(
    SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[source(GameState = GameState::InGame)]
pub enum PlayerState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct Speed(f32);
