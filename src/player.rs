use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::{Actionlike, InputMap};
use leafwing_input_manager::prelude::controller::GamepadButtonType;

use crate::{
    components::Health,
    movement::{Action, Velocity, DebugUi, InputConfig},
    weapon::spawn_orbital_weapon,
};

const PLAYER_SIZE: f32 = 32.0;

/// Marker component for the player entity.
#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // The state transition logic is now in main.rs with the state machine
        app.add_systems(Startup, setup_player);
    }
}

/// Spawn a simple player as a colored square sprite and give it a `Velocity` component.
/// Also attach an `InputManagerBundle` so the player entity has remappable input actions.
fn setup_player(mut commands: Commands) {
    // Spawn a simple 2D camera so the scene is visible.
    commands.spawn(Camera2dBundle::default());

    // Build default input map: WASD and arrow keys mapped to movement actions.
    let mut input_map = InputMap::default();
    input_map.insert(KeyCode::KeyW, Action::MoveUp);
    input_map.insert(KeyCode::ArrowUp, Action::MoveUp);
    input_map.insert(KeyCode::KeyS, Action::MoveDown);
    input_map.insert(KeyCode::ArrowDown, Action::MoveDown);
    input_map.insert(KeyCode::KeyA, Action::MoveLeft);
    input_map.insert(KeyCode::ArrowLeft, Action::MoveLeft);
    input_map.insert(KeyCode::KeyD, Action::MoveRight);
    input_map.insert(KeyCode::ArrowRight, Action::MoveRight);
    // Keyboard shortcuts for abilities
    input_map.insert(KeyCode::KeyJ, Action::Ability1);
    input_map.insert(KeyCode::KeyK, Action::Ability2);
    input_map.insert(KeyCode::KeyL, Action::Ability3);
    // Map common gamepad face buttons to abilities (South=A, East=B, North=Y on many controllers)
    input_map.insert(GamepadButtonType::South, Action::Ability1);
    input_map.insert(GamepadButtonType::East, Action::Ability2);
    input_map.insert(GamepadButtonType::North, Action::Ability3);

    let weapon_entity = spawn_orbital_weapon(&mut commands);

    // Spawn the player as a colored square sprite with an initial zero velocity.
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.3, 0.7, 0.9),
                    custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            Player,
            Health { value: 100.0 },
            // Start stationary
            Velocity(Vec2::ZERO),
            // Attach input manager bundle with our action map so the player entity can receive action state
            leafwing_input_manager::InputManagerBundle {
                input_map,
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::capsule_y(PLAYER_SIZE / 4.0, PLAYER_SIZE / 4.0),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .add_child(weapon_entity);

    // Spawn a simple debug UI text in the top-left that will be updated each frame.
    let font = AssetServer::load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn(TextBundle::from_sections([
            TextSection::new("", TextStyle { font: font.clone(), font_size: 16.0, color: Color::WHITE }),
        ]))
        .insert(Transform::from_translation(Vec3::new(-380.0, 220.0, 0.0)))
        .insert(GlobalTransform::default())
        .insert(DebugUi);
}
