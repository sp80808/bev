use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;

mod combat;
mod components;
mod enemy;
mod experience;
mod movement;
mod player;
mod weapon;

use combat::CombatPlugin;
use enemy::EnemyPlugin;
use experience::ExperiencePlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use weapon::WeaponPlugin;

// Central game states for the project.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Gameplay,
    LevelUp,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
    .add_state::<GameState>()
    // Global input configuration resource used for constructing input maps and runtime remapping
    .insert_resource(movement::InputConfig::default())
        // Physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // Input manager plugin for remappable actions
        .add_plugins(InputManagerPlugin::<movement::Action>::default())
        // Game feature plugins
        .add_plugins((
            PlayerPlugin,
            MovementPlugin,
            EnemyPlugin,
            WeaponPlugin,
            CombatPlugin,
            ExperiencePlugin,
        ))
        .run();
}
