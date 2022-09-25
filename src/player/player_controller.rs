use crate::{PlayerBody, PlayerCamera};
use bevy::prelude::*;
use bevy_mod_wanderlust::*;

pub struct PlayerController;
impl Plugin for PlayerController {
    fn build(&self, app: &mut App) {
        app.add_plugin(WanderlustPlugin)
            .add_startup_system(setup_player_controller);
    }
}

pub fn setup_player_controller(mut commands: Commands) {
    commands
        .spawn_bundle(CharacterControllerBundle { ..default() })
        .insert(PlayerBody)
        .with_children(|commands| {
            commands
                .spawn_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0., 0.5, 0.),
                    ..default()
                })
                .insert(PlayerCamera);
        });
}
