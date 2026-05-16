use crate::vertex::Vertex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct ObjIndex {
    v_idx: usize,
    vt_idx: usize,
    vn_idx: usize,
}

pub fn load_obj(path: &str) -> (Vec<Vertex>, Vec<u16>) {
    let file = File::open(path).expect("Failed to open OBJ file");
    let reader = BufReader::new(file);

    let mut raw_positions = Vec::new();
    let mut raw_normals = Vec::new();
    let mut _raw_uvs = Vec::new();
    let mut faces = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "v" => {
                let x: f32 = tokens[1].parse().unwrap();
                let y: f32 = tokens[2].parse().unwrap();
                let z: f32 = tokens[3].parse().unwrap();
                raw_positions.push(glam::Vec3::new(x, y, z));
            }
            "vn" => {
                let x: f32 = tokens[1].parse().unwrap();
                let y: f32 = tokens[2].parse().unwrap();
                let z: f32 = tokens[3].parse().unwrap();
                raw_normals.push(glam::Vec3::new(x, y, z));
            }
            "vt" => {
                let u: f32 = tokens[1].parse().unwrap();
                let v: f32 = tokens[2].parse().unwrap();
                _raw_uvs.push(glam::Vec2::new(u, v));
            }
            "f" => {
                for i in 1..=3 {
                    let parts: Vec<&str> = tokens[i].split('/').collect();

                    let v_idx = parts[0].parse::<usize>().unwrap() - 1;
                    let vt_idx = parts[1].parse::<usize>().unwrap() - 1;
                    let vn_idx = parts[2].parse::<usize>().unwrap() - 1;

                    faces.push(ObjIndex {
                        v_idx,
                        vt_idx,
                        vn_idx,
                    });
                }
            }
            _ => {}
        }
    }

    let mut out_vertices = Vec::new();
    let mut out_indices = Vec::new();

    let mut vertex_cache = std::collections::HashMap::new();

    let normal_to_color = |n: glam::Vec3| -> [f32; 4] {
        let c = n * 0.5 + glam::Vec3::splat(0.5);
        [c.x, c.y, c.z, 1.0]
    };

    for corner in faces {
        let cache_key = (corner.v_idx, corner.vn_idx);

        if let Some(&existing_index) = vertex_cache.get(&cache_key) {
            out_indices.push(existing_index);
        } else {
            let position = raw_positions[corner.v_idx];
            let normal = raw_normals[corner.vn_idx];

            let vertex = Vertex {
                position: position.to_array(),
                colour: normal_to_color(normal),
            };

            let new_index = out_vertices.len() as u16;
            out_vertices.push(vertex);
            out_indices.push(new_index);
            vertex_cache.insert(cache_key, new_index);
        }
    }

    (out_vertices, out_indices)
}
