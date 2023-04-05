pub const VERTICES: [[[f32; 3]; 4]; 6] = [
    // Top
    [[0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0]],
    // Front
    [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
    // Left
    [[0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
    // Right
    [[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0]],
    // Back
    [[1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]],
    // Bottom
    [[0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 1.0], [1.0, 0.0, 0.0]]
];

pub const TRIANGLES: [u32; 6] = [0, 1, 2, 2, 1, 3];

pub const NORMALS: [[f32; 3]; 6] = [
    // Top
    [0.0, 1.0, 0.0],
    // Front
    [0.0, 0.0, -1.0],
    // Left
    [-1.0, 0.0, 0.0],
    // Right
    [1.0, 0.0, 0.0],
    // Back
    [0.0, 0.0, 1.0],
    // Bottom
    [0.0, -1.0, 0.0]
];

pub const UVS: [[f32; 2]; 4] = [
    [0.0, 0.0],
    [0.0, 1.0],
    [1.0, 0.0],
    [1.0, 1.0]
];