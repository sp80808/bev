use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{components::Health, player::Player};

const ENEMY_SPAWN_TIME: f32 = 0.5;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct CurrentEnemyStats {
    pub speed: f32,
}

pub enum EnemyType {
    Grunt,
    Tank,
}

pub struct EnemyStats {
    pub health: f32,
    pub speed: f32,
    pub size: f32,
    pub color: Color,
}

impl EnemyType {
    fn get_stats(&self) -> EnemyStats {
        match self {
            EnemyType::Grunt => EnemyStats {
                health: 50.0,
                speed: 250.0,
                size: 40.0,
                color: Color::srgb(0.8, 0.2, 0.2), // Lighter red
            },
            EnemyType::Tank => EnemyStats {
                health: 200.0,
                speed: 150.0,
                size: 75.0,
                color: Color::srgb(0.6, 0.0, 0.0), // Darker red
            },
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            ENEMY_SPAWN_TIME,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (spawn_enemies, enemy_movement));
    }
}

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    windows: Query<&Window>,
    player_query: Query<&Transform, With<Player>>,
) {
    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }

    let window = windows.single();
    let player_transform = player_query.single();
    let mut rng = rand::thread_rng();

    let enemy_type = if rng.gen_bool(0.7) {
        EnemyType::Grunt
    } else {
        EnemyType::Tank
    };
    let stats = enemy_type.get_stats();

    let spawn_angle = rng.gen_range(0.0..360.0_f32).to_radians();
    let spawn_dist = (window.width().powi(2) + window.height().powi(2)).sqrt() / 2.0 + stats.size;

    let spawn_pos = player_transform.translation.truncate()
        + Vec2::new(spawn_angle.cos(), spawn_angle.sin()) * spawn_dist;

    commands
        .spawn(Enemy)
        .insert(Health { value: stats.health })
        .insert(CurrentEnemyStats { speed: stats.speed })
        .insert(SpriteBundle {
            sprite: Sprite {
                color: stats.color,
                custom_size: Some(Vec2::new(stats.size, stats.size)),
                ..default()
            },
            transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(stats.size / 2.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::zero());
}

fn enemy_movement(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Velocity, &CurrentEnemyStats), (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() {
        return;
    }
    let player_transform = player_query.single();

    for (enemy_transform, mut velocity, stats) in enemy_query.iter_mut() {
        let direction =
            (player_transform.translation - enemy_transform.translation).truncate().normalize_or_zero();
        velocity.linvel = direction * stats.speed;
    }
}
