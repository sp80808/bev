use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player::Player, GameState};

const GEM_SIZE: f32 = 15.0;
const GEM_VALUE: u32 = 10;
const INITIAL_XP_TO_NEXT_LEVEL: u32 = 100;

#[derive(Component)]
pub struct ExperienceGem;

#[derive(Resource)]
pub struct PlayerStats {
    pub level: u32,
    pub experience: u32,
    pub xp_to_next_level: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            level: 1,
            experience: 0,
            xp_to_next_level: INITIAL_XP_TO_NEXT_LEVEL,
        }
    }
}

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerStats>().add_systems(
            Update,
            (
                handle_gem_collection,
                level_up_system,
            )
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

pub fn spawn_experience_gem(commands: &mut Commands, position: Vec2) {
    commands
        .spawn(ExperienceGem)
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(GEM_SIZE, GEM_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(GEM_SIZE / 2.0))
        .insert(Sensor);
}

fn handle_gem_collection(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_stats: ResMut<PlayerStats>,
    player_query: Query<Entity, With<Player>>,
    gem_query: Query<Entity, With<ExperienceGem>>,
) {
    let player_entity = match player_query.get_single() {
        Ok(entity) => entity,
        Err(_) => return,
    };

    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            let (gem_entity, other_entity) =
                if gem_query.get(*entity1).is_ok() {
                    (*entity1, *entity2)
                } else if gem_query.get(*entity2).is_ok() {
                    (*entity2, *entity1)
                } else {
                    continue;
                };

            if other_entity == player_entity {
                player_stats.experience += GEM_VALUE;
                commands.entity(gem_entity).despawn();
            }
        }
    }
}

fn level_up_system(
    mut player_stats: ResMut<PlayerStats>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_stats.experience >= player_stats.xp_to_next_level {
        player_stats.level += 1;
        player_stats.experience -= player_stats.xp_to_next_level;
        player_stats.xp_to_next_level = (player_stats.xp_to_next_level as f32 * 1.5).round() as u32;
        next_state.set(GameState::LevelUp);
        println!(
            "LEVEL UP! New Level: {}, XP to next: {}",
            player_stats.level, player_stats.xp_to_next_level
        );
    }
}
