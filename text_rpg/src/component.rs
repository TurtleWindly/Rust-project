use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Magic(pub i32);

#[derive(Component)]
pub struct Location(pub String);

#[derive(Component)]
pub struct Area;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub damage: Damage,
    pub magic: Magic,
    pub location: Location,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player,
            health: Health(10),
            damage: Damage(2),
            magic: Magic(1),
            location: Location("".to_string()),
        }
    }
}

#[derive(Bundle)]
pub struct StructureBundle {
    pub area: Area,
    pub location: Location,
}

impl Default for StructureBundle {
    fn default() -> Self {
        StructureBundle {
            area: Area,
            location: Location("".to_string()),
        }
    }
}
