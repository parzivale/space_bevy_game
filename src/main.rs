use bevy::{prelude::*, window::WindowMode::*};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Inspectable)]
pub enum GameState {
    Paused,
    Unpaused,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerBody;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerCamera;

mod actions;
mod player;
mod world_gen;

fn main() {
    App::new()
        .add_loopless_state(GameState::Unpaused)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            resizable: false,
            mode: Fullscreen,
            ..default()
        })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(actions::ActionsPlugin)
        .add_plugin(world_gen::WorldGen)
        .add_plugin(player::Player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn();
}
