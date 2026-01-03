use std::{collections::HashMap, fs::read_to_string, path::Path};

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

pub fn load_wavefront(path: &Path) -> Result<Mesh, &str> {
    let mut positions: Vec<Vec3> = Vec::new();
    let mut uvs: Vec<Vec2> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut materials: Vec<Material> = Vec::new();

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();

    let mut vertex_map: HashMap<(usize, usize, usize), usize> = HashMap::new();

    let mut current_material = Material::default();

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
                positions.push(Vec3 {
                    x: words[1].parse().map_err(|_| "Invalid vertex")?,
                    y: words[2].parse().map_err(|_| "Invalid vertex")?,
                    z: words[3].parse().map_err(|_| "Invalid vertex")?,
                });
            }
            "vt" => {
                let u: f64 = words[1].parse().map_err(|_| "Invalid vt")?;
                let v: f64 = words[2].parse().map_err(|_| "Invalid vt")?;
                uvs.push(Vec2 { x: u, y: 1.0 - v });
            }
            "vn" => {
                normals.push(Vec3 {
                    x: words[1].parse().map_err(|_| "Invalid vn")?,
                    y: words[2].parse().map_err(|_| "Invalid vn")?,
                    z: words[3].parse().map_err(|_| "Invalid vn")?,
                });
            }
            "f" => {
                if words.len() != 4 {
                    return Err("Faces must be triangulated");
                }

                let material_index: usize;

                match materials.iter().position(|m| m == &current_material) {
                    Some(i) => material_index = i,
                    None => return Err("Invalid material"),
                }

                let mut face_indices = [0usize; 3];

                for i in 0..3 {
                    let parts: Vec<&str> = words[i + 1].split('/').collect();
                    if parts.len() != 3 {
                        return Err("Faces must be v/vt/vn");
                    }

                    let pi = parts[0]
                        .parse::<usize>()
                        .map_err(|_| "Invalid position index")?
                        - 1;
                    let ti = parts[1].parse::<usize>().map_err(|_| "Invalid uv index")? - 1;
                    let ni = parts[2]
                        .parse::<usize>()
                        .map_err(|_| "Invalid normal index")?
                        - 1;

                    let key = (pi, ti, ni);

                    let vertex_index = if let Some(&idx) = vertex_map.get(&key) {
                        idx
                    } else {
                        let p = positions[pi];
                        let uv = uvs[ti];
                        let vertex = Vertex::new(p.x, p.y, p.z, uv.x, uv.y);
                        let idx = vertices.len();
                        vertices.push(vertex);
                        vertex_map.insert(key, idx);
                        idx
                    };

                    face_indices[i] = vertex_index;
                }

                triangles.push(Triangle::new(
                    face_indices[0],
                    face_indices[1],
                    face_indices[2],
                    material_index,
                ));
            }
            "mtllib" => {
                if words.len() != 2 {
                    return Err("Invalid mtl loading");
                }

                let parent = path.parent();
                match parent {
                    Some(p) => {
                        materials.extend(
                            load_materials(&p.join(Path::new(words[1])))
                                .expect("Issue reading mtl file"),
                        );
                    }
                    None => return Err("Invalid mtl path"),
                }
            }
            "usemtl" => {
                if words.len() != 2 {
                    return Err("Invalid mtl usage");
                }

                match materials.iter().find(|&material| material.name == words[1]) {
                    Some(m) => current_material = m.clone(),
                    None => return Err("Invalid material name"),
                }
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
        materials: materials,
    })
}

pub fn load_materials(path: &Path) -> Result<Vec<Material>, &str> {
    let mut materials: Vec<Material> = Vec::new();
    let mut material = Material::default();

    for line in read_to_string(path)
        .map_err(|_| "Failed to read MTL")?
        .lines()
    {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() == 0 {
            continue;
        }

        match words[0] {
            "newmtl" => {
                if words.len() != 2 {
                    return Err("Invalid newmtl");
                }

                if materials.len() != 0 {
                    materials.push(material.clone());
                    material = Material::default();
                }

                material.name = String::from(words[1]);
            }

            "Kd" => {
                if material == Material::default() {
                    return Err("Invalid MTL file");
                }
                if words.len() != 4 {
                    return Err("Invalid diffuse colour");
                }

                let red: f64 = words[1].parse().map_err(|_| "Invalid colour")?;
                let green: f64 = words[2].parse().map_err(|_| "Invalid colour")?;
                let blue: f64 = words[3].parse().map_err(|_| "Invalid colour")?;

                material.diffuse = Colour::new(
                    (red.clamp(0.0, 1.0) * 255.0).round() as u8,
                    (green.clamp(0.0, 1.0) * 255.0).round() as u8,
                    (blue.clamp(0.0, 1.0) * 255.0).round() as u8,
                    255,
                );
            }

            "d" => {
                if material == Material::default() {
                    return Err("Invalid MTL file");
                }
                if words.len() != 2 {
                    return Err("Invalid dissolve");
                }

                let alpha: f64 = words[1].parse().map_err(|_| "Invalid dissolve")?;

                material.diffuse = Colour::new(
                    material.diffuse.red,
                    material.diffuse.green,
                    material.diffuse.blue,
                    (alpha.clamp(0.0, 1.0) * 255.0).round() as u8,
                );
            }

            "map_Kd" => {
                if material == Material::default() {
                    return Err("Invalid MTL file");
                }
                if words.len() != 2 {
                    return Err("Invalid texture");
                }

                material.texture =
                    Some(Texture::from_file(words[1]).map_err(|_| "Invalid texture")?);
            }

            _ => {}
        }
    }

    materials.push(material);

    Ok(materials)
}
