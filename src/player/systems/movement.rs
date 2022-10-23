use bevy::prelude::*;
use bevy_mod_wanderlust::*;
use leafwing_input_manager::prelude::*;

use crate::player::{
    components::{Player, PlayerCamera},
    resources::player_actions::PlayerActions,
};

pub fn movement_system(
    mut body: Query<&mut ControllerInput, With<Player>>,
    camera: Query<&GlobalTransform, (With<PlayerCamera>, Without<Player>)>,
    input: Query<&ActionState<PlayerActions>, With<Player>>,
) {
    let tf = camera.single();

    let mut player_input = body.single_mut();

    let mut dir = Vec3::ZERO;

    let actions_state = input.single();

    if actions_state.pressed(PlayerActions::Left) {
        dir += -tf.right();
    }
    if actions_state.pressed(PlayerActions::Right) {
        dir += tf.right();
    }
    if actions_state.pressed(PlayerActions::Backward) {
        dir += -tf.forward();
    }
    if actions_state.pressed(PlayerActions::Forward) {
        dir += tf.forward();
    }

    dir.y = 0.0;
    player_input.movement = dir.normalize_or_zero();

    player_input.jumping = actions_state.pressed(PlayerActions::Jump);
}
