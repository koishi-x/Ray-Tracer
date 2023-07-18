use crate::*;

pub struct AABox {
    pub box_min: Point3,
    pub box_max: Point3,
    pub sides: HittableList,
}

impl AABox {
    pub fn new(p0: Point3, p1: Point3, ptr: Arc<dyn Material>) -> AABox {
        let mut sides = HittableList::new();
        sides.add(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        sides.add(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));

        sides.add(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        sides.add(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));

        sides.add(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        sides.add(Arc::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, ptr)));

        AABox {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hittable for AABox {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
