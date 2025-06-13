use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode::FixedVertical;

mod action;
mod obstacles;
mod player;
mod ui;

const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HALF_WIDTH: f32 = WINDOW_WIDTH / 2.0;
const WINDOW_HALF_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;

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
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: FixedVertical {
                viewport_height: WINDOW_HEIGHT,
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
