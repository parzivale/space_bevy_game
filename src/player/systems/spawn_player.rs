use bevy::prelude::*;

use crate::player::bundle::{PlayerBundle, PlayerCameraBundle};

pub fn spawn_player_entity(mut commands: Commands) {
    commands
        .spawn_bundle(PlayerBundle::default())
        .with_children(|parent| {
            parent.spawn_bundle(PlayerCameraBundle::default());
        });
}
