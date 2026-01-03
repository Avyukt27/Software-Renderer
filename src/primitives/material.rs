use crate::primitives::{colour::Colour, texture::Texture};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Material {
    pub name: String,
    pub ambient: Colour,
    pub texture: Option<Texture>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: String::from(""),
            ambient: Colour::new(255, 255, 255, 255),
            texture: None,
        }
    }
}
