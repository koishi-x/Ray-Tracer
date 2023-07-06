use crate::vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub orig: vec3::Vec3,
    pub dir: vec3::Vec3,
}

impl Ray {
    // pub fn new() -> Self {
    //     Ray {
    //         orig: Vec3::new(),
    //         dir: Vec3::new(),
    //     }
    // }

    pub fn at(&self, t: f64) -> vec3::Vec3 {
        //self.orig.plus(&self.dir.multiply(t))
        self.orig + self.dir * t
    }
}
