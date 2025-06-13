use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn action_plugin(app: &mut App) {
    app.add_plugins(
        InputManagerPlugin::<PlayerAction>::default(),
    );
}

#[derive(
    Actionlike, Reflect, PartialEq, Eq, Clone, Copy, Hash, Debug,
)]
pub enum PlayerAction {
    Jump,
}

impl PlayerAction {
    pub fn new() -> InputMap<Self> {
        InputMap::default()
            // For keyboard.
            .with(Self::Jump, KeyCode::Space)
            // For gamepad.
            .with(Self::Jump, GamepadButton::South)
    }
}
