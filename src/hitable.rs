use super::*;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
}

/*
pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: &mut f32) -> bool;
    fn hit_record(&self, r: &Ray, t: f32, hit: &mut HitRecord);
    fn bbox(&self) -> Option<Aabb>;
    fn material(&self) -> &Material;
}
*/

pub enum Hitable {
    Sphere { s: Sphere },
}

impl Hitable {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: &mut f32) -> bool {
        match self {
            Hitable::Sphere { s } => s.hit(r, t_min, t_max),
        }
    }

    pub fn hit_record(&self, r: &Ray, t: f32, hit: &mut HitRecord) {
        match self {
            Hitable::Sphere { s } => s.hit_record(r, t, hit),
        }
    }

    pub fn bbox(&self) -> Option<Aabb> {
        match self {
            Hitable::Sphere { s } => s.bbox(),
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Hitable::Sphere { s } => s.material(),
        }
    }
}

pub fn sphere(centre: Vec3, radius: f32, material: Box<Material>) -> Hitable {
    Hitable::Sphere {
        s: Sphere::new(centre, radius, material),
    }
}
