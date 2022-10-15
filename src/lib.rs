use bevy::{prelude::*, window::WindowMode::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use states::*;

pub mod player;
pub mod states;
pub mod ui;
pub mod world;

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
            .add_plugin(player::PlayerPlugin);
        //.add_plugin(ui::MenuPlugin)
        // end section setup plugins

        // section Game systems
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(spawn_player)
                .label("Spawn Main Player"),
        );

        app.add_system_set(
            SystemSet::on_exit(AppState::Game)
                .with_system(states::despawn_recursive_from_state)
                .label("Clean Up Game State"),
        );
        // end section Game systems

        // section Paused systems
        app.add_system_set(
            SystemSet::on_enter(AppState::Paused)
                .with_system(player::enable_player_actions)
                .label("Enable Player Actions"),
        );

        app.add_system_set(
            SystemSet::on_exit(AppState::Paused)
                .with_system(player::disable_player_actions)
                .label("Disable Player Actions"),
        );
        // end section Paused systems

        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new());
        }
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(player::bundle::PlayerBundle::default())
        .insert(states::AppStateComponent(AppState::Game))
        .with_children(|parent| {
            parent.spawn_bundle(player::bundle::PlayerCameraBundle::default());
        });
}
