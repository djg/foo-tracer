use super::*;

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3(1., 1., 1.);
    while p.squared_len() >= 1. {
        p = 2. * Vec3(random::<f32>(), random::<f32>(), random::<f32>()) - Vec3(1., 1., 1.);
    }
    p
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * dot(v, n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = normalized(v);
    let dt = dot(uv, n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - n * dt) - n * f32::sqrt(discriminant))
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * f32::powf(1. - cosine, 5.)
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit.point + hit.normal + random_in_unit_sphere();
        let scattered = Ray {
            point: hit.point,
            direction: target - hit.point,
        };
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        let fuzz = f32::min(fuzz, 1.);
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(normalized(r_in.direction), hit.normal);
        let scattered = Ray {
            point: hit.point,
            direction: reflected + self.fuzz * random_in_unit_sphere(),
        };
        let attenuation = self.albedo;
        if dot(scattered.direction, hit.normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3(1., 1., 1.);
        let (outward_normal, ni_over_nt, cosine) = if dot(r_in.direction, hit.normal) > 0. {
            (
                -hit.normal,
                self.ref_idx,
                self.ref_idx * dot(r_in.direction, hit.normal) / r_in.direction.len(),
            )
        } else {
            (
                hit.normal,
                1. / self.ref_idx,
                -dot(r_in.direction, hit.normal) / r_in.direction.len(),
            )
        };
        let (refracted, reflect_prob) =
            if let Some(refracted) = refract(r_in.direction, outward_normal, ni_over_nt) {
                (refracted, schlick(cosine, self.ref_idx))
            } else {
                (Vec3(0., 0., 0.), 1.)
            };
        let scattered = if random::<f32>() < reflect_prob {
            let reflected = reflect(r_in.direction, hit.normal);
            Ray {
                point: hit.point,
                direction: reflected,
            }
        } else {
            Ray {
                point: hit.point,
                direction: refracted,
            }
        };
        Some((attenuation, scattered))
    }
}
