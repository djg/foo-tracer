use vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray(pub Vec3, pub Vec3);

impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.0
    }
    pub fn direction(&self) -> Vec3 {
        self.1
    }
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.0 + t * self.1
    }
}
