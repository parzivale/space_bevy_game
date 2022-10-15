use bevy::prelude::*;

mod pause_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        //app.add_plugin(pause_menu::PauseMenuPlugin);
    }
}
