use bevy::{input::mouse::MouseMotion,prelude::*};
use bevy_mod_wanderlust::*;


#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerBody;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerCamera;


#[derive(Reflect)]
pub struct Sensitivity(pub f32);

pub struct PlayerController;

impl Plugin for PlayerController {
    fn build(&self, app: &mut App) {
        app.add_plugin(WanderlustPlugin)
        .insert_resource(Sensitivity(0.15))
        .add_startup_system(setup_player_controller)
        .add_system_to_stage(CoreStage::PreUpdate, movement_input)
        .add_system(mouse_look);
    }
}

pub fn setup_player_controller(mut commands: Commands){
    commands.spawn_bundle(CharacterControllerBundle{..default()})
    .insert(PlayerBody)
    .with_children(|commands| {
        commands.spawn_bundle(Camera3dBundle{
            transform: Transform::from_xyz(0.,0.5,0.),
            ..default()
        })
        .insert(PlayerCamera);
    });
}

pub fn mouse_look(
    mut cam: Query<&mut Transform, With<PlayerCamera>>,
    mut body: Query<&mut Transform, (With<PlayerBody>, Without<PlayerCamera>)>,
    sensitivity: Res<Sensitivity>,
    mut input: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut cam_tf = cam.single_mut();
    let mut body_tf = body.single_mut();

    let dt = time.delta_seconds();
    let sens = sensitivity.0;

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