use bevy::prelude::*;

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ScoreMenu), restart_button)
            .add_systems(
                Update,
                check_restart_button.run_if(in_state(GameState::ScoreMenu)),
            )
            .add_systems(OnExit(GameState::ScoreMenu), despawn_menu);
    }
}

#[derive(Component)]
struct NodeScoreMenu;

#[derive(Component)]
struct RestartButton;

fn restart_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("restart_button.png");

    commands
        .spawn((
            NodeScoreMenu,
            Name::new("Score UI parent"),
            NodeBundle {
                style: Style {
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
                RestartButton,
                Name::new("Restart Button"),
                ButtonBundle {
                    image: UiImage {
                        texture,
                        ..default()
                    },
                    style: Style {
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
