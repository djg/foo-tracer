extern crate indicatif;
extern crate rand;
extern crate rayon;

use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use std::{f32, fs::File, io::Write};

mod aabb;
mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

use aabb::*;
use camera::*;
use hitable::*;
use material::*;
use ray::*;
use sphere::*;
use vec3::*;
use world::*;

const NX: i32 = 400;
const NY: i32 = 200;
//const NX: i32 = 1280;
//const NY: i32 = 720;
const NS: i32 = 100;

fn color(r: &Ray, world: &World, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
                return attenuation * color(&scattered, world, depth + 1);
            }
        }
        return Vec3(0., 0., 0.);
    } else {
        let unit_direction = normalized(r.direction);
        let t = 0.5 * (unit_direction.1 + 1.);
        (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
    }
}

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
    World::new(entities)
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
            }).collect_into_vec(&mut row);
        for (r, g, b) in &row {
            writeln!(file, "{} {} {}", r, g, b);
        }
    }
    pb.finish_with_message("done");
}
