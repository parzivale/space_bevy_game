use bevy::prelude::*;

use game::menu;

fn main(){
    App::new().add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_startup_system(menu::setup_menu);
}

fn setup(mut commands: Commands){
    commands.spawn_bundle(Camera3dBundle{
        ..default()
    });
}