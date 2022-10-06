use bevy::{prelude::*, window::WindowMode::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use states::GameState;

pub mod player;
pub mod states;
pub mod ui;
pub mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Unpaused)
            .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            .insert_resource(WindowDescriptor {
                resizable: false,
                mode: Fullscreen,
                ..default()
            })
            .add_plugins(DefaultPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(world::WorldGenPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(ui::MenuPlugin)
            .add_startup_system(setup);

        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new());
        }
    }
}

fn setup(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}
