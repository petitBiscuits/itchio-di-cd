use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin};

#[derive(Asset,AsBindGroup, TypePath, Debug, Clone)]
pub struct MandelbrotMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub zoom: f32,
}

impl Material2d for MandelbrotMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mandelbrot.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
) {
    // Spawn a 2D camera.
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 2.0, // Increase to zoom out, decrease to zoom in
            ..OrthographicProjection::default_2d()
        },
        Transform::default(),
        GlobalTransform::default(),
    ));

    // Create a quad mesh that covers the viewport.
    let mesh = meshes.add(Mesh::from(Rectangle::new(10.0, 10.0)));

    // Create the Mandelbrot material with initial uniform values.
    let material = materials.add(MandelbrotMaterial {
        time: 0.0,
        zoom: 1.0,
    });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}

fn animate_mandelbrot(
    time: Res<Time>,
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
) {
    // Update the time uniform for every MandelbrotMaterial.
    for (_id, material) in materials.iter_mut() {
        material.time += time.delta_secs();
    }
}

fn zoom_out_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    if keys.pressed(KeyCode::ArrowUp) {
        for mut proj in query.iter_mut() {
            proj.scale *= 1.1;
        }
    }
    if keys.pressed(KeyCode::ArrowDown) {
        for mut proj in query.iter_mut() {
            proj.scale *= 0.9;
        }
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        for mut proj in query.iter_mut() {
            proj.scale = 2.0;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<MandelbrotMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, animate_mandelbrot)
        .add_systems(Update, zoom_out_camera)
        .run();
}
