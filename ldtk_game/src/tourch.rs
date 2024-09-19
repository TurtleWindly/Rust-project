use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct TourchPlugin;

const DEFAULT_INTENSITY: f32 = 3.0;

#[derive(Default, Component)]
struct Tourch;

#[derive(Event)]
struct TourchClicked(Entity);

impl From<ListenerInput<Pointer<Down>>> for TourchClicked {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        TourchClicked(event.target)
    }
}

impl Plugin for TourchPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<TourchBundle>("Tourch")
            .add_event::<TourchClicked>()
            .add_systems(Update, (setup, receive_clicked));
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct TourchBundle {
    tourch: Tourch,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn setup(mut commands: Commands, tourches: Query<Entity, Added<Tourch>>) {
    for tourch in &tourches {
        commands.entity(tourch).insert((
            PointLight2d {
                radius: 100.0,
                intensity: DEFAULT_INTENSITY,
                ..default()
            },
            Collider::rectangle(16.0, 16.0),
            RigidBody::Static,
            On::<Pointer<Down>>::send_event::<TourchClicked>(),
        ));
    }
}

fn receive_clicked(
    mut tourch_clicked: EventReader<TourchClicked>,
    mut query: Query<&mut PointLight2d>,
) {
    for event in tourch_clicked.read() {
        if let Ok(mut tourch) = query.get_mut(event.0) {
            if tourch.intensity == 0.0 {
                tourch.intensity = DEFAULT_INTENSITY;
            } else {
                tourch.intensity = 0.0;
            }
        }
    }
}
