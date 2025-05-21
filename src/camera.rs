use std::{f64::INFINITY, io};

use nalgebra::Vector3;

use crate::{definitions::Color3, hittable::{HittableList, Sphere}, interval::Interval, ppm::{self, Image, Pixel}, ray::Ray};

pub struct Camera {
    image_width: usize,
    image_height: usize,
    aspect_ratio: f64,
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    viewport_u: Vector3<f64>,
    viewport_v: Vector3<f64>,
    camera_center: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
}

impl Camera {
    pub fn new(
        image_width: usize,
        image_height: usize,
        focal_length: f64,
        viewport_height: f64,
        camera_center: Vector3<f64>,
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
            aspect_ratio,
            focal_length,
            viewport_height,
            viewport_width,
            viewport_u,
            viewport_v,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
    // pub fn get_pixel_delta_u(&self) -> &Vector3<f64> {
    //     &self.pixel_delta_u
    // }
    // pub fn get_pixel_delta_v(&self) -> &Vector3<f64> {
    //     &self.pixel_delta_v
    // }
    // pub fn get_image_height(&self) -> usize {
    //     self.image_height
    // }
    // pub fn get_image_width(&self) -> usize {
    //     self.image_width
    // }
    pub fn get_viewport_upper_left(&self) -> Vector3<f64> {
        self.camera_center
            - Vector3::new(0.0, 0.0, self.focal_length)
            - self.viewport_u.scale(0.5)
            - self.viewport_v.scale(0.5)
    }
    pub fn get_pixel00_loc(&self) -> Vector3<f64> {
        self.get_viewport_upper_left() + 0.5 * (self.pixel_delta_u + self.pixel_delta_v)
    }

    pub fn render(&self, hittables: &HittableList) {
        let max = 255;
        let mut image: Vec<Vec<ppm::Pixel<u8>>> = Vec::new();

        let pixel00_loc = self.get_pixel00_loc();


        for j in 0..self.image_height {
            eprintln!("scanlines remaining: {}\n", self.image_height - j);
            let mut row = Vec::new();
            for i in 0..self.image_width {
                let pixel_center =
                    pixel00_loc + self.pixel_delta_u.scale(i as f64) + self.pixel_delta_v.scale(j as f64);
                let ray_dir = pixel_center - self.camera_center;
                let ray = Ray::new(self.camera_center, ray_dir);
                let color = color_ray(&ray, &hittables);
                row.push(Pixel::from(color));
            }
            image.push(row);
        }

        // let mut file = File::create("out.ppm").unwrap();
        let mut stdout = io::stdout().lock();
        let image = Image::new(self.image_height, self.image_width, max, image);
        image.write_ppm(&mut stdout);
    }

}
fn color_ray(r: &Ray<f64>, hittables: &HittableList) -> Color3<f64> {
    let hit = hittables.hit(r, Interval::new(0.0, INFINITY));
    if let Some(hr) = hit {
        // let normal = (r.at(hr.t) - Vector3::new(0.0, 0.0, -1.0)).normalize();
        return hr.normal.add_scalar(1.0).scale(0.5);
    }
    let unit_direction = r.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color3::new(1.0, 1.0, 1.0) + a * Color3::new(0.5, 0.7, 1.0)
}
