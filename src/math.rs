use crate::primitives::vertex::Vertex;

pub fn rotate_x(vertex: &Vertex, angle: f32) -> Vertex {
    let sin = angle.sin() as f64;
    let cos = angle.cos() as f64;

    Vertex::new(
        vertex.x,
        vertex.y * cos - vertex.z * sin,
        vertex.y * sin + vertex.z * cos,
        vertex.u,
        vertex.v,
    )
}

pub fn rotate_y(vertex: &Vertex, angle: f32) -> Vertex {
    let sin = angle.sin() as f64;
    let cos = angle.cos() as f64;

    Vertex::new(
        vertex.x * cos + vertex.z * sin,
        vertex.y,
        -vertex.x * sin + vertex.z * cos,
        vertex.u,
        vertex.v,
    )
}

pub fn rotate_z(vertex: &Vertex, angle: f32) -> Vertex {
    let sin = angle.sin() as f64;
    let cos = angle.cos() as f64;

    Vertex::new(
        vertex.x * cos - vertex.y * sin,
        vertex.x * sin + vertex.y * cos,
        vertex.z,
        vertex.u,
        vertex.v,
    )
}

pub fn rotate_vertex(vertex: &Vertex, rotation: (f32, f32, f32)) -> Vertex {
    let mut rotated = *vertex;

    rotated = rotate_x(&rotated, rotation.0);
    rotated = rotate_y(&rotated, rotation.1);
    rotated = rotate_z(&rotated, rotation.2);

    rotated
}

pub fn rotate_around_pivot(vertex: &Vertex, pivot: &Vertex, rotation: (f32, f32, f32)) -> Vertex {
    let mut v = Vertex::new(
        vertex.x - pivot.x,
        vertex.y - pivot.y,
        vertex.z - pivot.z,
        vertex.u,
        vertex.v,
    );

    v = rotate_vertex(&v, rotation);

    Vertex::new(v.x + pivot.x, v.y + pivot.y, v.z + pivot.z, v.u, v.v)
}

pub fn is_back_facing(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> bool {
    let ax = v1.x - v0.x;
    let ay = v1.y - v0.y;
    let az = v1.z - v0.z;

    let bx = v2.x - v0.x;
    let by = v2.y - v0.y;
    let bz = v2.z - v0.z;

    let _nx = ay * bz - az * by;
    let _ny = az * bx - ax * bz;
    let nz = ax * by - ay * bx;

    nz <= 0.0
}
