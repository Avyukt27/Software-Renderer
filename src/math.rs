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

pub fn rotate_vertex(vertex: &Vertex, rotation: (f32, f32, f32)) -> Vertex {
    let mut rotated = *vertex;

    rotated = rotate_x(&rotated, rotation.0);
    rotated = rotate_y(&rotated, rotation.1);
    rotated = rotate_z(&rotated, rotation.2);

    rotated
}

pub fn rotate_around_pivot(vertex: &Vertex, pivot: &Vertex, rotation: (f32, f32, f32)) -> Vertex {
    let mut v = Vertex {
        x: vertex.x - pivot.x,
        y: vertex.y - pivot.y,
        z: vertex.z - pivot.z,
    };

    v = rotate_vertex(&v, rotation);

    Vertex {
        x: v.x + pivot.x,
        y: v.y + pivot.y,
        z: v.z + pivot.z,
    }
}
