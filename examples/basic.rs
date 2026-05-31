//! Basic example rendering an amoeba.
//! `cargo run --example basic`
use std::f32::consts::PI;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    input::common_conditions::input_just_pressed,
    input::common_conditions::input_toggle_active,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

use bevy_amoeba::{AmoebaPlugin, MeshBuilder, Particle2dBuffer, SoftBodyMaterial};
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
        AmoebaPlugin,
    ))
    .add_systems(Startup, setup);
    app.add_systems(
        Update,
        toggle_wireframe.run_if(input_just_pressed(KeyCode::Space)),
    );
    app.add_systems(
        Update,
        rotate.run_if(input_toggle_active(false, KeyCode::KeyR)),
    );
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SoftBodyMaterial>>,
    asset_server: Res<AssetServer>,
    particles: Res<Particle2dBuffer>,
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
    commands.spawn((
        Mesh3d(meshes.add(MeshBuilder::circle_ngon(1.0, Particle2dBuffer::MAX_PARTICLES).build())),
        MeshMaterial3d(materials.add(SoftBodyMaterial {
            color: Color::WHITE.to_linear(),
            color_texture: Some(asset_server.load("textures/bubble_7.png")),
            particles: particles.0.clone(),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform { ..default() },
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
