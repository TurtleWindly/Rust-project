use crate::{GameState, UiFont};
use bevy::prelude::*;

pub struct Scores(pub f32);

pub struct ScoresPlugin;

impl Plugin for ScoresPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(spawn_scores))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(update_scores))
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(clean_ui));
    }
}

#[derive(Component)]
struct UiScores;

fn spawn_scores(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(280.),
                    top: Val::Px(50.),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "0",
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
        .insert(UiScores);
}

fn update_scores(mut ui_scores: Query<&mut Text, With<UiScores>>, current_scores: Res<Scores>) {
    for mut text in ui_scores.iter_mut() {
        text.sections[0].value = format!("{}", current_scores.0);
    }
}

fn clean_ui(mut commands: Commands, query: Query<Entity, With<UiScores>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
