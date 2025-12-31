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

    let mut vertex_map: HashMap<(usize, usize, usize), usize> = HashMap::new();

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
                    return Err("Invalid vertex");
                }
                positions.push(Vec3 {
                    x: words[1].parse().map_err(|_| "Invalid v")?,
                    y: words[2].parse().map_err(|_| "Invalid v")?,
                    z: words[3].parse().map_err(|_| "Invalid v")?,
                });
            }

            "vt" => {
                if words.len() < 3 {
                    return Err("Invalid vt");
                }
                let u: f64 = words[1].parse().map_err(|_| "Invalid vt")?;
                let v: f64 = words[2].parse().map_err(|_| "Invalid vt")?;

                uvs.push(Vec2 { x: u, y: 1.0 - v });
            }

            "vn" => {
                if words.len() != 4 {
                    return Err("Invalid vn");
                }
                normals.push(Vec3 {
                    x: words[1].parse().map_err(|_| "Invalid vn")?,
                    y: words[2].parse().map_err(|_| "Invalid vn")?,
                    z: words[3].parse().map_err(|_| "Invalid vn")?,
                });
            }

            "f" => {
                if words.len() != 4 {
                    return Err("Mesh must be triangulated");
                }

                let mut face = [0usize; 3];

                for i in 0..3 {
                    let parts: Vec<&str> = words[i + 1].split('/').collect();
                    if parts.len() != 3 {
                        return Err("Faces must be v/vt/vn");
                    }

                    let pi = parts[0].parse::<usize>().map_err(|_| "Bad index")? - 1;
                    let ti = parts[1].parse::<usize>().map_err(|_| "Bad index")? - 1;
                    let ni = parts[2].parse::<usize>().map_err(|_| "Bad index")? - 1;

                    let key = (pi, ti, ni);

                    let index = if let Some(&idx) = vertex_map.get(&key) {
                        idx
                    } else {
                        let p = positions[pi];
                        let uv = uvs[ti];

                        let vtx = Vertex::new(p.x, p.y, p.z, uv.x, uv.y);
                        let idx = vertices.len();
                        vertices.push(vtx);
                        vertex_map.insert(key, idx);
                        idx
                    };

                    face[i] = index;
                }

                triangles.push(Triangle::new(face[0], face[1], face[2]));
            }

            _ => {}
        }
    }

    Ok(Mesh {
        vertices,
        triangles,
        centre: Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0),
        rotate_around_pivot: false,
        pivot: None,
        texture: None,
    })
}
