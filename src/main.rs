use bevy::{prelude::*,window::WindowMode::*};
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;


mod player;
mod world_gen;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor{
            resizable: false,
            mode: Fullscreen,
            ..default()
        })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(world_gen::WorldGen)
        .add_plugin(player::Player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn();
}