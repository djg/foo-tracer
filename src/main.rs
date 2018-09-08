extern crate indicatif;
extern crate rand;
extern crate rayon;

use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use std::{f32, fs::File, io::Write};

mod vec3;
use vec3::*;

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

#[derive(Clone, Copy, Debug)]
struct Ray(pub Vec3, pub Vec3);

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

struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
    pub mat: &'a dyn Material,
}

trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

struct Sphere {
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
        let oc = r.origin() - self.centre;
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let discriminant = f32::sqrt(discriminant);
            let temp = (-b - discriminant) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at(t);
                let n = (p - self.centre) / self.radius;
                let mat = &*self.material;
                return Some(HitRecord { t, p, n, mat });
            }
            let temp = (-b + discriminant) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at(t);
                let n = (p - self.centre) / self.radius;
                let mat = &*self.material;
                return Some(HitRecord { t, p, n, mat });
            }
        }
        None
    }
}

struct World {
    pub entities: Vec<Box<Hitable>>,
}

impl Hitable for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;
        for entity in &self.entities {
            if let Some(hit) = entity.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                result = Some(hit)
            }
        }
        result
    }
}

unsafe impl std::marker::Sync for World {}

struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let lens_radius = aperture / 2.;
        let theta = vfov.to_radians();
        let half_height = f32::tan(theta / 2.);
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = normalized(look_from - look_at);
        let u = normalized(cross(vup, w));
        let v = cross(w, u);
        let lower_left_corner = origin - focus_dist * (half_width * u + half_height * v + w);
        let horizontal = 2. * half_width * focus_dist * u;
        let vertical = 2. * half_height * focus_dist * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.0 + self.v * rd.1;
        Ray(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut p = Vec3(1., 1., 1.);
    while p.squared_len() >= 1. {
        p = 2. * Vec3(random::<f32>(), random::<f32>(), 0.) - Vec3(1., 1., 0.);
    }
    p
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3(1., 1., 1.);
    while p.squared_len() >= 1. {
        p = 2. * Vec3(random::<f32>(), random::<f32>(), random::<f32>()) - Vec3(1., 1., 1.);
    }
    p
}

trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)>;
}

struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit.p + hit.n + random_in_unit_sphere();
        let scattered = Ray(hit.p, target - hit.p);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

struct Metal {
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
        let reflected = reflect(normalized(r_in.direction()), hit.n);
        let scattered = Ray(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;
        if dot(scattered.direction(), hit.n) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }

    fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        let r0 = r0 * r0;
        r0 + (1. - r0) * f32::powf(1. - cosine, 5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3(1., 1., 1.);
        let (outward_normal, ni_over_nt, cosine) = if dot(r_in.direction(), hit.n) > 0. {
            (
                -hit.n,
                self.ref_idx,
                self.ref_idx * dot(r_in.direction(), hit.n) / r_in.direction().len(),
            )
        } else {
            (
                hit.n,
                1. / self.ref_idx,
                -dot(r_in.direction(), hit.n) / r_in.direction().len(),
            )
        };
        let (refracted, reflect_prob) =
            if let Some(refracted) = refract(r_in.direction(), outward_normal, ni_over_nt) {
                (refracted, Self::schlick(cosine, self.ref_idx))
            } else {
                (Vec3(0., 0., 0.), 1.)
            };
        let scattered = if random::<f32>() < reflect_prob {
            let reflected = reflect(r_in.direction(), hit.n);
            Ray(hit.p, reflected)
        } else {
            Ray(hit.p, refracted)
        };
        Some((attenuation, scattered))
    }
}

fn color(r: &Ray, world: &World, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit.mat.scatter(r, &hit) {
                return attenuation * color(&scattered, world, depth + 1);
            }
        }
        return Vec3(0., 0., 0.);
    } else {
        let unit_direction = normalized(r.direction());
        let t = 0.5 * (unit_direction.1 + 1.);
        (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
    }
}

const NX: i32 = 1280; //400;
const NY: i32 = 720; //200;
const NS: i32 = 100;

fn random_scene() -> World {
    let mut entities: Vec<Box<dyn Hitable>> = Vec::with_capacity(501);
    entities.push(Box::new(Sphere::new(
        Vec3(0., -1000., 0.),
        1000.,
        Box::new(Lambertian::new(Vec3(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let centre = Vec3(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );
            if (centre - Vec3(4., 0.2, 0.)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    entities.push(Box::new(Sphere::new(
                        centre,
                        0.2,
                        Box::new(Lambertian::new(Vec3(
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    entities.push(Box::new(Sphere::new(
                        centre,
                        0.2,
                        Box::new(Metal::new(
                            Vec3(
                                0.5 * (1. + random::<f32>()),
                                0.5 * (1. + random::<f32>()),
                                0.5 * (1. + random::<f32>()),
                            ),
                            0.5 * random::<f32>(),
                        )),
                    )));
                } else {
                    // glass
                    entities.push(Box::new(Sphere::new(
                        centre,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    entities.push(Box::new(Sphere::new(
        Vec3(0., 1., 0.),
        1.,
        Box::new(Dielectric::new(1.5)),
    )));
    entities.push(Box::new(Sphere::new(
        Vec3(-4., 1., 0.),
        1.,
        Box::new(Lambertian::new(Vec3(0.4, 0.2, 0.1))),
    )));
    entities.push(Box::new(Sphere::new(
        Vec3(4., 1., 0.),
        1.,
        Box::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.)),
    )));
    World { entities }
}

fn main() {
    let mut file = File::create("image.ppm").expect("Failed to open image.ppm");
    writeln!(file, "P3\n{} {}\n255", NX, NY);
    let world = random_scene();
    let look_from = Vec3(13., 2., 3.);
    let look_at = Vec3(0., 0., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.05;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        20.,
        NX as f32 / NY as f32,
        aperture,
        dist_to_focus,
    );

    let pb = ProgressBar::new(NX as u64 * NY as u64);
    pb.set_style(
        ProgressStyle::default_bar().template("{elapsed_precise} {wide_bar} {percent}% ({eta})"),
    );
    let mut row = Vec::with_capacity(NX as usize);
    for j in (0..NY).rev() {
        (0..NX)
            .into_par_iter()
            .map(|i| {
                let mut col = (0..NS).fold(Vec3(0., 0., 0.), |mut col, _| {
                    let u = (i as f32 + random::<f32>()) / NX as f32;
                    let v = (j as f32 + random::<f32>()) / NY as f32;
                    let r = cam.ray(u, v);
                    col += color(&r, &world, 0);
                    col
                });
                col /= NS as f32;
                // gamma 2
                col = Vec3(f32::sqrt(col.0), f32::sqrt(col.1), f32::sqrt(col.2));

                let ir = (255.99 * col.0) as i32;
                let ig = (255.99 * col.1) as i32;
                let ib = (255.99 * col.2) as i32;

                pb.inc(1);

                (ir, ig, ib)
            })
            .collect_into_vec(&mut row);
        for (r, g, b) in &row {
            writeln!(file, "{} {} {}", r, g, b);
        }
    }
    pb.finish_with_message("done");
}
