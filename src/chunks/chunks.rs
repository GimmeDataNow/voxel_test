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
    blocks: [[[blocks::Block; CHUNK_SIZE_HORIZONTAL]; CHUNK_HEIGHT]; CHUNK_SIZE_HORIZONTAL],
}
impl Chunk {
    pub fn new(block: blocks::Block) -> Self {
        Chunk { blocks: [[[block; CHUNK_SIZE_HORIZONTAL]; CHUNK_HEIGHT]; CHUNK_SIZE_HORIZONTAL] }
    }
    pub fn new_stone() -> Self {
        Chunk { blocks: [[[Block::new(BlockType::Stone, blocks::Facing::XPositive); CHUNK_SIZE_HORIZONTAL]; CHUNK_HEIGHT]; CHUNK_SIZE_HORIZONTAL] }
    }
}

pub struct ChunkComp {
    chunk: Chunk,
    chunk_mesh: chunk_mesh_builder::ChunkMeshBuilder,
}

impl ChunkComp {
    
    pub fn new() -> Self {
        //let stone = Chunk::new_stone();
        
        // Trrain gen HERE

        ChunkComp {
            chunk: Chunk::new_stone(),
            chunk_mesh: chunk_mesh_builder::ChunkMeshBuilder::new()
        }
    }


    pub fn build_mesh(mut self) -> Mesh {
        for x in 0..CHUNK_SIZE_HORIZONTAL {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE_HORIZONTAL {

                    // var due to repeated use
                    let val = &mut self.chunk.blocks[x][y][z].get_base_properties().transparency;


                    let coord = [x as u32, y as u32, z as u32];

                    // makes the the code less verbose
                    let opaque = blocks::Transparency::Opaque;

                    // ignore any opaque cases
                    // opaque blocks need to be handled seperately
                    if *val == opaque {
                        continue;
                    }

                    // mesh builder for the chunk
                    if x == 0 || self.chunk.blocks[x - 1][y][z].get_base_properties().transparency == opaque {
                        self.chunk_mesh.add_face(coord, 2);
                    }
                    
                    if x == CHUNK_SIZE_HORIZONTAL-1 || self.chunk.blocks[x + 1][y][z].get_base_properties().transparency == opaque {
                        self.chunk_mesh.add_face(coord, 3);
                    }
                    
                    if y == 0 || self.chunk.blocks[x][y - 1][z].get_base_properties().transparency == opaque {
                        self.chunk_mesh.add_face(coord, 5);
                    }
                    
                    if y == CHUNK_HEIGHT-1 || self.chunk.blocks[x][y + 1][z].get_base_properties().transparency == opaque {
                        self.chunk_mesh.add_face(coord, 0);
                    }
                    
                    if z == 0 || self.chunk.blocks[x][y][z - 1].get_base_properties().transparency == opaque {
                        self.chunk_mesh.add_face(coord, 1);
                    }
                    
                    if z == CHUNK_SIZE_HORIZONTAL-1 || self.chunk.blocks[x][y][z + 1].get_base_properties().transparency == opaque {
                        self.chunk_mesh.add_face(coord, 4);
                    }
                    //for i in 0..5 {
                    //    self.mesh_builder.add_face(coord, i);
                    //}
                }
            }
        }

        self.chunk_mesh.build()
    }
}
