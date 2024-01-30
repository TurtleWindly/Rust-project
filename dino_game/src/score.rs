use bevy::prelude::*;

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ScoreMenu), restart_button)
            .add_systems(
                Update,
                (
                    check_restart_button.run_if(in_state(GameState::ScoreMenu)),
                ),
            )
            .add_systems(OnExit(GameState::ScoreMenu), despawn_menu);
    }
}

#[derive(Component)]
struct NodeScoreMenu;

#[derive(Component)]
struct RestartButton;

fn restart_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    let restart_button_img = asset_server.load("restart_button.png");
    let game_over_img = asset_server.load("GameOver.png");

    commands
        .spawn((
            NodeScoreMenu,
            Name::new("Restart UI parent"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Game Over Title"),
                ImageBundle {
                    image: UiImage {
                        texture: game_over_img,
                        ..default()
                    },
                    ..default()
                },
            ));
            parent.spawn((
                RestartButton,
                Name::new("Restart Button"),
                ButtonBundle {
                    image: UiImage {
                        texture: restart_button_img,
                        ..default()
                    },
                    style: Style {
                        margin: UiRect::all(Val::Px(50.)),
                        width: Val::Px(72.),
                        height: Val::Px(64.),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

fn check_restart_button(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
) {
    for interaction in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(GameState::Game)));
            }
            _ => {}
        }
    }
}

fn despawn_menu(mut commands: Commands, ui_query: Query<Entity, With<NodeScoreMenu>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
