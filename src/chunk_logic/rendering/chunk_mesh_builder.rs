// foreign imports
use bevy::prelude::Mesh;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::mesh::Indices;

// 'self' imports
// #[path ="./rendering_const.rs"]
// mod rendering_const;
use crate::chunk_logic::rendering::rendering_const;

/// # Description:
/// The struct that contains all the necessary info to render the Mesh to the screen
/// # Structure:
/// ```
/// pub struct ChunkMeshBuilder {
///     vertices: Vec<[f32; 3]>,
///     triangles: Vec<u32>,
///     normals: Vec<[f32; 3]>,
///     uvs: Vec<[f32; 2]>,
///     face_count:u32
/// }
/// ```
#[derive(Default)]
pub struct ChunkMeshBuilder {
    vertices: Vec<[f32; 3]>,
    triangles: Vec<u32>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    face_count:u32
}

impl ChunkMeshBuilder {
    
    /// # Description:
    /// Builds an empty mesh based on the ```Self::default()``` function
    pub fn new() -> Self {
        Self::default()
    }

    /// # Description:
    /// Makes the additon of the two arrays far easier to work with
    fn add_vec3(mut base: [f32; 3], addition: [u32; 3]) -> [f32; 3] {
        for i in 0..3 {
            base[i] += addition[i] as f32;
        }
        base
    }

    /// # Description:
    /// Adds a new face of the entity to the ```Mesh```
    pub fn add_face(&mut self, coord: [u32; 3], face_index: u8) {
        for i in &rendering_const::VERTICES[face_index as usize] {
            self.vertices.push(Self::add_vec3(*i, coord));
        }

        let mut arr=rendering_const::TRIANGLES.clone();
        self.triangles.extend_from_slice({
            for i in &mut arr {
                *i+=4*self.face_count;
            }
            &arr
        });

        for _ in 0..4 {
            self.normals.push(rendering_const::NORMALS[face_index as usize]);
        }

        self.uvs.extend_from_slice(&rendering_const::UVS);
        self.face_count+=1;
    }

    /// # Description:
    /// Fills the rest of the ```Mesh``` based on available data. This makes the ```Mesh``` usable by bevy
    pub fn build(self) -> Mesh {
        let mut msh=Mesh::new(PrimitiveTopology::TriangleList);
        msh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        msh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        msh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);

        msh.set_indices(Some(Indices::U32(self.triangles)));
        msh
    }
}