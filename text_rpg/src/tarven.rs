use bevy::prelude::*;
use crate::component::*;
use crate::PlayerState;

/// tarven event
pub struct TarvenEvent(pub String);

pub fn tarven_input_handle(
    mut player: Query<(&mut Health, &Damage, &Magic, &mut Location), With<Player>>,
    mut tarven_event: EventReader<TarvenEvent>,
    mut player_states: ResMut<State<PlayerState>>,
) {
    for event in tarven_event.iter() {
        match event.0.as_str() {
            "heal" => {
                for (mut health, _damage, _magic, _location) in player.iter_mut() {
                    health.0 += 10;
                    println!("Current health: {}", health.0);
                }
            }
            "dungeon" => {
                player_states.set(PlayerState::Dungeon).unwrap();
                for (_health, _damage, _magic, mut location) in player.iter_mut() {
                    location.0 = "Dungeon".to_string();
                    println!("Current location: {}", location.0);
                }
            }
            _ => println!("Dont know what is: {}", event.0.as_str()),
        }
        println!("----------------------------------------");
    }
}
