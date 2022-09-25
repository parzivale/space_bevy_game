use ::bevy_rapier3d::prelude::*;
use bevy::prelude::*;

pub struct WorldGen;

impl Plugin for WorldGen {
    fn build(&self, app: &mut App) {
        app.add_startup_system(plane);
    }
}

fn plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            ..default()
        })
        .insert(Collider::cuboid(10., 1., 10.));

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
