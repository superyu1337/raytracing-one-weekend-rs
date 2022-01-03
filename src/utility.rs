use glam::DVec3;
use rand::Rng;

use crate::{ray::{Ray}, hittable::{Hittable, HitRecord}, hittable_list::HittableList, IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLE_COUNT};

pub type Color = glam::f64::DVec3;

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min };
    if x > max { return max };
    return x;
}

pub fn f32_to_u8(clr: f32) -> u8 {
    (256.0 * clamp(clr, 0.0, 0.999)) as u8
}

pub fn f64_normalize(x: f64) -> f64 {
    let scale = 1.0 / SAMPLE_COUNT as f64;
    libm::sqrt(scale * x)
}

pub fn vec3_normalize(vec: DVec3) -> DVec3 {
    DVec3::new(
        f64_normalize(vec.x),
        f64_normalize(vec.y),
        f64_normalize(vec.z),
    )
}

pub fn write_to_image(image_data: Vec<u8>) {
    let image = image::RgbImage::from_vec(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, image_data)
        .expect("Create image buffer from image data");

    image.save("./image.png").expect("Save image");
}


pub fn ray_color<T: Hittable>(ray: &Ray, world: &HittableList<T>, ray_depth: usize) -> (Color, DVec3) {
    let mut hit_record = HitRecord::new();

    if ray_depth <= 0 {
        return (Color::new(0.0, 0.0, 0.0), hit_record.normal);
    }

    if world.hit(ray, 0.001, std::f64::INFINITY, &mut hit_record) {
        //let scattered = Ray::new(DVec3::default(), DVec3::default());
        //let attenuation = Color::new(0.0, 0.0, 0.0);

        //if (hit_record)

        let target = hit_record.point + hit_record.normal + random_unit_vector();
        let (clr, _) = ray_color(&Ray::new(hit_record.point, target - hit_record.point), world, ray_depth-1);
        return (0.5 * clr, hit_record.normal);
    }

    let unit_direction = DVec3::from(ray.direction()).normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    return ((1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0), hit_record.normal);
}

pub fn vec3_random_with(min: f64, max: f64) -> DVec3 {
    DVec3::new(
        rand::thread_rng().gen_range(min..max), 
        rand::thread_rng().gen_range(min..max), 
        rand::thread_rng().gen_range(min..max)
    )
}

pub fn random_unit_sphere() -> DVec3 {
    loop {
        let p = vec3_random_with(-1.0, 1.0);
        if p.length_squared() >= 1.0 { continue }
        return p;
    }
}

pub fn random_unit_vector() -> DVec3 {
    random_unit_sphere().normalize()
}

/*
pub fn vec3_near_zero(vec: &DVec3) -> bool {
    let s = 1e-8;
    (libm::fabs(vec.x) < s) && (libm::fabs(vec.y) < s) && (libm::fabs(vec.z) < s)
}

pub fn vec3_reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - 2.0*v.dot(n)*n
}
*/