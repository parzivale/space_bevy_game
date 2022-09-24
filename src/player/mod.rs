use bevy::{prelude::*};



mod player_controller;
pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App){
        app.add_plugin(player_controller::PlayerController);
    }
}
