use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{ 
    components::Health,
    enemy::Enemy,
    experience::{spawn_experience_gem, PlayerStats},
    loot::spawn_loot_drop,
    weapon::Projectile,
};

const PROJECTILE_DAMAGE: f32 = 10.0;
const LOOT_DROP_CHANCE: f32 = 0.3; // 30%

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_collisions,
                check_death.after(handle_collisions),
            ),
        );
    }
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    player_stats: Res<PlayerStats>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            let (_projectile_entity, enemy_entity) =
                if projectile_query.get(*entity1).is_ok() && enemy_query.get(*entity2).is_ok() {
                    (*entity1, *entity2)
                } else if projectile_query.get(*entity2).is_ok() && enemy_query.get(*entity1).is_ok()
                {
                    (*entity2, *entity1)
                } else {
                    continue;
                };

            if let Ok(mut health) = enemy_query.get_mut(enemy_entity) {
                health.value -= PROJECTILE_DAMAGE * player_stats.damage_multiplier;
            }
        }
    }
}

fn check_death(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Health), With<Enemy>>,
) {
    let mut rng = rand::thread_rng();
    for (entity, transform, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn();
            let position = transform.translation.truncate();

            if rng.gen_range(0.0..1.0) < LOOT_DROP_CHANCE {
                spawn_loot_drop(&mut commands, position);
            } else {
                spawn_experience_gem(&mut commands, position);
            }
        }
    }
}
