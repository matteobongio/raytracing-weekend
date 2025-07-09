use crate::{
    definitions::{Color3, random_on_hemisphere, random_unit_vec, scale_vecs},
    hittable::HittableList,
    interval::Interval,
    ppm::{self, Image, Pixel},
    ray::Ray,
};
use nalgebra::Vector3;
use rand::random_range;
use std::{f64::INFINITY, io};

pub struct Camera {
    image_width: usize,
    image_height: usize,
    // aspect_ratio: f64,
    focal_length: f64,
    // viewport_height: f64,
    // viewport_width: f64,
    viewport_u: Vector3<f64>,
    viewport_v: Vector3<f64>,
    camera_center: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    samples_per_pixel: usize,
    max_depth: usize,
}

impl Camera {
    pub fn new(
        image_width: usize,
        image_height: usize,
        focal_length: f64,
        viewport_height: f64,
        camera_center: Vector3<f64>,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        let aspect_ratio = (image_width as f64) / (image_height as f64);
        let viewport_width = viewport_height * (aspect_ratio);
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u.scale(1.0 / image_width as f64);
        let pixel_delta_v = viewport_v.scale(1.0 / image_height as f64);
        Self {
            image_width,
            image_height,
            // aspect_ratio,
            focal_length,
            // viewport_height,
            // viewport_width,
            viewport_u,
            viewport_v,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
        }
    }
    pub fn get_viewport_upper_left(&self) -> Vector3<f64> {
        self.camera_center
            - Vector3::new(0.0, 0.0, self.focal_length)
            - self.viewport_u.scale(0.5)
            - self.viewport_v.scale(0.5)
    }
    pub fn get_pixel00_loc(&self) -> Vector3<f64> {
        self.get_viewport_upper_left() + 0.5 * (self.pixel_delta_u + self.pixel_delta_v)
    }

    fn get_pixel_samples_scale(&self) -> f64 {
        1.0 / (self.samples_per_pixel as f64)
    }

    pub fn render(&self, hittables: &HittableList) {
        let max = 255;
        let mut image: Vec<Vec<ppm::Pixel<u8>>> = Vec::new();
        let pixel_samples_scale = self.get_pixel_samples_scale();

        for j in 0..self.image_height {
            eprintln!("scanlines remaining: {}\n", self.image_height - j);
            let mut row = Vec::new();
            for i in 0..self.image_width {
                let mut pixel_color = Color3::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += color_ray(&ray, self.max_depth, hittables);
                }
                row.push(Pixel::from(pixel_color * pixel_samples_scale));
            }
            image.push(row);
        }

        // let mut file = File::create("out.ppm").unwrap();
        let mut stdout = io::stdout().lock();
        let image = Image::new(self.image_height, self.image_width, max, image);
        image.write_ppm(&mut stdout);
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray<f64> {
        let offset = self.sample_square();
        let pixel_sample = self.get_pixel00_loc()
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.camera_center;
        let ray_dir = pixel_sample - ray_origin;
        return Ray::new(ray_origin, ray_dir);
    }

    fn sample_square(&self) -> Vector3<f64> {
        return Vector3::new(
            random_range(0.0 as f64..=0.99999) - 0.5,
            random_range(0.0 as f64..=0.99999) - 0.5,
            0.0,
        );
    }
}

fn color_ray(r: &Ray<f64>, depth: usize, hittables: &HittableList) -> Color3<f64> {
    if depth <= 0 {
        return Color3::new(0.0, 0.0, 0.0);
    }
    let hit = hittables.hit(r, Interval::new(0.001, INFINITY));
    if let Some(hr) = hit {
        // // let normal = (r.at(hr.t) - Vector3::new(0.0, 0.0, -1.0)).normalize();
        // return hr.normal.add_scalar(1.0).scale(0.5);
        if let Some((scattered, attenuation)) =
            hr.material.scatter(r, &hr, Color3::new(0.0, 0.0, 0.0))
        {
            return scale_vecs(&attenuation, &color_ray(&scattered, depth - 1, hittables));
        }
        return Color3::new(0.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color3::new(1.0, 1.0, 1.0) + a * Color3::new(0.5, 0.7, 1.0)
}
