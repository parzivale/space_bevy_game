use bevy::{ecs::schedule::StateData, prelude::*};
use bevy_mod_wanderlust::*;
use leafwing_input_manager::prelude::*;


pub mod bundle;
mod components;
mod resources;
mod systems;

pub struct PlayerPlugin<T: StateData + Copy> {
    state: T,
}

impl<T: StateData + Copy> Plugin for PlayerPlugin<T> {
    fn build(&self, app: &mut App) {
        let update_systems = SystemSet::on_update(self.state)
            .with_system(systems::look::look_system)
            .with_system(systems::movement::movement_system);

        let enter_systems =
            SystemSet::on_enter(self.state).with_system(systems::spawn_player::spawn_player_entity);

        let exit_systems = SystemSet::on_exit(self.state)
            .with_system(systems::despawn_player::despawn_player_entity);

        app.add_plugin(InputManagerPlugin::<resources::player_actions::PlayerActions>::default())
            .add_plugin(WanderlustPlugin)
            .add_system_set(update_systems)
            .add_system_set(enter_systems)
            .add_system_set(exit_systems);
    }
}

impl<T: StateData + Copy> PlayerPlugin<T> {
    pub fn from_state(s: T) -> Self {
        Self { state: s }
    }
}
