use crate::{ray::Ray, hittable::HitRecord, utility::{Color, random_unit_vector, vec3_near_zero, vec3_reflect}};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: HitRecord<impl Material>, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: HitRecord<impl Material>, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if vec3_near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;

        return scattered.direction().dot(hit_record.normal) > 0.0;
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: HitRecord<impl Material>, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = vec3_reflect(ray_in.direction().normalize(), hit_record.normal);
        *scattered = Ray::new(hit_record.point, reflected);
        *attenuation = self.albedo;

        return scattered.direction().dot(hit_record.normal) > 0.0;
    }
}