use super::*;

#[derive(Clone, Copy)]
enum BvhNode {
    Branch { bbox: Aabb },
    Leaf { entity: usize },
    Empty,
}

use self::BvhNode::{Branch, Empty, Leaf};

pub struct World {
    pub entities: Vec<Box<dyn Hitable>>,
    bvh: Vec<BvhNode>,
}

impl World {
    pub fn new(entities: Vec<Box<dyn Hitable>>) -> World {
        let n = entities.len();
        let mut ids = (0..n).collect::<Vec<usize>>();
        let mut world = World {
            entities,
            bvh: vec![],
        };
        world.build_bvh(0, &mut ids);
        world
    }

    fn hit_bvh(&self, r: &Ray, t_min: f32, t_max: f32, node_id: usize) -> Option<HitRecord> {
        match self.bvh[node_id] {
            Branch { bbox } => {
                if bbox.hit(r, t_min, t_max) {
                    let hit_left = self.hit_bvh(r, t_min, t_max, 2 * node_id + 1);
                    let hit_right = self.hit_bvh(r, t_min, t_max, 2 * node_id + 2);
                    match (hit_left, hit_right) {
                        (Some(hl), Some(hr)) => if hl.t < hr.t {
                            Some(hl)
                        } else {
                            Some(hr)
                        },
                        (Some(hl), None) => Some(hl),
                        (None, Some(hr)) => Some(hr),
                        (None, None) => None,
                    }
                } else {
                    None
                }
            }
            Leaf { entity } => self.entities[entity].hit(r, t_min, t_max),
            Empty => unreachable!(),
        }
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
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hit_bvh(r, t_min, t_max, 0)
    }

    #[cfg(not(feature = "bvh"))]
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, stats: &mut Stats) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;
        for entity in &self.entities {
            if let Some(hit) = entity.hit(r, t_min, closest_so_far) {
                stats.entity_hit += 1;
                closest_so_far = hit.t;
                result = Some(hit)
            } else {
                stats.entity_miss += 1;
            }
        }
        result
    }
}

// World is read-only during rendering.
unsafe impl std::marker::Sync for World {}
