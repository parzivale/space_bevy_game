use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::player::{
    components::{Player, PlayerCamera},
    resources::player_actions::PlayerActions,
};

pub fn look_system(
    mut cam: Query<&mut Transform, With<PlayerCamera>>,
    mut body: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
    input: Query<&ActionState<PlayerActions>, With<Player>>,
    time: Res<Time>,
) {

    let mut cam_tf = cam.single_mut();
    let mut body_tf = body.single_mut();
    let input = input.single();

    let dt = time.delta_seconds();
    let sens = 0.12;

    if input.pressed(PlayerActions::Look) {
        let axis_pair = input.axis_pair(PlayerActions::Look).unwrap();
        let rot = cam_tf.rotation;

        cam_tf.rotate(Quat::from_scaled_axis(
            rot * Vec3::X * -axis_pair.y() * dt * sens,
        ));

        let rot = body_tf.rotation;
        body_tf.rotate(Quat::from_scaled_axis(
            rot * Vec3::Y * -axis_pair.x() * dt * sens,
        ));
    }
}