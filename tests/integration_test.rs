use bevy::scene::ScenePlugin;
use bevy::{asset::AssetPlugin, input::InputPlugin, prelude::*, render::mesh::MeshPlugin};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use space_bevy_game::{player::PlayerActions, *};
#[test]
fn test_player() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(AssetPlugin)
        .add_plugin(MeshPlugin)
        .add_plugin(ScenePlugin);

    println!("got to here");
    app.add_state(states::GameState::Unpaused)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(player::PlayerPlugin);

    app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::A);
    app.update();
    assert!(app
        .world
        .resource::<ActionState<PlayerActions>>()
        .pressed(PlayerActions::Left));
}
