use approx::assert_relative_eq;
use nalgebra::Vector3;

use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(point: Vector3<f64>, outward_normal: Vector3<f64>, t: f64, ray: &Ray<f64>) -> Self {
        let (front_face, normal) = Self::set_face_normal(&ray, &outward_normal);
        Self { point, normal, t, front_face }

    }
    fn set_face_normal(ray: &Ray<f64>, outward_normal: &Vector3<f64>) ->(bool, Vector3<f64>) {
        // outward_normal should be a unit vector
        // assert_relative_eq!(outward_normal.magnitude(), 1.0);
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = {
            if front_face {
                outward_normal.clone()
            } else {
                outward_normal.scale(-1.0)
            }
        };
        (front_face, normal)
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray<f64>, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}


pub struct HittableList(pub Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn new() -> Self {
        return Self(Vec::new());
    }
    
    pub fn hit(&self, ray: &Ray<f64>, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut output = None;
        let mut closest_so_far = ray_tmax;

        for hittable in &self.0 {
            if let Some(hit) =  hittable.hit(ray, ray_tmin, closest_so_far) {
                closest_so_far = hit.t;
                output = Some(hit);
            }
        }
        output
    }
}






pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray<f64>, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().norm_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        //find closest root in range
        let mut root = (h - discriminant.sqrt()) / a;
        if root < ray_tmin || root > ray_tmax {
            root = (h + discriminant.sqrt()) / a;
            if root < ray_tmin || root > ray_tmax {
                return None;
            }
        }
        let t = root;
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        let hr = HitRecord::new(point, normal, t, ray);
        Some(hr)
    }


}
