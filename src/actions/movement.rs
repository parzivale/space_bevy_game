use crate::{PlayerBody, PlayerCamera};
use ::bevy::prelude::*;
use bevy_mod_wanderlust::ControllerInput;

pub fn movement_input(
    mut body: Query<&mut ControllerInput, With<PlayerBody>>,
    camera: Query<&GlobalTransform, (With<PlayerCamera>, Without<PlayerBody>)>,
    input: Res<Input<KeyCode>>,
) {
    let tf = camera.single();

    let mut player_input = body.single_mut();

    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::A) {
        dir += -tf.right();
    }
    if input.pressed(KeyCode::D) {
        dir += tf.right();
    }
    if input.pressed(KeyCode::S) {
        dir += -tf.forward();
    }
    if input.pressed(KeyCode::W) {
        dir += tf.forward();
    }
    dir.y = 0.0;
    player_input.movement = dir.normalize_or_zero();

    player_input.jumping = input.pressed(KeyCode::Space);
}
