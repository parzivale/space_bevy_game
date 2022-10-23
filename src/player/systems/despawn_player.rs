use bevy::prelude::*;

use crate::player::components::Player;

pub fn despawn_player_entity(mut commands: Commands, entity: Query<Entity, With<Player>>) {
    commands.entity(entity.single()).despawn_recursive();
}
