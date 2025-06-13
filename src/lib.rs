use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode::FixedVertical;

mod action;
mod obstacles;
mod player;
mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            action::action_plugin,
            ui::ui_plugin,
            player::player_plugin,
            obstacles::obstacle_plugin,
        ));

        app.insert_resource(Gravity(-Vec2::Y * 1000.0))
            // Initialize the states.
            .init_state::<GameState>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                stop_game
                    // Only run if we are not in main menu.
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn stop_game(
    kb_inputs: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if kb_inputs.just_pressed(KeyCode::Escape) {
        info!("Return to MainMenu.");
        next_state.set(GameState::MainMenu);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: FixedVertical {
                viewport_height: 720.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

#[derive(
    States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
