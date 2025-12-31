use std::{collections::HashMap, fs::read_to_string};

use crate::{
    mesh::Mesh,
    primitives::{
        triangle::Triangle,
        vector::{Vec2, Vec3},
        vertex::Vertex,
    },
};

pub fn load_wavefront(path: &str) -> Result<Mesh, &str> {
    let mut positions: Vec<Vec3> = Vec::new();
    let mut uvs: Vec<Vec2> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut vertex_map: HashMap<(usize, usize, usize), u32> = HashMap::new();

    for line in read_to_string(path).unwrap().lines() {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.is_empty() || words[0].starts_with("#") {
            continue;
        }

        match words[0] {
            "v" => {
                if words.len() != 4 {
                    return Err("Invalid vertex positions");
                }

                let x = match words[1].parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Invalid vertex position"),
                };
                let y = match words[2].parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Invalid vertex position"),
                };
                let z = match words[3].parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Invalid vertex position"),
                };
                positions.push(Vec3 { x: x, y: y, z: z });
            }
            "vn" => {
                if words.len() != 4 {
                    return Err("Invalid vertex normals");
                }

                let x = match words[1].parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Invalid vertex position"),
                };
                let y = match words[2].parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Invalid vertex position"),
                };
                let z = match words[3].parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Invalid vertex position"),
                };
                normals.push(Vec3 { x: x, y: y, z: z });
            }
            "vt" => {
                if words.len() < 3 {
                    return Err("Invalid texture coordinates");
                }

                let x = match words[1].parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Invalid vertex position"),
                };
                let y: f64 = match words[2].parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Invalid vertex position"),
                };
                uvs.push(Vec2 { x: x, y: 1.0 - y });
            }
            "f" => {
                if words.len() != 4 {
                    return Err("Faces are not triangulated");
                }

                let mut face_indices = [0usize; 3];

                for i in 0..3 {
                    let parts: Vec<&str> = words[i + 1].split('/').collect();
                    if parts.len() != 3 {
                        return Err("Faces must be v/vt/vn");
                    }

                    let position_index: usize =
                        parts[0].parse().map_err(|_| "Invalid position index")?;
                    let texture_index: usize =
                        parts[1].parse().map_err(|_| "Invalid texture index")?;
                    let normal_index: usize =
                        parts[2].parse().map_err(|_| "Invalid normal index")?;

                    let key = (position_index - 1, texture_index - 1, normal_index - 1);

                    let vertex_index = if let Some(&idx) = vertex_map.get(&key) {
                        idx
                    } else {
                        let position = positions[key.0];
                        let uv = uvs[key.1];
                        let n = normals[key.2];

                        let vertex = Vertex::new(position.x, position.y, position.z, uv.x, uv.y);

                        let idx = vertices.len();
                        vertices.push(vertex);
                        vertex_map.insert(key, idx as u32);
                        idx as u32
                    };

                    face_indices[i] = vertex_index as usize;
                }

                triangles.push(Triangle::new(
                    face_indices[0],
                    face_indices[1],
                    face_indices[2],
                ));
            }
            _ => {}
        }
    }

    let mut min = Vec3 {
        x: f64::INFINITY,
        y: f64::INFINITY,
        z: f64::INFINITY,
    };
    let mut max = Vec3 {
        x: f64::NEG_INFINITY,
        y: f64::NEG_INFINITY,
        z: f64::NEG_INFINITY,
    };

    for v in &vertices {
        min.x = min.x.min(v.x);
        min.y = min.y.min(v.y);
        min.z = min.z.min(v.z);

        max.x = max.x.max(v.x);
        max.y = max.y.max(v.y);
        max.z = max.z.max(v.z);
    }

    let centre = Vertex::new(
        (min.x + max.x) * 0.5,
        (min.y + max.y) * 0.5,
        (min.z + max.z) * 0.5,
        0.0,
        0.0,
    );

    for v in &mut vertices {
        v.x -= centre.x;
        v.y -= centre.y;
        v.z -= centre.z;
    }

    Ok(Mesh {
        vertices,
        triangles,
        centre,
        rotate_around_pivot: false,
        pivot: None,
        texture: None,
    })
}
