use bevy::prelude::*;

use crate::GameState;
use crate::UiFont;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_camera)
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .with_system(spawn_tilte)
                    .with_system(spawn_play_button),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(check_play_button),
            )
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(clean_ui));
    }
}

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct UiMainMenu;

fn ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_tilte(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(100.),
                    top: Val::Px(50.),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "Chippy Crab",
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
        .insert(UiMainMenu);
}

fn spawn_play_button(mut commands: Commands, font: Res<UiFont>) {
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
        .insert(PlayButton)
        .insert(UiMainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..default()
            }).insert(UiMainMenu);
        });
}

fn check_play_button(
    query: Query<&Interaction, With<PlayButton>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            game_state
                .set(GameState::InGame)
                .expect("Can't set game state via click play button");
        }
    }
}

fn clean_ui(mut commands: Commands, query: Query<Entity, With<UiMainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
