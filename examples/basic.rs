//! Basic example rendering an amoeba.
//! `cargo run --example basic`
use std::f32::consts::PI;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    input::common_conditions::{input_just_pressed, input_toggle_active},
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

use bevy_amoeba::{SoftBody, SoftBodyMaterial, SoftBodyNode, SoftBodyPlugin};
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
        .insert_resource(ClearColor(Color::WHITE))
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
        translation: Vec3::new(0.0, 0.0, 8.0),
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
                base_color: Color::BLACK.into(),
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

#[derive(Component, Reflect, Copy, Clone)]
#[component(on_add = CustomSoftBody::on_add)]
#[require(Name::new("SoftBody"))]
struct CustomSoftBody {
    pub offset: Vec3,
}
impl CustomSoftBody {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let Self { offset } = world.entity(context.entity).get::<Self>().unwrap().clone();
        let entities = [
            (
                CustomSoftBodyNode { radius: 0.6 },
                Transform {
                    translation: Vec3::new(0.1, -0.1, 0.0) + offset,
                    ..default()
                },
            ),
            (
                CustomSoftBodyNode { radius: 0.5 },
                Transform {
                    translation: Vec3::new(0.3, 0.3, 0.0) + offset,
                    ..default()
                },
            ),
            (
                CustomSoftBodyNode { radius: 0.4 },
                Transform {
                    translation: Vec3::new(-0.2, -0.2, 0.0) + offset,
                    ..default()
                },
            ),
        ]
        .into_iter()
        .map(|bundle| world.commands().spawn(bundle).id())
        .collect();
        world
            .commands()
            .entity(context.entity)
            .insert(SoftBody(entities));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(MainCamera);
    commands.spawn(DirectionalLight::default());

    let z = -0.1;
    let x_step = 1.6;
    let y_step = 1.6;
    let x_total = 8;
    let y_total = 8;

    for y in 0..y_total {
        for x in 0..x_total {
            commands.spawn(CustomSoftBody {
                offset: Vec3::new(
                    (x - x_total / 2) as f32 * x_step,
                    (y - y_total / 2) as f32 * y_step,
                    z,
                ),
            });
        }
    }
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
