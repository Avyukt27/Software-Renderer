use crate::vertex::Vertex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
}

struct ObjIndex {
    v_idx: usize,
    vt_idx: usize,
    vn_idx: usize,
    mat_name: String,
}

fn load_mtl(path: &Path) -> HashMap<String, Material> {
    let mut materials = HashMap::new();
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            println!("Warning: Could not find material file at {:?}", path);
            return materials;
        }
    };

    let reader = BufReader::new(file);
    let mut current_mat: Option<Material> = None;

    for line in reader.lines() {
        let line = line.expect("Failed to read MTL line");
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "newmtl" => {
                if let Some(mat) = current_mat.take() {
                    materials.insert(mat.name.clone(), mat);
                }
                current_mat = Some(Material {
                    name: tokens[1].to_string(),
                    ambient: [1.0, 1.0, 1.0],
                    diffuse: [1.0, 1.0, 1.0],
                    specular: [0.0, 0.0, 0.0],
                    shininess: 32.0,
                });
            }
            "Ka" => {
                if let Some(ref mut mat) = current_mat {
                    mat.ambient = [
                        tokens[1].parse().unwrap(),
                        tokens[2].parse().unwrap(),
                        tokens[3].parse().unwrap(),
                    ];
                }
            }
            "Kd" => {
                if let Some(ref mut mat) = current_mat {
                    mat.diffuse = [
                        tokens[1].parse().unwrap(),
                        tokens[2].parse().unwrap(),
                        tokens[3].parse().unwrap(),
                    ];
                }
            }
            "Ks" => {
                if let Some(ref mut mat) = current_mat {
                    mat.specular = [
                        tokens[1].parse().unwrap(),
                        tokens[2].parse().unwrap(),
                        tokens[3].parse().unwrap(),
                    ];
                }
            }
            "Ns" => {
                if let Some(ref mut mat) = current_mat {
                    mat.shininess = tokens[1].parse().unwrap();
                }
            }
            _ => {}
        }
    }

    if let Some(mat) = current_mat {
        materials.insert(mat.name.clone(), mat);
    }

    materials
}

pub fn load_obj(path_str: &str) -> (Vec<Vertex>, Vec<u16>) {
    let obj_path = Path::new(path_str);
    let base_dir = obj_path.parent().unwrap_or_else(|| Path::new("."));

    let file = File::open(obj_path).expect("Failed to open OBJ file");
    let reader = BufReader::new(file);

    let mut raw_positions = Vec::new();
    let mut raw_normals = Vec::new();
    let mut raw_uvs = Vec::new();
    let mut faces = Vec::new();

    let mut materials: HashMap<String, Material> = HashMap::new();
    let mut current_material_name = String::from("Default");

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "mtllib" => materials.extend(load_mtl(&base_dir.join(tokens[1]))),
            "usemtl" => current_material_name = tokens[1].to_string(),
            "v" => raw_positions.push(glam::Vec3::new(
                tokens[1].parse().unwrap(),
                tokens[2].parse().unwrap(),
                tokens[3].parse().unwrap(),
            )),
            "vn" => raw_normals.push(glam::Vec3::new(
                tokens[1].parse().unwrap(),
                tokens[2].parse().unwrap(),
                tokens[3].parse().unwrap(),
            )),
            "vt" => raw_uvs.push(glam::Vec2::new(
                tokens[1].parse().unwrap(),
                1.0 - tokens[2].parse::<f32>().unwrap(),
            )),
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
                        mat_name: current_material_name.clone(),
                    });
                }
            }
            _ => {}
        }
    }

    let mut out_vertices = Vec::new();
    let mut out_indices = Vec::new();

    let mut vertex_cache = HashMap::new();

    for corner in faces {
        let cache_key = (
            corner.v_idx,
            corner.vt_idx,
            corner.vn_idx,
            corner.mat_name.clone(),
        );

        if let Some(&existing_index) = vertex_cache.get(&cache_key) {
            out_indices.push(existing_index);
        } else {
            let position = raw_positions[corner.v_idx];

            let uv = if corner.vt_idx < raw_uvs.len() {
                raw_uvs[corner.vt_idx].to_array()
            } else {
                [0.0, 0.0]
            };

            let diffuse_color = if let Some(mat) = materials.get(&corner.mat_name) {
                [mat.diffuse[0], mat.diffuse[1], mat.diffuse[2], 1.0]
            } else {
                [1.0, 1.0, 1.0, 1.0]
            };

            let vertex = Vertex {
                position: position.to_array(),
                colour: diffuse_color,
                uv,
            };

            let new_index = out_vertices.len() as u16;
            out_vertices.push(vertex);
            out_indices.push(new_index);
            vertex_cache.insert(cache_key, new_index);
        }
    }

    (out_vertices, out_indices)
}
