use super::*;

#[inline]
fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1. / r.direction[a];
            let mut t0 = (self.min[a] - r.point[a]) * inv_d;
            let mut t1 = (self.max[a] - r.point[a]) * inv_d;
            if inv_d < 0. {
                ::std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = ffmax(t0, t_min);
            let t_max = ffmin(t1, t_max);
            if t_max < t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let min = Vec3(
        ffmin(box0.min.0, box1.min.0),
        ffmin(box0.min.1, box1.min.1),
        ffmin(box0.min.2, box1.min.2),
    );
    let max = Vec3(
        ffmax(box0.max.0, box1.max.0),
        ffmax(box0.max.1, box1.max.1),
        ffmax(box0.max.2, box1.max.2),
    );
    Aabb { min, max }
}
