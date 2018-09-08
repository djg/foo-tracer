use super::*;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
    pub mat: &'a dyn Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
