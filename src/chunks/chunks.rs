#![allow(dead_code)]


// standard imports
use crate::blocks::{self, Block, BlockType};
use bevy::prelude::Mesh;

// my imports
#[path ="./rendering/rendering_const.rs"]
mod rendering_const;

#[path ="./rendering/chunk_mesh_builder.rs"]
mod chunk_mesh_builder;


//chunk constants
const CHUNK_SIZE_HORIZONTAL: usize = 16;
const CHUNK_HEIGHT: usize = 5; 

pub struct Chunk {
    chunk_list: [[[blocks::Block; CHUNK_SIZE_HORIZONTAL]; CHUNK_HEIGHT]; CHUNK_SIZE_HORIZONTAL],
    mesh_builder: chunk_mesh_builder::ChunkMeshBuilder,
}
pub struct ChunkComp {
    chunk_list: [[[blocks::Block; CHUNK_SIZE_HORIZONTAL]; CHUNK_HEIGHT]; CHUNK_SIZE_HORIZONTAL],
    mesh_builder: chunk_mesh_builder::ChunkMeshBuilder,
}

impl Chunk {
    
    pub fn new() -> Self {
        let stone = [[[Block::new(BlockType::Stone, blocks::Facing::None); CHUNK_SIZE_HORIZONTAL]; CHUNK_HEIGHT]; CHUNK_SIZE_HORIZONTAL];
        
        // Trrain gen HERE

        Chunk {
            chunk_list: stone,
            mesh_builder: chunk_mesh_builder::ChunkMeshBuilder::new()
        }
    }


    pub fn build_mesh(mut self) -> Mesh {
        for x in 0..CHUNK_SIZE_HORIZONTAL {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE_HORIZONTAL {
                    let val = &mut self.chunk_list[x][y][z].get_base_properties().transparency;
                    let coord = [x as u32, y as u32, z as u32];

                    let enum_transparency_opaque = blocks::Transparency::Opaque;
                    //FAULT HERE
                    // DISPARITY BETWEEN ITERARATOR AND 
                    if *val == enum_transparency_opaque {
                        continue;
                    }
                    
                    if x == 0 || self.chunk_list[x - 1][y][z].get_base_properties().transparency == enum_transparency_opaque {
                        self.mesh_builder.add_face(coord, 2);
                    }
                    
                    if x == CHUNK_SIZE_HORIZONTAL-1 || self.chunk_list[x + 1][y][z].get_base_properties().transparency == enum_transparency_opaque {
                        self.mesh_builder.add_face(coord, 3);
                    }
                    
                    if y == 0 || self.chunk_list[x][y - 1][z].get_base_properties().transparency == enum_transparency_opaque {
                        self.mesh_builder.add_face(coord, 5);
                    }
                    
                    if y == CHUNK_HEIGHT-1 || self.chunk_list[x][y + 1][z].get_base_properties().transparency == enum_transparency_opaque {
                        self.mesh_builder.add_face(coord, 0);
                    }
                    
                    if z == 0 || self.chunk_list[x][y][z - 1].get_base_properties().transparency == enum_transparency_opaque {
                        self.mesh_builder.add_face(coord, 1);
                    }
                    
                    if z == CHUNK_SIZE_HORIZONTAL-1 || self.chunk_list[x][y][z + 1].get_base_properties().transparency == enum_transparency_opaque {
                        self.mesh_builder.add_face(coord, 4);
                    }
                    //for i in 0..5 {
                    //    self.mesh_builder.add_face(coord, i);
                    //}
                }
            }
        }

        self.mesh_builder.build()
    }
}
