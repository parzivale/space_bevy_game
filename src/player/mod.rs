use bevy::{prelude::*};
use bevy_mod_wanderlust::*;


mod player_controller;
pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App){
        app.add_plugin(WanderlustPlugin)
        .insert_resource(player_controller::Sensitivity(0.15))
        .add_startup_system(player_controller::setup_player_controller)
        .add_system_to_stage(CoreStage::PreUpdate, player_controller::movement_input)
        .add_system(player_controller::mouse_look);
    }
}
