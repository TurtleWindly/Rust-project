use bevy::prelude::*;
use crate::component::*;
use crate::PlayerState;
use colored::*;

/// An Enemy resource
pub struct Enemy {
    pub health: i32,
    pub damage: i32,
    pub magic: i32,
}

/// event
pub struct ResetEnemyEvent;

pub fn reset_enemy(
    mut enemy: ResMut<Enemy>,
    mut reset_event: EventReader<ResetEnemyEvent>,
    mut players: Query<(&mut Damage, &mut Magic), With<Player>>,
) {
    for _event in reset_event.iter() {
        enemy.health = 10;

        for (mut damage, mut magic) in players.iter_mut() {
            damage.0 += 1;
            magic.0 += 2;
        }
        println!("{}","Corract you have defeated an enemy".red().bold());
    }
}

///event
pub struct FightingEvent(pub String);

pub fn fighting_input_handle(
    mut players: Query<(&mut Health, &Damage, &Magic), With<Player>>,
    mut fighting_event: EventReader<FightingEvent>,
    mut player_states: ResMut<State<PlayerState>>,
    mut reset_enemy_event: EventWriter<ResetEnemyEvent>,
    mut enemy: ResMut<Enemy>,
) {
    for event in fighting_event.iter() {
        for (mut health, damage, magic) in players.iter_mut() {
            match event.0.as_str() {
                "yell" => {
                    println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
                }
                "run" => {
                    player_states.set(PlayerState::Dungeon).unwrap();
                    println!("RUN !");
                }
                "enemy" => {
                    println!("enemy status\n{}: {}\n{}: {}\n{}: {}", "health".green(), enemy.health, "damage".red(), enemy.damage, "magic".magenta(), enemy.magic);
                }
                "damage" => {
                    enemy.health -= damage.0;
                    if enemy.health <= 0 {
                        reset_enemy_event.send(ResetEnemyEvent);
                    }
                    health.0 -= enemy.damage;
                    println!("You attacked");
                }
                "magic" => {
                    enemy.health -= magic.0;
                    if enemy.health <= 0 {
                        reset_enemy_event.send(ResetEnemyEvent);
                    }
                    health.0 -= enemy.damage;
                    println!("You attacked with magic");
                }
                _ => println!("Dont know what is: {}", event.0),
            }
            println!("----------------------------------------");
        }
    }
}
