use bevy::prelude::*;
use bevy_mod_wanderlust::*;
use leafwing_input_manager::prelude::*;

use crate::player::{
    components::{Player, PlayerCamera},
    resources::player_actions::PlayerActions,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    controller: CharacterControllerBundle,
    #[bundle]
    input: InputManagerBundle<PlayerActions>,
    marker: Player,
    name: Name,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            controller: Default::default(),
            input: InputManagerBundle::<PlayerActions> {
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
            },
            marker: Player,
            name: Name::new("Player"),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    #[bundle]
    camera: Camera3dBundle,
    marker: PlayerCamera,
    name: Name,
}

impl Default for PlayerCameraBundle {
    fn default() -> Self {
        Self {
            camera: Camera3dBundle {
                transform: Transform::from_xyz(0., 0.5, 0.),
                ..default()
            },
            marker: PlayerCamera,
            name: Name::new("Player Camera"),
        }
    }
}

pub fn spawn_player_system(mut commands: Commands) {
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
        .insert(Player)
        .insert(Name::new("Player"))
        .with_children(|commands| {
            commands
                .spawn_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0., 0.5, 0.),
                    ..default()
                })
                .insert(PlayerCamera)
                .insert(Name::new("Camera"));
        });
}
