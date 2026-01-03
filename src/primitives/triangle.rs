#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub i0: usize,
    pub i1: usize,
    pub i2: usize,
    pub material_index: usize,
}

impl Triangle {
    pub fn new(i0: usize, i1: usize, i2: usize, material_index: usize) -> Self {
        Self {
            i0,
            i1,
            i2,
            material_index,
        }
    }
}
