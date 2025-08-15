use bevy::prelude::*;
use bevy::input::gamepad::{GamepadAxis, GamepadAxisType};
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
    // Mapped to gamepad face buttons or keyboard shortcuts for abilities
    Ability1,
    Ability2,
    Ability3,
}

/// Input configuration resource holds default key and gamepad bindings and
/// can be mutated at runtime to support remapping. We keep it small for now.
#[derive(Resource, Debug, Clone)]
pub struct InputConfig {
    // Deadzone for gamepad axis inputs
    pub gamepad_deadzone: f32,
}

impl Default for InputConfig {
    fn default() -> Self {
        InputConfig {
            gamepad_deadzone: 0.2,
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // Update stage systems: one to read input and set velocity, one to apply it.
        app.add_systems(Update, player_movement_system)
            .add_systems(Update, apply_velocity_system)
            .add_systems(Update, debug_ui_update_system);
    }
}

/// Read input from `ActionState<Action>` and gamepad axes to update the player's `Velocity`.
fn player_movement_system(
    action_states: Query<&ActionState<Action>, With<Player>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    input_cfg: Res<InputConfig>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    // Movement speed in units per second.
    const SPEED: f32 = 200.0;

    // Build a gamepad axis vector if a gamepad is connected (uses first gamepad)
    let mut gp_dir = Vec2::ZERO;
    if let Some(gp) = gamepads.iter().next() {
        // Typical mapping: left stick is GamepadAxis::LeftStickX / LeftStickY
        let x = axes.get(GamepadAxis(*gp, GamepadAxisType::LeftStickX)).unwrap_or(0.0);
        let y = axes.get(GamepadAxis(*gp, GamepadAxisType::LeftStickY)).unwrap_or(0.0);
        // In many gamepad APIs, up is negative Y; invert for consistent +Y=up
        gp_dir = Vec2::new(x, -y);
        if gp_dir.length() < input_cfg.gamepad_deadzone {
            gp_dir = Vec2::ZERO;
        } else if gp_dir.length_squared() > 0.0 {
            gp_dir = gp_dir.normalize();
        }
    }

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

        // Combine keyboard/action input with gamepad input. Gamepad has priority if present.
        if gp_dir.length_squared() > 0.0 {
            dir = gp_dir;
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

/// Simple debug UI: we spawn a TextBundle with a marker and update it each frame to
/// show which actions are active and (if present) the first gamepad axes.
#[derive(Component)]
pub struct DebugUi;

fn debug_ui_update_system(
    action_states: Query<&ActionState<Action>, With<Player>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<&mut Text, With<DebugUi>>,
) {
    let mut lines = Vec::new();

    if let Some(action_state) = action_states.iter().next() {
        lines.push(format!("MoveUp: {}", action_state.pressed(Action::MoveUp)));
        lines.push(format!("MoveDown: {}", action_state.pressed(Action::MoveDown)));
        lines.push(format!("MoveLeft: {}", action_state.pressed(Action::MoveLeft)));
        lines.push(format!("MoveRight: {}", action_state.pressed(Action::MoveRight)));
    lines.push(format!("Ability1: {}", action_state.pressed(Action::Ability1)));
    lines.push(format!("Ability2: {}", action_state.pressed(Action::Ability2)));
    lines.push(format!("Ability3: {}", action_state.pressed(Action::Ability3)));
    }

    if let Some(gp) = gamepads.iter().next() {
        let x = axes.get(GamepadAxis(*gp, GamepadAxisType::LeftStickX)).unwrap_or(0.0);
        let y = axes.get(GamepadAxis(*gp, GamepadAxisType::LeftStickY)).unwrap_or(0.0);
        lines.push(format!("Gamepad LStick: x={:.2}, y={:.2}", x, y));
    }

    let text_value = lines.join("\n");
    for mut text in query.iter_mut() {
        if !text.sections.is_empty() {
            text.sections[0].value = text_value.clone();
        } else {
            // Fallback: ensure there's at least one section
            text.sections.push(TextSection::new(text_value.clone(), TextStyle::default()));
        }
    }
}
