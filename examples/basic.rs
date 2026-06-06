//! Basic example rendering an amoeba.
//! `cargo run --example basic`
use std::f32::consts::PI;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    input::common_conditions::{input_just_pressed, input_toggle_active},
    mesh::MeshTag,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

use bevy_amoeba::{SoftBody, SoftBodyAssets, SoftBodyMaterial, SoftBodyNode, SoftBodyPlugin};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WireframePlugin::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            FpsOverlayPlugin {
                config: FpsOverlayConfig::default(),
            },
            SoftBodyPlugin,
        ))
        .init_resource::<CustomSoftBodyNodeAssets>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_wireframe.run_if(input_just_pressed(KeyCode::Space)),
                rotate.run_if(input_toggle_active(false, KeyCode::KeyR)),
            ),
        )
        .run();
}

#[derive(Component, Reflect)]
#[require(
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
    })]
struct MainCamera;

#[derive(Resource, Reflect, Clone)]
struct CustomSoftBodyNodeAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}
impl FromWorld for CustomSoftBodyNodeAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: world.add_asset(Circle { radius: 0.1 }),
            material: world.add_asset(StandardMaterial {
                base_color: Color::WHITE.into(),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
        }
    }
}

#[derive(Component, Reflect)]
#[component(on_add = CustomSoftBodyNode::on_add)]
struct CustomSoftBodyNode {
    radius: f32,
}
impl CustomSoftBodyNode {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let CustomSoftBodyNodeAssets { mesh, material } =
            world.resource::<CustomSoftBodyNodeAssets>().clone();
        let radius = world.entity(context.entity).get::<Self>().unwrap().radius;
        world.commands().entity(context.entity).insert((
            Name::new("SoftBodyNode"),
            Mesh3d(mesh),
            MeshMaterial3d(material),
            SoftBodyNode { radius },
        ));
    }
}

fn setup(mut commands: Commands, assets: Res<SoftBodyAssets>) {
    commands.spawn(MainCamera);
    commands.spawn(DirectionalLight::default());

    let z = -0.1;
    let x_offset = 1.6;

    let node1 = commands
        .spawn((
            CustomSoftBodyNode { radius: 0.6 },
            Transform::from_xyz(0.1 - x_offset, -0.1, z),
        ))
        .id();
    let node2 = commands
        .spawn((
            CustomSoftBodyNode { radius: 0.5 },
            Transform::from_xyz(0.3 - x_offset, 0.3, z),
        ))
        .id();
    let node3 = commands
        .spawn((
            CustomSoftBodyNode { radius: 0.4 },
            Transform::from_xyz(-0.2 - x_offset, -0.2, z),
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
            CustomSoftBodyNode { radius: 0.6 },
            Transform::from_xyz(0.1, -0.1, z),
        ))
        .id();
    let node5 = commands
        .spawn((
            CustomSoftBodyNode { radius: 0.5 },
            Transform::from_xyz(0.3, 0.3, z),
        ))
        .id();
    let node6 = commands
        .spawn((
            CustomSoftBodyNode { radius: 0.4 },
            Transform::from_xyz(-0.2, -0.2, z),
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
            CustomSoftBodyNode { radius: 0.6 },
            Transform::from_xyz(0.1 + x_offset, -0.1, z),
        ))
        .id();
    let node8 = commands
        .spawn((
            CustomSoftBodyNode { radius: 0.5 },
            Transform::from_xyz(0.3 + x_offset, 0.3, z),
        ))
        .id();
    let node9 = commands
        .spawn((
            CustomSoftBodyNode { radius: 0.4 },
            Transform::from_xyz(-0.2 + x_offset, -0.2, z),
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
