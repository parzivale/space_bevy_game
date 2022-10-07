use leafwing_input_manager::prelude::*;
use bevy::prelude::*;
use bevy_mod_wanderlust::*;

pub struct player{
    #[bundle]
    controller: CharacterControllerBundle,
    #[bundle]
    input_manger: InputManagerBundle,
    #[bundle]
    camera:Camera3dBundle
}
