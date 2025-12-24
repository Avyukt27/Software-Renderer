#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub i0: usize,
    pub i1: usize,
    pub i2: usize,
}

impl Triangle {
    pub fn new(i0: usize, i1: usize, v2: usize) -> Self {
        Self { i0, i1, i2: v2 }
    }
}
