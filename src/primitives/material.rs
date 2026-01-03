use crate::primitives::{colour::Colour, texture::Texture};

#[derive(Debug)]
pub struct Material {
    pub ambient: Colour,
    pub texture: Option<Texture>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Colour::new(255, 255, 255, 255),
            texture: None,
        }
    }
}
