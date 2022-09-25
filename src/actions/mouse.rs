use crate::{PlayerBody, PlayerCamera};
use ::bevy::{input::mouse::MouseMotion, prelude::*};

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
