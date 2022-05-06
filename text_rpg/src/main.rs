use bevy::prelude::*;
use std::io;
use colored::*;

use crate::startup::*;
use crate::component::*;
use crate::tarven::*;
use crate::dungeon::*;
use crate::fighting::*;

mod startup;
mod component;
mod tarven;
mod dungeon;
mod fighting;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayerState {
    Tarven,
    Dungeon,
    Fighting,
}

fn main() {
    App::new()
        .insert_resource(Enemy {health: 10, damage: 2, magic: 0})
        .add_plugin(StartupPlugin)
        .add_state(PlayerState::Tarven)
        .add_event::<TarvenEvent>()
        .add_event::<DungeonEvent>()
        .add_event::<FightingEvent>()
        .add_event::<ResetEnemyEvent>()
        .add_system(global_input_handle.label("global input"))
        .add_system(tarven_input_handle.after("global input"))
        .add_system(dungeon_input_handle.after("global input"))
        .add_system(fighting_input_handle.after("global input"))
        .add_system(reset_enemy)
        .run();
}

fn global_input_handle(
    player: Query<(&Health, &Damage, &Magic, &Location), With<Player>>,
    player_states: Res<State<PlayerState>>,
    mut tarven_event: EventWriter<TarvenEvent>,
    mut dungeon_event: EventWriter<DungeonEvent>,
    mut fighting_event: EventWriter<FightingEvent>,
) {
    let mut input = String::new();
    println!("\nChoose your action: ");
    io::stdin().read_line(&mut input).expect("Can't read input");
    println!("----------------------------------------");
    match input.trim() {
        "status" | "s" => {
            for (health, damage, magic, _) in player.iter() {
                println!("{}: {}\n{}: {}\n{}: {}", "health".green(), health.0, "damage".red(), damage.0, "magic".magenta(), magic.0);
            }
            println!("----------------------------------------");
        }
        "location" | "l" => {
            for (_health, _damage, _magic, location) in player.iter() {
                println!("Current location: {}", location.0);
            }
            println!("----------------------------------------");
        }
        _ => {
            match player_states.current() {
                PlayerState::Tarven => {
                    tarven_event.send(TarvenEvent(input.trim().to_string()));
                }
                PlayerState::Dungeon => {
                    dungeon_event.send(DungeonEvent(input.trim().to_string()));
                }
                PlayerState::Fighting => {
                    fighting_event.send(FightingEvent(input.trim().to_string()));
                }
            }
        }
    }
    input.clear();
}
