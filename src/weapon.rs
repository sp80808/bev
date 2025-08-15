use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const ORBIT_DISTANCE: f32 = 100.0;
const ORBIT_SPEED: f32 = 5.0;
const WEAPON_SIZE: f32 = 25.0;

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct Projectile;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orbit_weapon);
    }
}

pub fn spawn_orbital_weapon(commands: &mut Commands) -> Entity {
    commands
        .spawn(Weapon)
        .insert(Projectile)
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(WEAPON_SIZE, WEAPON_SIZE)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(WEAPON_SIZE / 2.0))
        .insert(Sensor)
        .id()
}

fn orbit_weapon(
    time: Res<Time>,
    mut weapon_query: Query<(&Parent, &mut Transform), With<Weapon>>,
    player_query: Query<&Transform, (With<crate::player::Player>, Without<Weapon>)>,
) {
    for (parent, mut weapon_transform) in weapon_query.iter_mut() {
        if let Ok(player_transform) = player_query.get(parent.get()) {
            let angle = ORBIT_SPEED * time.elapsed_seconds();
            let new_pos = Vec2::new(angle.cos(), angle.sin()) * ORBIT_DISTANCE;

            weapon_transform.translation = player_transform.translation + Vec3::new(new_pos.x, new_pos.y, 1.0);
        }
    }
}
