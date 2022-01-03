use glam::f64::DVec3;

pub type Point3 = DVec3;

pub struct Ray {
    origin: Point3,
    dir: DVec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: DVec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> DVec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t*self.dir
    }
}