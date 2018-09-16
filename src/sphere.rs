use super::*;

use std::f32;

pub struct Sphere {
    pub centre: Vec3,
    pub radius_sq: f32,
    pub material: Box<Material>,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: Box<Material>) -> Self {
        Sphere {
            centre,
            radius_sq: radius * radius,
            material,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: &mut f32) -> bool {
        let co = self.centre - r.point;
        let nb = dot(co, r.direction);
        let c = dot(co, co) - self.radius_sq;
        let discriminant = nb * nb - c;

        if discriminant > 0. {
            let d = discriminant.sqrt();

            // Try earlier t
            let mut t = nb - d;
            if t < t_min {
                t = nb + d;
            }

            if t > t_min && t < *t_max {
                *t_max = t;
                return true;
            }
        }
        false
    }

    pub fn hit_record(&self, r: &Ray, t: f32, hit: &mut HitRecord) {
        hit.point = r.point_at(t);
        hit.normal = normalized(hit.point - self.centre);
    }

    pub fn bbox(&self) -> Option<Aabb> {
        let r = self.radius_sq.sqrt();
        Some(Aabb {
            min: self.centre - Vec3(r, r, r),
            max: self.centre + Vec3(r, r, r),
        })
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}
