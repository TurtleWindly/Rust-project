use bevy::prelude::*;
use crate::component::*;
use crate::PlayerState;

///event
pub struct DungeonEvent(pub String);

pub fn dungeon_input_handle(
    mut player: Query<(&mut Health, &Damage, &Magic, &mut Location), With<Player>>,
    mut dungeon_event: EventReader<DungeonEvent>,
    mut player_states: ResMut<State<PlayerState>>,
) {
    for event in dungeon_event.iter() {
        match event.0.as_str() {
            "yell" => {
                println!("IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII");
            }
            "tarven" => {
                player_states.set(PlayerState::Dungeon).unwrap();
                for (_health, _damage, _magic, mut location) in player.iter_mut() {
                    location.0 = "Tarven".to_string();
                    println!("Current location: {}", location.0);
                }
            }
            "fight" => {
                player_states.set(PlayerState::Fighting).unwrap();
                println!("Fighting !")
            }
            _ => println!("Dont know what is: {}", event.0.as_str()),
        }
        println!("----------------------------------------");
    }
}
