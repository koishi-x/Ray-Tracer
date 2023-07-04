use crate::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        //self.orig.plus(&self.dir.multiply(t))
        self.orig + self.dir * t
    }
}
