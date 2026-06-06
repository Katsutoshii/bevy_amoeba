//! Basic example rendering an amoeba.
//! `cargo run --example basic`
use std::f32::consts::PI;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    input::common_conditions::{input_just_pressed, input_toggle_active},
    mesh::MeshTag,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

use bevy_amoeba::{SoftBody, SoftBodyAssets, SoftBodyMaterial, SoftBodyNode, SoftBodyPlugin};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        WireframePlugin::default(),
        EguiPlugin::default(),
        WorldInspectorPlugin::new(),
        FpsOverlayPlugin {
            config: FpsOverlayConfig::default(),
        },
        SoftBodyPlugin,
    ))
    .add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            toggle_wireframe.run_if(input_just_pressed(KeyCode::Space)),
            rotate.run_if(input_toggle_active(false, KeyCode::KeyR)),
        ),
    );
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<SoftBodyAssets>,
) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: PI / 2.0,
            near: 0.1,
            far: 2000.,
            ..default()
        }),
        Transform {
            translation: Vec3::new(0.0, 0.0, 2.0),
            ..default()
        },
    ));
    commands.spawn(DirectionalLight::default());

    let circle = meshes.add(Circle { radius: 0.1 });
    let white_material = std_materials.add(StandardMaterial {
        base_color: Color::WHITE.into(),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let z = -0.1;
    let x_offset = 1.6;

    let node1 = commands
        .spawn((
            Name::new("Node1"),
            SoftBodyNode { radius: 0.6 },
            Transform::from_xyz(0.1 - x_offset, -0.1, z),
            Mesh3d(circle.clone()),
            MeshMaterial3d(white_material.clone()),
        ))
        .id();
    let node2 = commands
        .spawn((
            Name::new("Node2"),
            SoftBodyNode { radius: 0.5 },
            Transform::from_xyz(0.3 - x_offset, 0.3, z),
            Mesh3d(circle.clone()),
            MeshMaterial3d(white_material.clone()),
        ))
        .id();
    let node3 = commands
        .spawn((
            Name::new("Node3"),
            SoftBodyNode { radius: 0.4 },
            Transform::from_xyz(-0.2 - x_offset, -0.2, z),
            Mesh3d(circle.clone()),
            MeshMaterial3d(white_material.clone()),
        ))
        .id();
    commands.spawn((
        Name::new("SoftBodyMesh1"),
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(assets.material.clone()),
        Transform { ..default() },
        SoftBody(vec![node1, node2, node3]),
        MeshTag(0),
    ));

    let node4 = commands
        .spawn((
            Name::new("Node4"),
            SoftBodyNode { radius: 0.6 },
            Transform::from_xyz(0.1, -0.1, z),
            Mesh3d(circle.clone()),
            MeshMaterial3d(white_material.clone()),
        ))
        .id();
    let node5 = commands
        .spawn((
            Name::new("Node5"),
            SoftBodyNode { radius: 0.5 },
            Transform::from_xyz(0.3, 0.3, z),
            Mesh3d(circle.clone()),
            MeshMaterial3d(white_material.clone()),
        ))
        .id();
    let node6 = commands
        .spawn((
            Name::new("Node6"),
            SoftBodyNode { radius: 0.4 },
            Transform::from_xyz(-0.2, -0.2, z),
            Mesh3d(circle.clone()),
            MeshMaterial3d(white_material.clone()),
        ))
        .id();
    commands.spawn((
        Name::new("SoftBodyMesh2"),
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(assets.material.clone()),
        Transform { ..default() },
        SoftBody(vec![node4, node5, node6]),
        MeshTag(1),
    ));

    let node7 = commands
        .spawn((
            Name::new("Node7"),
            SoftBodyNode { radius: 0.6 },
            Transform::from_xyz(0.1 + x_offset, -0.1, z),
            Mesh3d(circle.clone()),
            MeshMaterial3d(white_material.clone()),
        ))
        .id();
    let node8 = commands
        .spawn((
            Name::new("Node8"),
            SoftBodyNode { radius: 0.5 },
            Transform::from_xyz(0.3 + x_offset, 0.3, z),
            Mesh3d(circle.clone()),
            MeshMaterial3d(white_material.clone()),
        ))
        .id();
    let node9 = commands
        .spawn((
            Name::new("Node9"),
            SoftBodyNode { radius: 0.4 },
            Transform::from_xyz(-0.2 + x_offset, -0.2, z),
            Mesh3d(circle.clone()),
            MeshMaterial3d(white_material.clone()),
        ))
        .id();
    commands.spawn((
        Name::new("SoftBodyMesh3"),
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(assets.material.clone()),
        Transform { ..default() },
        SoftBody(vec![node7, node8, node9]),
        MeshTag(2),
    ));

    let mut text = "Press 'R' to pause/resume rotation".to_string();
    text.push_str("\nPress 'Space' to toggle wireframes");

    commands.spawn((
        Text::new(text),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn toggle_wireframe(mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = !wireframe_config.global;
}

fn rotate(
    mut query: Query<&mut Transform, With<MeshMaterial3d<SoftBodyMaterial>>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_secs() / 2.0);
    }
}
