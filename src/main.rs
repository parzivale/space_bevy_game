use bevy::{prelude::*, window::WindowMode::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

mod menu;
mod player;
mod world_gen;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum GameState {
    Unpaused,
    Paused,
}

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
        .add_plugin(world_gen::WorldGenPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}
