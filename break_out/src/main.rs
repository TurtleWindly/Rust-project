use bevy::prelude::*;

use crate::startup::*;

mod startup;
mod component;

fn main() {
    App::new()
        .add_plugin(StartupPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
