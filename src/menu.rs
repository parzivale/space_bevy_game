use bevy::prelude::*;
//use leafwing_input_manager::prelude::*;
use crate::GameState;
use crate::common;


#[derive(Component)]
struct PauseMenu;


pub struct MenuPlugin;

impl Plugin for MenuPlugin{
    fn build(&self, app:&mut App){
        app.add_startup_system(setup_menu)
        .add_system_set(
            SystemSet::on_update(GameState::Paused)
            .with_system(create_menu)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Unpaused)
            .with_system(common::despawn_entity::<PauseMenu>)
        );
    }
}

fn setup_menu(){

}

fn create_menu(mut commands: Commands){
    commands.spawn_bundle(NodeBundle{
        ..default()
    });
}