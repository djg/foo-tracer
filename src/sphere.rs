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
        let co = self.centre - r.point;
        let nb = dot(co, r.direction);
        let c = dot(co, co) - self.radius * self.radius;
        let discriminant = nb * nb - c;

        if discriminant > 0. {
            let d = discriminant.sqrt();

            // Try earlier t
            let mut t = nb - d;
            if t < t_min {
                t = nb + d;
            }

            if t > t_min && t < t_max {
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

    fn bbox(&self) -> Option<Aabb> {
        let rrr = Vec3(self.radius, self.radius, self.radius);
        Some(Aabb {
            min: self.centre - rrr,
            max: self.centre + rrr,
        })
    }
}
