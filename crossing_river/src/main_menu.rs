use bevy::prelude::*;
use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_systems(Startup, setup_main_menu)
            .add_systems(Update, play_button_system)
            .add_systems(OnExit(GameState::MainMenu), clean_ui);
    }
}

#[derive(Component)]
struct MenuTitle;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct MainUI;

fn setup_main_menu(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands
        .spawn((
            (NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            }),
            MainUI,
            Name::new("Menu Layout"),
        ))
        // Menu Title
        .with_children(|parent| {
            parent.spawn((
                (TextBundle::from_section(
                    "Crossing River",
                    TextStyle {
                        font: assert_server.load("fonts/HackNerdFont.ttf"),
                        font_size: 60.,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(35.),
                    top: Val::Percent(20.),
                    ..default()
                }),),
                MenuTitle,
                Name::new("Title"),
            ));
        })
        // Play Button
        .with_children(|parent| {
            parent.spawn((Name::new("Play Button"), PlayButton, ButtonBundle {
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Px(100.),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb_u8(255, 255, 255).into(),
                ..default()
            })).with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Play", TextStyle {
                        font: assert_server.load("fonts/HackNerdFont.ttf"),
                        font_size: 30.,
                        color: Color::BLACK,
                    },
                    ).with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                            position_type: PositionType::Relative,
                            ..default()
                    }));
                });
        });
}

fn play_button_system(mut commands: Commands, interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>) {    
    for interaction in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(GameState::Game)));
            }
            _ => {}
        }
    }
}

fn clean_ui(mut commands: Commands, query: Query<Entity, With<MainUI>>) {
    for ui in query.iter() {
        commands.entity(ui).despawn();
    }
}
