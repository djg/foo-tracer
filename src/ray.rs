use vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub point: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.point + t * self.direction
    }
}
