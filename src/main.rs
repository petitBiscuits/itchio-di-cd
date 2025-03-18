use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin};

const MOVEMENT_SPEED: f32 = 0.1;

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct MandelbrotMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub zoom: f32,
    #[uniform(0)]
    pub offsetx: f32,
    #[uniform(0)]
    pub offsety: f32,
}
#[derive(Resource)]
struct Manager {
    value: bool,
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
        offsetx: 0.0,
        offsety: 0.0,
    });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));

    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    commands.insert_resource(Manager { value: true });
}

fn animate_mandelbrot(
    time: Res<Time>,
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
    mut text: Single<&mut Text>,
    manager: ResMut<Manager>,
) {
    // Update the time uniform for every MandelbrotMaterial.
    for (_id, material) in materials.iter_mut() {
        if manager.value {
            material.time += time.delta_secs();
        }

        text.0 = "Mandelbrot set!\n".to_string();
        text.push_str(&format!("exponents: {}\n", material.time / 2.));
        text.push_str(&format!(
            "offset: ({}, {})\n",
            material.offsetx, material.offsety
        ));
        text.push_str(&format!("zoom: {}\n", material.zoom));
    }
}

fn zoom_out_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
    mut manager: ResMut<Manager>,
) {
    let mut zoom: f32 = 1.0;
    let mut offset: Vec2 = Vec2::new(0.0, 0.0);

    for (_id, material) in materials.iter_mut() {
        zoom = material.zoom;
        offset = Vec2::new(material.offsetx, material.offsety);
    }

    if keys.pressed(KeyCode::ArrowDown) {
        zoom *= 1.1;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        zoom *= 0.9;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        zoom = 1.;
    }
    if keys.pressed(KeyCode::KeyW) {
        offset += Vec2::new(0.0, MOVEMENT_SPEED) * zoom;
    }
    if keys.pressed(KeyCode::KeyS) {
        offset += Vec2::new(0.0, -MOVEMENT_SPEED) * zoom;
    }
    if keys.pressed(KeyCode::KeyA) {
        offset += Vec2::new(-MOVEMENT_SPEED, 0.0) * zoom;
    }
    if keys.pressed(KeyCode::KeyD) {
        offset += Vec2::new(MOVEMENT_SPEED, 0.0) * zoom;
    }
    if keys.just_pressed(KeyCode::Space) {
        manager.value = !manager.value;
    }

    for (_id, material) in materials.iter_mut() {
        material.zoom = zoom;
        material.offsetx = offset.x;
        material.offsety = offset.y;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(Material2dPlugin::<MandelbrotMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, animate_mandelbrot)
        .add_systems(Update, zoom_out_camera)
        .run();
}
