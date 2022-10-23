use bevy::{prelude::*, ecs::schedule::StateData};
use bevy_ui_navigation::systems::InputMapping;

mod resources;

pub struct PauseMenuPlugin<T>
where
    T: StateData + Copy,
{
    state: Vec<T>,
}

impl<T:StateData + Copy> Plugin for PauseMenuPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::UiAssets>();
    }
}


impl<T:StateData + Copy> PauseMenuPlugin<T> {
    pub fn from_state(state: Vec<T>) -> Self {
        Self{
            state,
        }
    }
}
