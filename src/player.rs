use crate::GameState;
use bevy::{input::{mouse::MouseMotion, keyboard::KeyCode}, prelude::*};
use bevy_mod_wanderlust::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerBody;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerCamera;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Forward,
    Left,
    Right,
    Backward,
    Jump,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WanderlustPlugin)
            .add_plugin(InputManagerPlugin::<Action>::default())
            .add_startup_system(setup_player_controller)
            .add_system_set(
                SystemSet::on_update(GameState::Unpaused)
                .with_system(mouse_look)
                .with_system(movement_input)
            );
    }
}

pub fn setup_player_controller(mut commands: Commands) {
    commands
        .spawn_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::W, Action::Forward),
                (KeyCode::S, Action::Backward),
                (KeyCode::A, Action::Left),
                (KeyCode::D, Action::Right),
                (KeyCode::Space, Action::Jump),
            ]),
        })
        .insert_bundle(CharacterControllerBundle { ..default() })
        .insert(PlayerBody)
        .with_children(|commands| {
            commands
                .spawn_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0., 0.5, 0.),
                    ..default()
                })
                .insert(PlayerCamera);
        });
}

pub fn movement_input(
    mut body: Query<&mut ControllerInput, With<PlayerBody>>,
    camera: Query<&GlobalTransform, (With<PlayerCamera>, Without<PlayerBody>)>,
    input: Query<&ActionState<Action>, With<PlayerBody>>,
) {
    let tf = camera.single();

    let mut player_input = body.single_mut();

    let mut dir = Vec3::ZERO;

    let actions_state = input.single();

    if actions_state.pressed(Action::Left) {
        dir += -tf.right();
    }
    if actions_state.pressed(Action::Right) {
        dir += tf.right();
    }
    if actions_state.pressed(Action::Backward) {
        dir += -tf.forward();
    }
    if actions_state.pressed(Action::Forward) {
        dir += tf.forward();
    }

    dir.y = 0.0;
    player_input.movement = dir.normalize_or_zero();

    player_input.jumping = actions_state.pressed(Action::Jump);
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
