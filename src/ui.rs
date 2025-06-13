use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

use crate::GameState;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), main_menu)
        .add_systems(OnEnter(GameState::GameOver), game_over);
}

fn main_menu(mut commands: Commands) {
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
            BackgroundColor(EMERALD_800.into()),
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
        info!("Enter InGame.");
        next_state.set(GameState::InGame);
    }
}

fn game_over(mut commands: Commands) {
    // Creates a simple UI with a start button at the center.
    commands
        .spawn((
            StateScoped(GameState::GameOver),
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
            BackgroundColor(ORANGE_500.into()),
            BorderRadius::all(Val::Px(10.0)),
            children![Text::new("Continue")],
        ))
        // Adds a targeted observer.
        .observe(stop_game);

    /// Observer to see if there's any mouse click on the
    /// targeted entity (node).
    ///
    /// If so, switch to GameState::MainMenu.
    fn stop_game(
        _: Trigger<Pointer<Click>>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        info!("Enter MainMenu.");
        next_state.set(GameState::MainMenu);
    }
}
