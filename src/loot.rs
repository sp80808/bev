use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{components::Health, player::Player, GameState};

const HEALTH_PACK_SIZE: f32 = 20.0;
const HEALTH_PACK_VALUE: f32 = 25.0;

#[derive(Component)]
pub struct LootDrop {
    pub loot_type: LootType,
}

pub enum LootType {
    HealthPack,
}

pub struct LootPlugin;

impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_loot_collection.run_if(in_state(GameState::Gameplay)),
        );
    }
}

pub fn spawn_loot_drop(commands: &mut Commands, position: Vec2) {
    // For now, only health packs can drop
    let loot_type = LootType::HealthPack;

    commands
        .spawn(LootDrop { loot_type })
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.8, 0.2), // Green
                custom_size: Some(Vec2::new(HEALTH_PACK_SIZE, HEALTH_PACK_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(HEALTH_PACK_SIZE / 2.0))
        .insert(Sensor);
}

fn handle_loot_collection(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Health), With<Player>>,
    loot_query: Query<(Entity, &LootDrop)>,
) {
    let (player_entity, mut player_health) = match player_query.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            let (loot_entity, loot_drop) = if let Ok(l) = loot_query.get(*entity1) {
                (l.0, l.1)
            } else if let Ok(l) = loot_query.get(*entity2) {
                (l.0, l.1)
            } else {
                continue;
            };

            let other_entity = if loot_entity == *entity1 {
                *entity2
            } else {
                *entity1
            };

            if other_entity == player_entity {
                match loot_drop.loot_type {
                    LootType::HealthPack => {
                        player_health.value += HEALTH_PACK_VALUE;
                        // Optional: Clamp health to a max value
                        // player_health.value = player_health.value.min(MAX_PLAYER_HEALTH);
                        println!("Collected a health pack! Current health: {}", player_health.value);
                    }
                }
                commands.entity(loot_entity).despawn();
            }
        }
    }
}
