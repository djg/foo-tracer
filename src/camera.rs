use super::*;

fn random_in_unit_disk() -> Vec3 {
    let mut p = Vec3(1., 1., 1.);
    while p.squared_len() >= 1. {
        p = 2. * Vec3(random::<f32>(), random::<f32>(), 0.) - Vec3(1., 1., 0.);
    }
    p
}

pub struct Camera {
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
