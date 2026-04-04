pub struct Camera {
    position: glam::Vec3,
    target: glam::Vec3,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            position: glam::Vec3::new(0.0, 0.0, 3.0),
            target: glam::Vec3::ZERO,
            fov: 90.0_f32.to_radians(),
            aspect_ratio: size.0 as f32 / size.1 as f32,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        self.aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
    }

    pub fn view_proj(&self) -> glam::Mat4 {
        let view = glam::Mat4::look_at_rh(self.position, self.target, glam::Vec3::Y);
        let projection =
            glam::Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far);
        projection * view
    }
}
