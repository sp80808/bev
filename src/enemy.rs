use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{components::Health, player::Player};

const ENEMY_SPAWN_TIME: f32 = 0.5;
const ENEMY_SPEED: f32 = 200.0;
const ENEMY_SIZE: f32 = 50.0;

#[derive(Component)]
pub struct Enemy;

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

    let spawn_angle = rng.gen_range(0.0..360.0_f32).to_radians();
    let spawn_dist = (window.width().powi(2) + window.height().powi(2)).sqrt() / 2.0 + ENEMY_SIZE;

    let spawn_pos = player_transform.translation.truncate()
        + Vec2::new(spawn_angle.cos(), spawn_angle.sin()) * spawn_dist;

    commands
        .spawn(Enemy)
        .insert(Health { value: 100.0 })
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(ENEMY_SIZE / 2.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::zero());
}

fn enemy_movement(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Velocity), (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() {
        return;
    }
    let player_transform = player_query.single();

    for (enemy_transform, mut velocity) in enemy_query.iter_mut() {
        let direction =
            (player_transform.translation - enemy_transform.translation).truncate().normalize_or_zero();
        velocity.linvel = direction * ENEMY_SPEED;
    }
}
