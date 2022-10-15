use bevy::prelude::*;
use bevy_mod_wanderlust::*;
use leafwing_input_manager::prelude::*;

pub mod bundle;
mod components;
mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<resources::player_actions::PlayerActions>::default())
            .add_plugin(WanderlustPlugin)
            .add_system_to_stage(CoreStage::PreUpdate, systems::look::look_system)
            .add_system_to_stage(CoreStage::PreUpdate, systems::movement::movement_system);
    }
}

pub fn enable_player_actions(
    mut toggle: ResMut<ToggleActions<resources::player_actions::PlayerActions>>,
) {
    toggle.enabled = true;
}

pub fn disable_player_actions(
    mut toggle: ResMut<ToggleActions<resources::player_actions::PlayerActions>>,
) {
    toggle.enabled = false;
}
