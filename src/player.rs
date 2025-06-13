use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{GameState, action::PlayerAction};

pub fn player_plugin(app: &mut App) {
    app.insert_resource(JumpImpulse(400.0))
        .add_systems(OnEnter(GameState::InGame), spawn_player)
        .add_systems(
            Update,
            (player_jump, check_collisions)
                .run_if(in_state(GameState::InGame)),
        );
}

fn check_collisions(
    q_player: Query<Entity, With<Player>>,
    collisions: Collisions,
    mut next_state: ResMut<NextState<GameState>>,
) -> Result {
    let entity = q_player.single()?;
    let mut contacts = collisions.collisions_with(entity);
    // Have collided...
    if contacts.next().is_some() {
        next_state.set(GameState::GameOver);
    }

    Ok(())
}

/// Move player based on [`PlayerAction`].
fn player_jump(
    mut q_player: Query<
        (&mut LinearVelocity, &ActionState<PlayerAction>),
        With<Player>,
    >,
    jump_impulse: Res<JumpImpulse>,
) -> Result {
    // Move the player transform based on speed and direction.
    let (mut linear_velocity, player_action) =
        q_player.single_mut()?;

    if player_action.just_pressed(&PlayerAction::Jump) {
        linear_velocity.0 = Vec2::Y * jump_impulse.0;
        info!("Jump.");
    }

    Ok(())
}

/// Spawns the player.
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn player.
    commands.spawn((
        StateScoped(GameState::InGame),
        Player,
        Sprite {
            // Loads the asset handle.
            image: asset_server.load(
                "Sprites/Enemies/Default/slime_block_rest.png",
            ),
            ..default()
        },
        PlayerAction::new(),
        RigidBody::Dynamic,
        LockedAxes::new().lock_translation_x().lock_rotation(),
        TransformInterpolation,
        Collider::rectangle(40.0, 40.0),
    ));

    info!("Spawned player.");
}

#[derive(Component)]
#[require(CollisionEventsEnabled)]
pub struct Player;

#[derive(Resource)]
pub struct JumpImpulse(f32);
