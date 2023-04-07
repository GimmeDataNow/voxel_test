// due to the way f32 works there are coordinates such as (-0,.,.) which ruins any simple calculations regarding positioning
// this file aims to fix this

use bevy::math::f32::Vec3;

use crate::chunk_logic::chunk;

pub fn chunk_translation_fix(mut translation: Vec3) -> Vec3 {
    if translation.x < 0.0 {
        translation.x = translation.x - 1.0
    }
    if translation.y < 0.0 {
        translation.y = translation.y - 1.0
    }
    if translation.z < 0.0 {
        translation.z = translation.z - 1.0
    }
    translation
}

pub fn to_chunk_coordinates(translation: Vec3) -> Vec3 {
    let offset_chunk_grid = chunk::CHUNK_SIZE_HORIZONTAL as f32;
    let mut return_vec = Vec3::new(translation.x.abs().div_euclid(offset_chunk_grid), translation.y.abs().div_euclid(offset_chunk_grid), translation.y.abs().div_euclid(offset_chunk_grid));

    if translation.x < 0.0 {
        return_vec.x = -return_vec.x - 1.0
    }
    if translation.y < 0.0 {
        return_vec.y = -return_vec.y - 1.0
    }
    if translation.z < 0.0 {
        return_vec.z = -return_vec.z - 1.0
    }
    return_vec
}