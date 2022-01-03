use crate::{hittable::{Hittable, HitRecord}};

#[derive(Clone)]
pub struct HittableList<T: Hittable> {
    objects: Vec<T>
}

impl<T: Hittable> HittableList<T> {

    pub fn new() -> HittableList<T> {
        HittableList { 
            objects: std::vec::Vec::new() 
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_record= HitRecord::new();
        let mut hit_anything = false;
        let mut closest = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest, &mut temp_record) {
                hit_anything = true;
                closest = temp_record.t;
                *hit_record = temp_record;
            }
        }

        return hit_anything;
    }
}