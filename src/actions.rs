use crate::{GameState, PlayerBody, PlayerCamera};
use ::bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_mod_wanderlust::ControllerInput;
use iyes_loopless::prelude::*;
pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            movement_input.run_in_state(GameState::Unpaused),
        )
        .add_system(mouse_look.run_in_state(GameState::Unpaused));
    }
}

pub fn mouse_look(
    mut cam: Query<&mut Transform, With<PlayerCamera>>,
    mut body: Query<&mut Transform, (With<PlayerBody>, Without<PlayerCamera>)>,
    mut input: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut cam_tf = cam.single_mut();
    let mut body_tf = body.single_mut();

    let dt = time.delta_seconds();
    let sens = 0.12;

    for motion in input.iter() {
        // Vertical
        let rot = cam_tf.rotation;
        cam_tf.rotate(Quat::from_scaled_axis(
            rot * Vec3::X * -motion.delta.y * dt * sens,
        ));

        // Horizontal
        let rot = body_tf.rotation;
        body_tf.rotate(Quat::from_scaled_axis(
            rot * Vec3::Y * -motion.delta.x * dt * sens,
        ));
    }
}

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
