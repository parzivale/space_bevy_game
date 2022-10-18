use crate::{
    player::{bundle::{PlayerBundle, PlayerCameraBundle}},
    states::{AppStateComponent, AppState,}
};
use bevy::{prelude::*};


//spawns player entity with camera entity as child and inserts the appstate component with value game
pub fn spawn_player_entity(mut commands: Commands){
    commands
    .spawn_bundle(PlayerBundle::default())
    .insert(AppStateComponent(AppState::Game))
    .with_children(|parent|{
        parent.spawn_bundle(PlayerCameraBundle::default());
    });
}