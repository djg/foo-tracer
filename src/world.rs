use super::*;

pub struct World {
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

// World is read-only during rendering.
unsafe impl std::marker::Sync for World {}
