// crate modifiers
#![allow(dead_code)]

// foreign imports
use bevy::{
    prelude::*,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin}
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::{egui, EguiContexts};
use bevy_flycam::prelude::*;
use chunk_logic::position_handling::to_chunk_coordinates;
// use chunks::{chunk::ChunkComp, rendering::rendering_const};

// 'self' imports
mod blocks;
// #[path ="./chunks/chunks.rs"]
// pub mod chunks;
// #[path ="./chunks/position_handling.rs"]
// mod position_handling;
mod chunk_logic;

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
        // .add_system(debug_distance)
        .add_system(lightup_toggle)
        .add_system(ui_example_system)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
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
}

fn wireframe_toggle(mut wireframe_config: ResMut<WireframeConfig>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Numpad0) {
        wireframe_config.global = !wireframe_config.global
    }
}
fn lightup_toggle(mut materials: ResMut<Assets<StandardMaterial>>, query: Query<&mut Handle<StandardMaterial>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Numpad1) {
        for handle in query.iter() {
            match materials.get_mut(handle) {
                Some(material) => material.unlit = !material.unlit,
                None => (),
            }
        }
    }
}

fn spawn_chunks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {

    let texture_handle_stone:Handle<Image> = asset_server.load("textures/stone.png");
    let texture_handle_dirt:Handle<Image> = asset_server.load("textures/dirt.png");


    //let x = 0;
    // let y = 0;
    //for y in 0..20{
        for x in -5..5{
            let chunk = chunk_logic::chunk::ChunkComp::new_simple();
            let white_material = materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle_stone.clone()),
                unlit: false,
                ..Default::default()
            });
            commands.spawn(PbrBundle {
                mesh: meshes.add(chunk.build_mesh_culling()),
                material: white_material.clone(),
                transform: Transform::from_xyz((x * chunk_logic::chunk::CHUNK_SIZE_HORIZONTAL as i32) as f32, 0., (1 * chunk_logic::chunk::CHUNK_SIZE_HORIZONTAL) as f32),
                ..Default::default()
            });
            let chunk = chunk_logic::chunk::ChunkComp::new_simple();
            let white_material = materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle_dirt.clone()),
                unlit: false,
                ..Default::default()
            });
            commands.spawn(PbrBundle {
                mesh: meshes.add(chunk.build_mesh_culling()),
                material: white_material.clone(),
                transform: Transform::from_xyz((x * chunk_logic::chunk::CHUNK_SIZE_HORIZONTAL as i32) as f32, 0., (2 * chunk_logic::chunk::CHUNK_SIZE_HORIZONTAL) as f32),
                ..Default::default()
            });
        }
    // }
}
fn ui_example_system(query: Query<(&mut Transform, &Camera)>, mut contexts: EguiContexts) {
    let translation = query.single().0.translation;
    let offset_chunk_grid = chunk_logic::chunk::CHUNK_SIZE_HORIZONTAL as f32;
    let b = chunk_logic::position_handling::chunk_translation_fix((translation - (translation % offset_chunk_grid)) / offset_chunk_grid);
    let c = translation % offset_chunk_grid;
    egui::Window::new("Info").show(contexts.ctx_mut(), |ui| {
        ui.label("Position:");
        ui.label(translation.round().to_string());
        ui.separator();
        ui.label("Distance to Origin:");
        ui.label(translation.distance(bevy::math::Vec3::ZERO).to_string());
        ui.separator();
        ui.label("Chunk coordinates:");
        ui.label(b.round().to_string());
        ui.separator();
        ui.label("In-chunk coordinates:");
        ui.label(c.round().to_string());
        ui.label("Chunk coordinates 2222:");
        ui.label(to_chunk_coordinates(c).to_string());
        ui.separator();
    });
}


fn debug_distance(mut param_set: ParamSet<(Query<(&mut Transform, &Handle<Mesh>)>, Query<&mut Transform, &Camera>)>) {

    let cam_pos = param_set.p1().single().translation;
    let chunk_center_offset = bevy::math::Vec3::new(-((chunk_logic::chunk::CHUNK_SIZE_HORIZONTAL/2) as f32), 0.0, -((chunk_logic::chunk::CHUNK_SIZE_HORIZONTAL/2) as f32));
    for (transform, _mesh) in param_set.p0().iter(){
        
        let diff = (transform.translation - cam_pos).distance(cam_pos + chunk_center_offset);
        println!("balls {diff}")
    }
}

fn debug_player(mut param_set: ParamSet<(Query<(&mut Transform, &Handle<Mesh>)>, Query<&mut Transform, &Camera>)>) {

    let cam_pos = param_set.p1().single().translation;
    let chunk_center_offset = bevy::math::Vec3::new(-((chunk_logic::chunk::CHUNK_SIZE_HORIZONTAL/2) as f32), 0.0, -((chunk_logic::chunk::CHUNK_SIZE_HORIZONTAL/2) as f32));
    for (transform, _mesh) in param_set.p0().iter(){
        
        let diff = (transform.translation - cam_pos).distance(cam_pos + chunk_center_offset);
        println!("balls {diff}")
    }
}