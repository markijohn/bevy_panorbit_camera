//! Demonstrates usage with an orthographic camera

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin, PanOrbitCameraSystemSet};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, switch_projection.before(PanOrbitCameraSystemSet))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // help
    commands.spawn(Text::new("Press R to switch projection".to_string()));
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // Camera
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 6.0)),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 1.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        PanOrbitCamera::default(),
    ));
}

fn switch_projection(
    mut next_projection: Local<Projection>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut PanOrbitCamera, &mut Projection)>,
) {
    if key_input.just_pressed(KeyCode::KeyR) {
        let Ok((mut camera, mut projection)) = camera_query.single_mut() else {
            return;
        };
        std::mem::swap(&mut *next_projection, &mut *projection);
        camera.force_update = true;
    }
}
