extern crate indicatif;
extern crate rand;
extern crate rayon;

use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use std::{f32, fs::File, io::Write, time::Instant};

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

const NX: usize = 200;
const NY: usize = 100;
//const NX: usize = 1280;
//const NY: usize = 720;
//const NS: usize = 100;
const NS: usize = 100;

pub struct Stats {
    pub first_rays: u64,
    pub bounce_rays: u64,
    pub miss_rays: u64,
    pub entity_hit: u64,
    pub entity_miss: u64,
}

fn random_scene() -> World {
    let mut entities: Vec<Box<dyn Hitable>> = Vec::with_capacity(501);
    entities.push(Box::new(Sphere::new(
        Vec3(0., -1000., 0.),
        1000.,
        Box::new(lambertian(Vec3(0.5, 0.5, 0.5))),
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
                        Box::new(lambertian(Vec3(
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
                        Box::new(metal(
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
                        Box::new(dielectric(1.5)),
                    )));
                }
            }
        }
    }

    entities.push(Box::new(Sphere::new(
        Vec3(0., 1., 0.),
        1.,
        Box::new(dielectric(1.5)),
    )));
    entities.push(Box::new(Sphere::new(
        Vec3(-4., 1., 0.),
        1.,
        Box::new(lambertian(Vec3(0.4, 0.2, 0.1))),
    )));
    entities.push(Box::new(Sphere::new(
        Vec3(4., 1., 0.),
        1.,
        Box::new(metal(Vec3(0.7, 0.6, 0.5), 0.)),
    )));
    World::new(entities)
}

fn pixel(cam: &Camera, world: &World, i: usize, j: usize, stats: &mut Stats) -> u32 {
    let mut col = Vec3(0., 0., 0.);
    for _ in 0..NS {
        let u = (i as f32 + random::<f32>()) / NX as f32;
        let v = (j as f32 + random::<f32>()) / NY as f32;
        let r = cam.ray(u, v);
        col += world.color(r, stats);
    }
    col /= NS as f32;
    // gamma 2
    col = Vec3(f32::sqrt(col.0), f32::sqrt(col.1), f32::sqrt(col.2));

    let ir = (255.99 * col.0) as u32;
    let ig = (255.99 * col.1) as u32;
    let ib = (255.99 * col.2) as u32;

    (ib << 16) | (ig << 8) | ir
}

fn main() {
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

    let mut image = Vec::<u32>::new();
    image.resize(NX as usize * NY as usize, 0);
    // let pb = ProgressBar::new(NX as u64 * NY as u64);
    // pb.set_style(
    //     ProgressStyle::default_bar().template("{elapsed_precise} {wide_bar} {percent}% ({eta})"),
    // );
    let mut stats = Stats {
        first_rays: 0,
        bounce_rays: 0,
        miss_rays: 0,
        entity_hit: 0,
        entity_miss: 0,
    };
    let start = Instant::now();
    // if cfg!(feature = "go-faster") {
    //     for j in (0..NY).rev() {
    //         image[(NY - 1 - j) * NX..(NY - j) * NX]
    //             .par_iter_mut()
    //             .enumerate()
    //             .for_each(|(i, rgb)| {
    //                 *rgb = pixel(&cam, &world, i, j, &mut stats);
    //                 //                    pb.inc(1);
    //             });
    //     }
    // } else {
    for j in (0..NY).rev() {
        for i in 0..NX {
            image[NX * (NY - 1 - j) + i] = pixel(&cam, &world, i, j, &mut stats);
        }
        //            pb.inc(1);
    }
    // }
    let duration = Instant::now().duration_since(start);

    let dt = 1_000_000. * duration.as_secs() as f64 + duration.subsec_millis() as f64 * 1000.;
    println!(
        "Rays {:.3} Mrays/s:\n  total:  {}\n  first:  {}\n  bounce: {}\n  miss:   {}\n",
        (stats.first_rays + stats.bounce_rays) as f64 / dt,
        stats.first_rays + stats.bounce_rays,
        stats.first_rays,
        stats.bounce_rays,
        stats.miss_rays,
    );
    println!(
        "Entities:\n  total: {}\n  hit:   {}\n  miss:  {}\n",
        stats.entity_hit + stats.entity_miss,
        stats.entity_hit,
        stats.entity_miss
    );

    let mut file = File::create("image.ppm").expect("Failed to open image.ppm");
    writeln!(file, "P3\n{} {}\n255", NX, NY);
    for rgb in &image {
        writeln!(
            file,
            "{} {} {}",
            rgb & 0xff,
            (rgb >> 8) & 0xffu32,
            (rgb >> 16) & 0xffu32
        );
    }
}
