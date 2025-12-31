use std::{collections::HashMap, fs::read_to_string};

use crate::{
    mesh::Mesh,
    primitives::{
        vector::{Vec2, Vec3},
        vertex::Vertex,
    },
};

pub fn load_wavefront(path: &str) -> Result<Mesh, &str> {
    let mut positions: Vec<Vec3> = Vec::new();
    let mut uvs: Vec<Vec2> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut vertex_map: HashMap<(usize, usize, usize), u32> = HashMap::new();

    for line in read_to_string(path).unwrap().lines() {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.is_empty() || words[0].starts_with("#") {
            continue;
        }

        match words[0] {
            "v" => {
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
            "f" => {}
            _ => {}
        }
    }

    Ok(Mesh {
        vertices,
        triangles: Vec::new(),
        centre: Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0),
        rotate_around_pivot: false,
        pivot: None,
        texture: None,
    })
}
