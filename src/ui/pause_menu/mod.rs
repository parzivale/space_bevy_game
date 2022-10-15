use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

mod resources;
//mod spawn;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<resources::PauseMenuActions>::default());
        //.add_startup_system(setup_menu)
        //.add_system(toggle_menu)
        //.add_system(exit_game);
    }
}
