use bevy::{prelude::*, window::WindowMode::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use states::*;

pub mod player;
pub mod states;
pub mod ui;
pub mod world;
pub mod spawn;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // section setup states
        app.add_state(AppState::Game);
        // end section setup states

        // section setup resources
        app.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            .insert_resource(WindowDescriptor {
                resizable: false,
                mode: Fullscreen,
                cursor_locked: true,
                cursor_visible: false,
                ..default()
            });
        // end section setup resources

        // section setup plugins
        app.add_plugins(DefaultPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(world::WorldGenPlugin)
            .add_plugin(player::PlayerPlugin::<states::AppState>::from_state(AppState::Game));
            //.add_plugin(ui::MenuPlugin)
        // end section setup plugins

        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new());
        }
    }
}