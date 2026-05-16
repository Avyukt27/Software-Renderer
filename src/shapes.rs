use crate::vertex::Vertex;

pub fn cube() -> (Vec<Vertex>, Vec<u16>) {
    let red = [1.0, 0.0, 0.0, 1.0];
    let green = [0.0, 1.0, 0.0, 1.0];
    let blue = [0.0, 0.0, 1.0, 1.0];
    let yellow = [1.0, 1.0, 0.0, 1.0];
    let cyan = [0.0, 1.0, 1.0, 1.0];
    let magenta = [1.0, 0.0, 1.0, 1.0];

    let vertices = vec![
        Vertex {
            position: [0.5, 0.5, 0.5],
            colour: red,
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            colour: red,
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            colour: red,
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            colour: red,
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            colour: green,
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            colour: green,
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            colour: green,
        },
        Vertex {
            position: [-0.5, -0.5, -0.5],
            colour: green,
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            colour: blue,
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            colour: blue,
        },
        Vertex {
            position: [-0.5, -0.5, -0.5],
            colour: blue,
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            colour: blue,
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            colour: yellow,
        },
        Vertex {
            position: [0.5, 0.5, 0.5],
            colour: yellow,
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            colour: yellow,
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            colour: yellow,
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            colour: cyan,
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            colour: cyan,
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            colour: cyan,
        },
        Vertex {
            position: [0.5, 0.5, 0.5],
            colour: cyan,
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            colour: magenta,
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            colour: magenta,
        },
        Vertex {
            position: [-0.5, -0.5, -0.5],
            colour: magenta,
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            colour: magenta,
        },
    ];

    let indices = vec![
        0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 17,
        18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
    ];

    (vertices, indices)
}
