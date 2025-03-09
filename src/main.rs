mod utils;

use crate::utils::*;
use bevy::prelude::*;
use std::f32::consts::PI;

const MAX_DISTANCE: f32 = 300.;
const MAX_JOINTS: usize = 3;
const MAX_ALLOWED_ANGLE: f32 = PI / 8.;

#[derive(Component)]
struct Joint;

#[derive(Component)]
struct Arm {
    anchor: Vec2,
    joints: Vec<Vec2>, // which is a array of Joint?

    angles: Vec<f32>,
}

impl Arm {
    fn new(
        anchor: Vec2,
        joint_count: usize,
        segment_length: f32,
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let mut joints = Vec::new();
        let mut angles = Vec::new();

        for i in 0..joint_count {
            let joint_position = anchor + Vec2::X * segment_length * (i as f32);
            joints.push(joint_position);
            angles.push(0.0);

            commands.spawn((
                Mesh2d::from(meshes.add(Circle::new(5.0))), // ✅ Crée un cercle de rayon 5
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))), // ✅ Définit la couleur
                Transform::from_xyz(joint_position.x, joint_position.y, 0.0),
                Visibility::default(),
                Joint,
            ));
        }

        Self {
            anchor,
            joints,
            angles,
        }
    }

    fn update(&mut self, target: Vec2) {
        self.angles[0] = (target - self.joints[0]).to_angle();
        self.joints[0] = target;
        let segment_length = MAX_DISTANCE / self.joints.len() as f32;

        // Forward pass
        for i in 1..self.joints.len() {
            let angle = self.joints[i - 1].angle_to(self.joints[i]);

            self.angles[i] = contraint_angle_v2(angle, self.angles[i - 1], MAX_ALLOWED_ANGLE);
            self.joints[i] = self.joints[i - 1]
                - Vec2::new(self.angles[i].cos(), self.angles[i].sin()) * segment_length;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Space Invaders"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            ),
        )
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

    let arm = Arm::new(
        Vec2::new(0.0, 0.0),
        MAX_JOINTS,
        MAX_DISTANCE / (MAX_JOINTS as f32),
        &mut commands,
        meshes,
        materials,
    ); // 3 joints

    commands.spawn((arm,));
}

fn update_constraint(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut arm_query: Query<&mut Arm>,
    mut joint_query: Query<&mut Transform, With<Joint>>,
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

    let target = point; // Mouse position

    let Ok(mut arm) = arm_query.get_single_mut() else {
        return;
    };

    arm.update(target);

    for (i, mut transform) in joint_query.iter_mut().enumerate() {
        if i < arm.joints.len() {
            transform.translation.x = arm.joints[i].x;
            transform.translation.y = arm.joints[i].y;
        }
    }
}
