use crate::utils::constrain_distance;
use bevy::prelude::*;

const MAX_DISTANCE: f32 = 100.;

#[derive(Component)]
struct Joint;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_arm)
        .add_systems(Update, update_constraint)
        .run();
}

fn setup_arm(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Joint,
        Mesh2d::from(meshes.add(Annulus::new(5., 10.))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::default(),
        Visibility::default(),
    ));
}

fn update_constraint(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut query: Single<&mut Transform, With<Joint>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Convert cursor position to world coordinates
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let mut transform = query;

    if ((transform.translation.truncate() - point).length() > MAX_DISTANCE) {
        let new_position = constrain_distance(&transform.translation.truncate(), &point, MAX_DISTANCE);

        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
    }

    gizmos.circle_2d(point, MAX_DISTANCE, Color::WHITE);
}
