use super::*;

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.point - self.centre;
        let a = dot(r.direction, r.direction);
        let b = dot(oc, r.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let discriminant = f32::sqrt(discriminant);
            let temp = (-b - discriminant) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let point = r.point_at(t);
                let normal = (point - self.centre) / self.radius;
                let material = &*self.material;
                return Some(HitRecord {
                    t,
                    point,
                    normal,
                    material,
                });
            }
            let temp = (-b + discriminant) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let point = r.point_at(t);
                let normal = (point - self.centre) / self.radius;
                let material = &*self.material;
                return Some(HitRecord {
                    t,
                    point,
                    normal,
                    material,
                });
            }
        }
        None
    }
}
