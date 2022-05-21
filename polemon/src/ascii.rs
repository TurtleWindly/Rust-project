use bevy::prelude::*;
use crate::TITLE_SIZE;

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}

pub struct AsciiSheet(pub Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assests: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
    let image = assests.load("Ascii.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        // texture
        image,
        // title_size
        Vec2::splat(9.),
        // columns then rows
        16,
        16,
        // padding
        Vec2::splat(2.),
        );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TITLE_SIZE));

    commands.spawn_bundle(SpriteSheetBundle {
        sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation,
            ..default()
        },
        ..default()
    })
    .id()
}
