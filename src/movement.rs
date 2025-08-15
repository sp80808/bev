use bevy::prelude::*;
use leafwing_input_manager::prelude::{Actionlike, ActionState};

use crate::player::Player;

/// Simple velocity component represented as units per second in X/Y.
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity(pub Vec2);

/// Actions that the player can perform via input. These are remappable via
/// `leafwing-input-manager` and used by the movement system.
#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // Update stage systems: one to read input and set velocity, one to apply it.
        app.add_systems(Update, player_movement_system)
            .add_systems(Update, apply_velocity_system);
    }
}

/// Read input from `ActionState<Action>` and update the player's `Velocity`.
fn player_movement_system(
    action_states: Query<&ActionState<Action>, With<Player>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    // Movement speed in units per second.
    const SPEED: f32 = 200.0;

    for action_state in action_states.iter() {
        let mut dir = Vec2::ZERO;
        if action_state.pressed(Action::MoveUp) {
            dir.y += 1.0;
        }
        if action_state.pressed(Action::MoveDown) {
            dir.y -= 1.0;
        }
        if action_state.pressed(Action::MoveLeft) {
            dir.x -= 1.0;
        }
        if action_state.pressed(Action::MoveRight) {
            dir.x += 1.0;
        }

        // Normalize to avoid faster diagonal movement.
        if dir.length_squared() > 0.0 {
            dir = dir.normalize();
        }

        for mut vel in query.iter_mut() {
            vel.0 = dir * SPEED;
        }
    }
}

/// Apply velocity to all entities that have a `Velocity` and a `Transform`.
fn apply_velocity_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let delta = time.delta_seconds();
    for (vel, mut transform) in query.iter_mut() {
        transform.translation.x += vel.0.x * delta;
        transform.translation.y += vel.0.y * delta;
    }
}
