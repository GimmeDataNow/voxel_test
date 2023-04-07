// crate modifiers
#![allow(dead_code)]

// foreign imports
use bevy::prelude::Mesh;

// 'self' imports
use crate::blocks::{self, Block, BlockType};
// #[path ="./rendering/rendering_const.rs"]
// mod rendering_const;

#[path ="./rendering/chunk_mesh_builder.rs"]
mod chunk_mesh_builder;
//use super::chunks::chunk_mesh_builder;


//chunk constants
pub const CHUNK_SIZE_HORIZONTAL: usize = 16;
pub const CHUNK_HEIGHT: usize = 5; 

/// # Description:
/// This struct holds the ```Block``` struct and thus the base configuration of elements in a ```Chunk```. This struct later on needs to be saved to a file to allow for loading and saving worlds.
/// # Structure:
/// ```
/// pub struct Chunk {
///     blocks: [[[blocks::Block; CHUNK_SIZE_HORIZONTAL]; CHUNK_HEIGHT]; CHUNK_SIZE_HORIZONTAL],
/// }
/// ```
pub struct Chunk {
    blocks: [[[blocks::Block; CHUNK_SIZE_HORIZONTAL]; CHUNK_HEIGHT]; CHUNK_SIZE_HORIZONTAL],
}

/// # Description:
/// This struct holds the ```Chunk``` struct along with the ```ChunkMeshBuilder``` struct. The ```ChunkMeshBuilder``` is later used to render the entity to the screen.
/// The grouping of the ```Chunk``` and ```ChunkMeshBuilder``` makes access later on more convienient
/// # Structure:
/// ```
/// pub struct ChunkComp {
///     chunk: Chunk,
///     chunk_mesh: chunk_mesh_builder::ChunkMeshBuilder,
/// }
/// ```
pub struct ChunkComp {
    chunk: Chunk,
    chunk_mesh: chunk_mesh_builder::ChunkMeshBuilder,
}

impl Chunk {

    /// # Description:
    /// Takes in a ```Block``` struct and fills and entire chunk with said ```Block```
    pub fn new_simple(block: blocks::Block) -> Self {
        Chunk { blocks: [[[block; CHUNK_SIZE_HORIZONTAL]; CHUNK_HEIGHT]; CHUNK_SIZE_HORIZONTAL] }
    }

    /// # Description:
    /// Does the same as ```new_simple(block: blocks::Block::new(BlockType::Air, Facing::XPositive))```
    pub fn new_simple_stone() -> Self {
        Chunk { blocks: [[[Block::new(BlockType::Stone, blocks::Facing::XPositive); CHUNK_SIZE_HORIZONTAL]; CHUNK_HEIGHT]; CHUNK_SIZE_HORIZONTAL] }
    }
}

impl ChunkComp {
    
    /// # Description:
    /// Creates a simple ```Chunk``` filled to the brim with ```BlockType::Stone```. It also includes an empty ```ChunkMeshBuilder``` for further processing
    pub fn new_simple() -> Self {        
        ChunkComp {
            chunk: Chunk::new_simple_stone(),
            chunk_mesh: chunk_mesh_builder::ChunkMeshBuilder::new()
        }
    }

    /// # Description:
    /// Builds the mesh based on the ```Chunk``` data, ignoring any ```Blocks``` with ```Transparency::Opaque```.
    /// 
    /// This funcion uses basic culling to reduce the poly count.
    pub fn build_mesh_culling(mut self) -> Mesh {
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
                }
            }
        }

        self.chunk_mesh.build()
    }

    /// # Description:
    /// Builds the mesh based on the ```Chunk``` data, ignoring any ```Blocks``` with ```Transparency::Opaque```.
    /// 
    /// This funcion uses basic culling to reduce the poly count.
    pub fn build_mesh_base(mut self) -> Mesh {
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

                    for i in 0..5 {
                        self.chunk_mesh.add_face(coord, i);
                    }
                }
            }
        }

        self.chunk_mesh.build()
    }
}