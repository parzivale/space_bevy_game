use bevy::{prelude::*, window::WindowMode::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Paused,
    Unpaused,
}

mod player;
mod world_gen;

fn main() {
    App::new()
        .add_state(GameState::Unpaused)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            resizable: false,
            mode: Fullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(world_gen::WorldGen)
        .add_plugin(player::Player)
        .run();
}