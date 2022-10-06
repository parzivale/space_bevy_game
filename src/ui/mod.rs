use bevy::prelude::*;

mod pause_menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(pause_menu::PauseMenuPlugin);
    }
}
