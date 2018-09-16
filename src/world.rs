use super::*;
use std::{f32, mem};

#[derive(Clone, Copy)]
enum BvhNode {
    Branch { bbox: Aabb },
    Leaf { entity: usize },
    Empty,
}

use self::BvhNode::{Branch, Empty, Leaf};

pub struct World {
    pub entities: Vec<Hitable>,
    bvh: Vec<BvhNode>,
}

impl World {
    pub fn new(entities: Vec<Hitable>) -> World {
        let n = entities.len();
        let mut ids = (0..n).collect::<Vec<usize>>();
        let mut world = World {
            entities,
            bvh: vec![],
        };
        world.build_bvh(0, &mut ids);
        world
    }

    fn build_bvh(&mut self, node_id: usize, ids: &mut [usize]) {
        let axis = thread_rng().gen_range::<usize>(0, 3);
        ids.sort_unstable_by(|&i, &j| {
            let a = self.entities[i].bbox().unwrap();
            let b = self.entities[j].bbox().unwrap();
            a.min[axis].partial_cmp(&b.min[axis]).unwrap()
        });

        let node_left = 2 * node_id + 1;
        let node_right = 2 * node_id + 2;
        if self.bvh.len() < 2 * node_id + 3 {
            self.bvh.resize(2 * node_id + 3, Empty);
        }
        let n = ids.len();
        match n {
            1 => {
                self.bvh[node_left] = Leaf { entity: ids[0] };
                self.bvh[node_right] = Leaf { entity: ids[0] };
            }
            2 => {
                self.bvh[node_left] = Leaf { entity: ids[0] };
                self.bvh[node_right] = Leaf { entity: ids[1] };
            }
            n => {
                let (left_ids, right_ids) = ids.split_at_mut(n / 2);
                self.build_bvh(node_left, left_ids);
                self.build_bvh(node_right, right_ids);
            }
        }
        let left_aabb = self
            .bvh_bbox(node_left)
            .expect("No bounding box in BVH node construction");
        let right_aabb = self
            .bvh_bbox(node_right)
            .expect("No bounding box in BVH node construction");
        let bbox = surrounding_box(&left_aabb, &right_aabb);
        self.bvh[node_id] = Branch { bbox };
    }

    fn bvh_bbox(&self, node_id: usize) -> Option<Aabb> {
        match self.bvh[node_id] {
            Branch { bbox } => Some(bbox),
            Leaf { entity } => self.entities[entity].bbox(),
            Empty => None,
        }
    }
}

impl World {
    #[cfg(feature = "bvh")]
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: &mut f32, entity: &mut usize, stats: &mut Stats) {
        let mut top = 0;
        let mut visit: [u32; 32] = unsafe { mem::uninitialized() };
        visit[top] = 0;
        top += 1;

        while top > 0 {
            assert!(top > 0 && top < 32);
            top -= 1;
            let node_id = visit[top];
            match self.bvh[node_id as usize] {
                Branch { bbox } => {
                    if bbox.hit(r, t_min, *t_max) {
                        assert!(top < 30);
                        visit[top] = 2 * node_id + 2;
                        visit[top + 1] = 2 * node_id + 1;
                        top += 2;
                        stats.aabb_hit += 1;
                    } else {
                        stats.aabb_miss += 1;
                    }
                }
                Leaf { entity: n } => {
                    if self.entities[n].hit(r, t_min, t_max) {
                        stats.entity_hit += 1;
                        *entity = n;
                    } else {
                        stats.entity_miss += 1;
                    }
                }
                Empty => unreachable!(),
            }
        }
    }

    #[cfg(not(feature = "bvh"))]
    fn hit(&self, r: &Ray, t_min: f32, t_max: &mut f32, entity: &mut usize, stats: &mut Stats) {
        for (n, ntt) in self.entities.iter().enumerate() {
            if ntt.hit(r, t_min, t_max) {
                stats.entity_hit += 1;
                *entity = n;
            } else {
                stats.entity_miss += 1;
            }
        }
    }

    pub fn color(&self, mut r: Ray, stats: &mut Stats) -> Vec3 {
        let mut depth = 0;
        let mut color = Vec3(1., 1., 1.);
        let mut attenuation = Vec3(1., 1., 1.);
        let mut hit = HitRecord {
            normal: Vec3(0., 0., 0.),
            point: Vec3(0., 0., 0.),
        };

        while depth < 50 {
            if depth == 0 {
                stats.first_rays += 1;
            } else {
                stats.bounce_rays += 1;
            }

            let mut t = std::f32::MAX;
            let mut entity = !0;
            self.hit(&r, 0.001, &mut t, &mut entity, stats);
            if entity != !0 {
                self.entities[entity].hit_record(&r, t, &mut hit);
                if self.entities[entity]
                    .material()
                    .scatter(&mut r, &mut attenuation, &hit)
                {
                    color *= attenuation;
                } else {
                    break;
                }
            } else {
                stats.miss_rays += 1;
                let t = 0.5 * (r.direction.1 + 1.);
                return (1. - t) * color + t * Vec3(0.5, 0.7, 1.) * color;
            }
            depth += 1;
        }

        Vec3(0., 0., 0.)
    }
}

// World is read-only during rendering.
unsafe impl std::marker::Sync for World {}
