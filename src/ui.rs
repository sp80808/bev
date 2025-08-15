use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{experience::PlayerStats, GameState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Upgrade {
    AddOrbital,
    IncreaseDamage,
    IncreaseSpeed,
}

impl Upgrade {
    fn all() -> Vec<Self> {
        vec![
            Upgrade::AddOrbital,
            Upgrade::IncreaseDamage,
            Upgrade::IncreaseSpeed,
        ]
    }

    fn get_description(&self) -> &str {
        match self {
            Upgrade::AddOrbital => "Add another orbiting weapon",
            Upgrade::IncreaseDamage => "Increase weapon damage by 20%",
            Upgrade::IncreaseSpeed => "Increase movement speed by 10%",
        }
    }
}

#[derive(Component)]
struct LevelUpScreen;

#[derive(Component)]
struct UpgradeButton(Upgrade);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LevelUp), setup_level_up_screen)
            .add_systems(OnExit(GameState::LevelUp), teardown_level_up_screen)
            .add_systems(
                Update,
                upgrade_button_interaction.run_if(in_state(GameState::LevelUp)),
            );
    }
}

fn setup_level_up_screen(mut commands: Commands) {
    let mut rng = thread_rng();
    let all_upgrades = Upgrade::all();
    let chosen_upgrades = all_upgrades
        .choose_multiple(&mut rng, 3)
        .cloned()
        .collect::<Vec<_>>();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            LevelUpScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Level Up!",
                TextStyle {
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            for upgrade in chosen_upgrades {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(400.0),
                                height: Val::Px(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        },
                        UpgradeButton(upgrade),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            upgrade.get_description(),
                            TextStyle {
                                font_size: 30.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
            }
        });
}

fn teardown_level_up_screen(
    mut commands: Commands,
    query: Query<Entity, With<LevelUpScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn upgrade_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &UpgradeButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_stats: ResMut<PlayerStats>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands, // To spawn new weapon
    player_query: Query<Entity, With<crate::player::Player>>,
) {
    for (interaction, button, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.75, 0.35).into();
                apply_upgrade(
                    button.0,
                    &mut player_stats,
                    &mut commands,
                    &player_query,
                );
                next_state.set(GameState::Gameplay);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn apply_upgrade(
    upgrade: Upgrade,
    player_stats: &mut ResMut<PlayerStats>,
    commands: &mut Commands,
    player_query: &Query<Entity, With<crate::player::Player>>,
) {
    match upgrade {
        Upgrade::AddOrbital => {
            if let Ok(player_entity) = player_query.get_single() {
                let new_weapon = crate::weapon::spawn_orbital_weapon(commands);
                commands.entity(player_entity).add_child(new_weapon);
                player_stats.orbital_count += 1;
            }
        }
        Upgrade::IncreaseDamage => {
            player_stats.damage_multiplier += 0.2;
        }
        Upgrade::IncreaseSpeed => {
            player_stats.speed_multiplier += 0.1;
        }
    }
    println!("Applied upgrade: {:?}", upgrade);
}
