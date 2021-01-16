use bevy::prelude::*;
use bevy_orbit_camera::{OrbitCamera, OrbitCameraPlugin};
fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(OrbitCameraPlugin)
        .add_startup_system(setup_camera.system())
        .add_startup_system(setup_scene.system())
        .run();
}
fn setup_camera(commands: &mut Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(OrbitCamera::default());
}

fn setup_scene(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..Default::default()
        });
}
