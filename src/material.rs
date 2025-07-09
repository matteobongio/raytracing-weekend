use rand::random_range;

use crate::{
    definitions::{Color3, is_near_zero, random_unit_vec, reflect, refract},
    hittable::HitRecord,
    ray::Ray,
};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray<f64>,
        hit_record: &HitRecord,
        color: Color3<f64>,
    ) -> Option<(Ray<f64>, Color3<f64>)>;
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
    fn scatter(
        &self,
        ray_in: &Ray<f64>,
        hit_record: &HitRecord,
        color: Color3<f64>,
    ) -> Option<(Ray<f64>, Color3<f64>)> {
        let mut scatter_direction = hit_record.normal + random_unit_vec();

        if is_near_zero(scatter_direction) {
            scatter_direction = hit_record.normal
        }

        let outgoing = Ray::new(hit_record.point, scatter_direction);
        return Some((outgoing, self.albedo));
    }
}

pub struct Metal {
    albedo: Color3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color3<f64>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray<f64>,
        hit_record: &HitRecord,
        color: Color3<f64>,
    ) -> Option<(Ray<f64>, Color3<f64>)> {
        let reflected =
            reflect(ray_in.direction(), &hit_record.normal) + (self.fuzz * random_unit_vec());
        let outgoing = Ray::new(hit_record.point, reflected);
        if outgoing.direction().dot(&hit_record.normal) > 0.0 {
            return Some((outgoing, self.albedo));
        }
        None
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray<f64>,
        hit_record: &HitRecord,
        color: Color3<f64>,
    ) -> Option<(Ray<f64>, Color3<f64>)> {
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = ray_in.direction().normalize();

        let cos_theta = (-unit_dir).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract
            || (Self::reflectance(cos_theta, ri) > random_range(0.0 as f64..1.0))
        {
            reflect(&unit_dir, &hit_record.normal)
        } else {
            refract(&unit_dir, &hit_record.normal, ri)
        };
        Some((
            Ray::new(hit_record.point, direction),
            Color3::new(1.0, 1.0, 1.0),
        ))
    }
}
