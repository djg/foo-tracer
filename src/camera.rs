use super::*;

fn random_in_unit_disk() -> Vec3 {
    let mut p = Vec3(1., 1., 1.);
    while p.squared_len() >= 1. {
        p = 2. * Vec3(random::<f32>(), random::<f32>(), 0.) - Vec3(1., 1., 0.);
    }
    p
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    right: Vec3,
    up: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
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
        let back = normalized(look_from - look_at);
        let right = normalized(cross(up, back));
        let up = cross(back, right);
        let lower_left_corner =
            origin - focus_dist * (half_width * right + half_height * up + back);
        let horizontal = 2. * half_width * focus_dist * right;
        let vertical = 2. * half_height * focus_dist * up;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            right,
            up,
            lens_radius,
        }
    }
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.right * rd.0 + self.up * rd.1;
        Ray {
            point: self.origin + offset,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - offset,
        }
    }
}
