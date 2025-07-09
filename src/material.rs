use crate::{definitions::{is_near_zero, random_unit_vec, reflect, Color3}, hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, ray_in: &Ray<f64>, hit_record: &HitRecord, color: Color3<f64>) -> Option<(Ray<f64>, Color3<f64>)>;
}

pub struct Lambertian {
    albedo: Color3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Color3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray<f64>, hit_record: &HitRecord, color: Color3<f64>) -> Option<(Ray<f64>, Color3<f64>)> {
        let mut scatter_direction = hit_record.normal + random_unit_vec();
    
        if is_near_zero(scatter_direction) {
            scatter_direction = hit_record.normal
        }

        let outgoing = Ray::new(hit_record.point, scatter_direction);
        return Some((outgoing, self.albedo))
    }
}

pub struct Metal {
    albedo: Color3<f64>,
    fuzz: f64
}


impl Metal {
    pub fn new(albedo: Color3<f64>, fuzz: f64) -> Self {
        Self { albedo, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray<f64>, hit_record: &HitRecord, color: Color3<f64>) -> Option<(Ray<f64>, Color3<f64>)> {
        let reflected = reflect(ray_in.direction(), &hit_record.normal) + (self.fuzz * random_unit_vec());
        let outgoing = Ray::new(hit_record.point, reflected);
        if outgoing.direction().dot(&hit_record.normal) > 0.0 {
            return Some((outgoing, self.albedo))
        }
        None
    }
}

