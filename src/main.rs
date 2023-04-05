// crate modifiers
#![allow(dead_code)]

// foreign imports
use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin}
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_flycam::prelude::*;

// 'self' imports
mod blocks;
#[path ="./chunks/chunks.rs"]
pub mod chunks;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(MovementSettings {
            sensitivity: 0.00010, // default: 0.00012
            speed: 12.0, // default: 12.0
        })
        .insert_resource(KeyBindings {
            ..Default::default()
        })
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings { features: WgpuFeatures::POLYGON_MODE_LINE, ..default()}
        }).set(ImagePlugin::default_nearest())
        )
        .add_plugin(WireframePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(wireframe_toggle)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(20.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    let chunk = chunks::ChunkComp::new_simple();
    let texture_handle = asset_server.load("textures/stone.png");
    let white_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        unlit: false,
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(chunk.build_mesh_culling()),
        material: white_material.clone(),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
}
fn wireframe_toggle(mut wireframe_config: ResMut<WireframeConfig>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Numpad0) {
        wireframe_config.global = !wireframe_config.global
    }
}