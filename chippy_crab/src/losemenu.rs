use crate::{GameState, UiFont};
use bevy::prelude::*;

pub struct LoseMenu;

impl Plugin for LoseMenu {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::ScoreMenu)
                .with_system(spawn_tilte)
                .with_system(spawn_home_button),
        )
        .add_system_set(SystemSet::on_update(GameState::ScoreMenu).with_system(check_home_button))
        .add_system_set(SystemSet::on_exit(GameState::ScoreMenu).with_system(clean_ui));
    }
}

#[derive(Component)]
struct UiLoseMenu;

#[derive(Component)]
struct HomeButton;

fn spawn_tilte(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(150.),
                    top: Val::Px(150.),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "You Lose",
                TextStyle {
                    font: font.0.clone(),
                    font_size: 70.,
                    color: Color::WHITE,
                    ..default()
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(UiLoseMenu);
}

fn spawn_home_button(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::YELLOW.into(),
            ..default()
        })
        .insert(HomeButton)
        .insert(UiLoseMenu)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Home",
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                        Default::default(),
                    ),
                    ..default()
                })
                .insert(UiLoseMenu);
        });
}

fn check_home_button(
    query: Query<&Interaction, With<HomeButton>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            game_state
                .set(GameState::MainMenu)
                .expect("Can't set game state via click home button");
        }
    }
}

fn clean_ui(mut commands: Commands, query: Query<Entity, With<UiLoseMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
