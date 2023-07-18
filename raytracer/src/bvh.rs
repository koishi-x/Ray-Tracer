#![allow(dead_code)]
use std::cmp::Ordering;

use crate::*;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    box_: AABB,
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.box_)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // if let None = self.box_.hit(r, t_min, t_max) {
        //     return None;
        // }
        self.box_.hit(r, t_min, t_max)?;

        match self.left.hit(r, t_min, t_max) {
            // None => match self.right.hit(r, t_min, t_max) {
            //     None => None,
            //     Some(rec) => Some(rec),
            // },
            None => self.right.hit(r, t_min, t_max),
            Some(rec) => match self.right.hit(r, t_min, rec.t) {
                None => Some(rec),
                Some(rec2) => Some(rec2),
            },
        }
    }
}

impl BvhNode {
    pub fn new(
        mut src_objects: Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = random_int(0, 2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let object_span = end - start;
        // let mut left: Arc<dyn Hittable>;
        // let mut right: Arc<dyn Hittable>;

        // if object_span == 1 {
        //     left = sArc_objects[start].clone();
        //     right =
        // }
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if object_span == 1 {
            left = src_objects[start].clone();
            right = src_objects[start].clone();
        } else if object_span == 2 {
            if comparator(&src_objects[start], &src_objects[start + 1]) == Ordering::Less {
                left = src_objects[start].clone();
                right = src_objects[start + 1].clone();
            } else {
                left = src_objects[start + 1].clone();
                right = src_objects[start].clone();
            }
        } else {
            //sArc_objects[start..end].sort_by(|a, b| comparator(a, b));
            src_objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            left = Arc::new(BvhNode::new(src_objects.clone(), start, mid, time0, time1));
            right = Arc::new(BvhNode::new(src_objects, mid, end, time0, time1));
        }

        let box_ = surrounding_box(
            &(left.bounding_box(time0, time1).unwrap()),
            &(right.bounding_box(time0, time1).unwrap()),
        );
        BvhNode { left, right, box_ }
    }
    pub fn new_hittablelist(list: HittableList, time0: f64, time1: f64) -> Self {
        let len = list.objects.len();
        //println!("{len}");
        Self::new(list.objects, 0, len, time0, time1)
    }
}

#[allow(dead_code)]
fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: i32) -> Option<bool> {
    match a.bounding_box(0.0, 0.0) {
        None => None,
        Some(box_a) => match b.bounding_box(0.0, 0.0) {
            None => None,
            Some(box_b) => match axis {
                0 => Some(box_a.minimum.x < box_b.minimum.x),
                1 => Some(box_a.minimum.y < box_b.minimum.y),
                2 => Some(box_a.minimum.z < box_b.minimum.z),
                _ => None,
            },
        },
    }
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    match box_compare(a, b, 0) {
        None => {
            eprintln!("No bounding box in bvh_node constructor.\n");
            Ordering::Less
        }
        Some(res) => {
            if res {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    match box_compare(a, b, 1) {
        None => {
            eprintln!("No bounding box in bvh_node constructor.\n");
            Ordering::Less
        }
        Some(res) => {
            if res {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    match box_compare(a, b, 2) {
        None => {
            eprintln!("No bounding box in bvh_node constructor.\n");
            Ordering::Less
        }
        Some(res) => {
            if res {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}
