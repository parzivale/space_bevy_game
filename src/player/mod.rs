use bevy::{input::keyboard::KeyCode, prelude::*};
use bevy_mod_wanderlust::*;
use leafwing_input_manager::prelude::*;

use crate::states::GameState;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerActions {
    Forward,
    Left,
    Right,
    Backward,
    Jump,
    Look,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerBody;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerCamera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let actions_toggle = ToggleActions::<PlayerActions>::default();

        app.add_plugin(InputManagerPlugin::<PlayerActions>::default())
            .add_plugin(WanderlustPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(disable_movement))
            .add_system_set(SystemSet::on_exit(GameState::Paused).with_system(enable_movement))
            .insert_resource(actions_toggle)
            .add_startup_system(setup_player_controller)
            .add_system(mouse_look)
            .add_system(movement_input);
    }
}

fn disable_movement(mut toggle_actions: ResMut<ToggleActions<PlayerActions>>) {
    toggle_actions.enabled = false;
}

fn enable_movement(mut toggle_actions: ResMut<ToggleActions<PlayerActions>>) {
    toggle_actions.enabled = true;
}

fn setup_player_controller(mut commands: Commands) {
    commands
        .spawn_bundle(InputManagerBundle::<PlayerActions> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::W, PlayerActions::Forward),
                (KeyCode::S, PlayerActions::Backward),
                (KeyCode::A, PlayerActions::Left),
                (KeyCode::D, PlayerActions::Right),
                (KeyCode::Space, PlayerActions::Jump),
            ])
            .insert(DualAxis::mouse_motion(), PlayerActions::Look)
            .build(),
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

fn movement_input(
    mut body: Query<&mut ControllerInput, With<PlayerBody>>,
    camera: Query<&GlobalTransform, (With<PlayerCamera>, Without<PlayerBody>)>,
    input: Query<&ActionState<PlayerActions>, With<PlayerBody>>,
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

fn mouse_look(
    mut cam: Query<&mut Transform, With<PlayerCamera>>,
    mut body: Query<&mut Transform, (With<PlayerBody>, Without<PlayerCamera>)>,
    input: Query<&ActionState<PlayerActions>, With<PlayerBody>>,
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
