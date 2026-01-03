use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::{
    mesh::Mesh,
    primitives::{
        colour::Colour,
        material::Material,
        texture::Texture,
        triangle::Triangle,
        vector::{Vec2, Vec3},
        vertex::Vertex,
    },
};

pub fn load_wavefront(path: &Path) -> Result<Mesh, String> {
    let mut positions: Vec<Vec3> = Vec::new();
    let mut uvs: Vec<Vec2> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut vertex_map: HashMap<(usize, usize, usize), usize> = HashMap::new();

    // ---- Materials ----
    let mut materials: Vec<Material> = Vec::new();
    let mut material_lookup: HashMap<String, usize> = HashMap::new();

    // Default material (index 0)
    materials.push(Material {
        name: "__default".to_string(),
        diffuse: Colour::new(255, 255, 255, 255),
        kd_texture: None,
    });
    material_lookup.insert("__default".to_string(), 0);

    let mut current_material_index: usize = 0;

    // ---- OBJ parsing ----
    for line in read_to_string(path)
        .map_err(|_| "Failed to read OBJ")?
        .lines()
    {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() || words[0].starts_with('#') {
            continue;
        }

        match words[0] {
            "v" => {
                if words.len() != 4 {
                    return Err(String::from("Invalid vertex"));
                }
                positions.push(Vec3 {
                    x: words[1].parse().map_err(|_| "Invalid vertex")?,
                    y: words[2].parse().map_err(|_| "Invalid vertex")?,
                    z: words[3].parse().map_err(|_| "Invalid vertex")?,
                });
            }

            "vt" => {
                if words.len() < 3 {
                    return Err(String::from("Invalid vt"));
                }
                let u: f64 = words[1].parse().map_err(|_| "Invalid vt")?;
                let v: f64 = words[2].parse().map_err(|_| "Invalid vt")?;
                uvs.push(Vec2 { x: u, y: 1.0 - v });
            }

            "vn" => {
                if words.len() != 4 {
                    return Err(String::from("Invalid vn"));
                }
                normals.push(Vec3 {
                    x: words[1].parse().map_err(|_| "Invalid vn")?,
                    y: words[2].parse().map_err(|_| "Invalid vn")?,
                    z: words[3].parse().map_err(|_| "Invalid vn")?,
                });
            }

            "f" => {
                if words.len() != 4 {
                    return Err(String::from("Faces must be triangulated"));
                }

                let mut face_indices = [0usize; 3];

                for i in 0..3 {
                    let parts: Vec<&str> = words[i + 1].split('/').collect();
                    if parts.len() != 3 {
                        return Err(String::from("Faces must be v/vt/vn"));
                    }

                    let pi = parts[0].parse::<usize>().map_err(|_| "Invalid index")? - 1;
                    let ti = parts[1].parse::<usize>().map_err(|_| "Invalid index")? - 1;
                    let ni = parts[2].parse::<usize>().map_err(|_| "Invalid index")? - 1;

                    let key = (pi, ti, ni);

                    let index = if let Some(&idx) = vertex_map.get(&key) {
                        idx
                    } else {
                        let p = positions[pi];
                        let uv = uvs[ti];
                        let v = Vertex::new(p.x, p.y, p.z, uv.x, uv.y);
                        let idx = vertices.len();
                        vertices.push(v);
                        vertex_map.insert(key, idx);
                        idx
                    };

                    face_indices[i] = index;
                }

                triangles.push(Triangle::new(
                    face_indices[0],
                    face_indices[1],
                    face_indices[2],
                    current_material_index,
                ));
            }

            "mtllib" => {
                if words.len() != 2 {
                    return Err(String::from("Invalid mtllib"));
                }

                let parent = path.parent().ok_or("Invalid OBJ path")?;
                let mtl_path = parent.join(words[1]);

                let loaded = load_materials(mtl_path)?;

                for material in loaded {
                    let index = materials.len();
                    material_lookup.insert(material.name.clone(), index);
                    materials.push(material);
                }
            }

            "usemtl" => {
                if words.len() != 2 {
                    return Err(String::from("Invalid usemtl"));
                }

                current_material_index =
                    *material_lookup.get(words[1]).ok_or("Material not found")?;
            }

            _ => {}
        }
    }

    Ok(Mesh {
        vertices,
        triangles,
        centre: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotate_around_pivot: false,
        pivot: None,
        materials,
    })
}

pub fn load_materials(path: PathBuf) -> Result<Vec<Material>, String> {
    let mut materials: Vec<Material> = Vec::new();
    let mut current: Option<Material> = None;

    let base_dir: PathBuf = path.parent().unwrap_or(Path::new("")).to_path_buf();

    for line in read_to_string(path)
        .map_err(|_| "Failed to read MTL")?
        .lines()
    {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() || words[0].starts_with('#') {
            continue;
        }

        match words[0] {
            "newmtl" => {
                if words.len() != 2 {
                    return Err(String::from("Invalid newmtl"));
                }

                if let Some(mat) = current.take() {
                    materials.push(mat);
                }

                current = Some(Material {
                    name: words[1].to_string(),
                    diffuse: Colour::new(255, 255, 255, 255),
                    kd_texture: None,
                });
            }

            "Kd" => {
                let mat = current.as_mut().ok_or("Kd before newmtl")?;
                let r: f64 = words[1].parse().map_err(|_| "Invalid Kd")?;
                let g: f64 = words[2].parse().map_err(|_| "Invalid Kd")?;
                let b: f64 = words[3].parse().map_err(|_| "Invalid Kd")?;

                mat.diffuse = Colour::new(
                    (r * 255.0).round() as u8,
                    (g * 255.0).round() as u8,
                    (b * 255.0).round() as u8,
                    mat.diffuse.alpha,
                );
            }

            "d" => {
                let mat = current.as_mut().ok_or("d before newmtl")?;
                let a: f64 = words[1].parse().map_err(|_| "Invalid d")?;
                mat.diffuse.alpha = (a * 255.0).round() as u8;
            }

            "map_Kd" => {
                let mat = current.as_mut().ok_or("map_Kd before newmtl")?;
                let tex_path = base_dir.join(words[1]);
                mat.kd_texture =
                    Some(Texture::from_file(&tex_path).map_err(|_| "Invalid texture")?);
            }

            _ => {}
        }
    }

    if let Some(mat) = current {
        materials.push(mat);
    }

    Ok(materials)
}
