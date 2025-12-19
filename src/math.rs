use crate::vertex::Vertex;

pub fn rotate_x(vertex: &Vertex, angle: f32) -> Vertex {
    let sin = angle.sin() as f64;
    let cos = angle.cos() as f64;

    Vertex {
        x: vertex.x,
        y: vertex.y * cos - vertex.z * sin,
        z: vertex.y * sin + vertex.z * cos,
    }
}

pub fn rotate_y(vertex: &Vertex, angle: f32) -> Vertex {
    let sin = angle.sin() as f64;
    let cos = angle.cos() as f64;

    Vertex {
        x: vertex.x * cos + vertex.z * sin,
        y: vertex.y,
        z: -vertex.x * sin + vertex.z * cos,
    }
}

pub fn rotate_z(vertex: &Vertex, angle: f32) -> Vertex {
    let sin = angle.sin() as f64;
    let cos = angle.cos() as f64;

    Vertex {
        x: vertex.x * cos - vertex.y * sin,
        y: vertex.x * sin + vertex.y * cos,
        z: vertex.z,
    }
}

pub fn rotate_vertex(vertex: &Vertex, angles: (f32, f32, f32)) -> Vertex {
    let centre = Vertex {
        x: 0.0,
        y: 0.0,
        z: 10.0,
    };

    let mut rotated = Vertex {
        x: vertex.x - centre.x,
        y: vertex.y - centre.y,
        z: vertex.z - centre.z,
    };

    rotated = rotate_x(&rotated, angles.0);
    rotated = rotate_y(&rotated, angles.1);
    rotated = rotate_z(&rotated, angles.2);
    Vertex {
        x: rotated.x + centre.x,
        y: rotated.y + centre.y,
        z: rotated.z + centre.z,
    }
}
