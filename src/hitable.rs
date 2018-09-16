use super::*;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: &mut f32) -> bool;
    fn hit_record(&self, r: &Ray, t: f32, hit: &mut HitRecord);
    fn bbox(&self) -> Option<Aabb>;
    fn material(&self) -> &Material;
}
