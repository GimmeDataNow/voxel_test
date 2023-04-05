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
        //sampeling for the renderer
        .insert_resource(Msaa::Sample4)
        // movement speed for the fly_cam
        .insert_resource(MovementSettings {
            sensitivity: 0.00010, // default: 0.00012
            speed: 12.0, // default: 12.0
        })
        // change Keybinds if necessary
        .insert_resource(KeyBindings {
            ..Default::default()
        })
        // change wgpu setting to prevent texture blur
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings { features: WgpuFeatures::POLYGON_MODE_LINE, ..default()}
        }).set(ImagePlugin::default_nearest()))
        // setup for the wireframe mode
        .add_plugin(WireframePlugin)
        // adds the fly_cam
        .add_plugin(PlayerPlugin)
        //create the gui world inspector
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_startup_system(spawn_chunks)
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
    if input.just_pressed(KeyCode::L) {
        wireframe_config.global = !wireframe_config.global
    }
}
fn spawn_chunks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    for y in 0..20{
        for x in 0..20{
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
                transform: Transform::from_xyz((x * chunks::CHUNK_SIZE_HORIZONTAL) as f32, 0., (y * chunks::CHUNK_SIZE_HORIZONTAL) as f32),
                ..Default::default()
            });
        }
    }
}