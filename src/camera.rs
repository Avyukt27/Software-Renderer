pub struct Camera {
    position: glam::Vec3,
    yaw: f32,
    pitch: f32,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            position: glam::Vec3::new(0.0, 0.0, 3.0),
            yaw: -90.0_f32.to_radians(),
            pitch: 0.0,
            fov: 90.0_f32.to_radians(),
            aspect_ratio: size.0 as f32 / size.1 as f32,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        self.aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
    }

    pub fn get_forawrd(&self) -> glam::Vec3 {
        let x = self.yaw.cos() * self.pitch.cos();
        let y = self.pitch.sin();
        let z = self.yaw.sin() * self.pitch.cos();
        glam::Vec3::new(x, y, z).normalize()
    }

    pub fn view_proj(&self) -> glam::Mat4 {
        let forward = self.get_forawrd();
        let target = self.position + forward;

        let view = glam::Mat4::look_at_rh(self.position, target, glam::Vec3::Y);
        let projection =
            glam::Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far);
        projection * view
    }

    pub fn rotate_view(&mut self, delta: (f64, f64)) {
        let sensitivity = 0.002_f32;
        self.yaw += (delta.0 as f32) * sensitivity;
        self.pitch -= (delta.1 as f32) * sensitivity;
        self.pitch = self
            .pitch
            .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
    }

    pub fn move_forward(&mut self, speed: f32) {
        let forward = self.get_forawrd();
        self.position += forward * speed;
    }
    pub fn move_strafe(&mut self, speed: f32) {
        let forward = self.get_forawrd();
        let right = forward.cross(glam::Vec3::Y).normalize();
        self.position += right * speed;
    }
    pub fn move_up(&mut self, speed: f32) {
        let forward = self.get_forawrd();
        let right = forward.cross(glam::Vec3::Y).normalize();
        let up = right.cross(forward).normalize();
        self.position += up * speed;
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    matrix: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn from_camera(camera: &Camera) -> Self {
        Self {
            matrix: camera.view_proj().to_cols_array_2d(),
        }
    }
}
