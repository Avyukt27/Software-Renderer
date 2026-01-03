use crate::primitives::{colour::Colour, texture::Texture};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Material {
    pub name: String,
    pub diffuse: Colour,
    pub texture: Option<Texture>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: String::from(""),
            diffuse: Colour::new(255, 255, 255, 255),
            texture: None,
        }
    }
}
