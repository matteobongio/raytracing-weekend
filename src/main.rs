pub mod camera;
pub mod definitions;
pub mod hittable;
pub mod interval;
pub mod ppm;
pub mod ray;
pub mod write_color;
use camera::Camera;
use definitions::Color3;
use hittable::{HittableList, Sphere};
use interval::Interval;
use nalgebra::Vector3;
use ppm::{Image, Pixel};
use ray::Ray;
use std::{f64::INFINITY, io};

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

struct Scene {
    camera: Camera,
}

fn main() {
    let image_width = 900;
    let image_height = 400;
    let camera_center = Vector3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(image_width, image_height, 1.0, 2.0, camera_center);
    let mut hittables = HittableList::new();
    hittables
        .0
        .push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    hittables
        .0
        .push(Box::new(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5)));
    hittables.0.push(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
    )));
    camera.render(&hittables);
}

// fn draw_gradient(i: usize, j: usize, image_width: usize, image_height: usize) -> Pixel<u8> {
//     let r = (i as f32) / (image_width - 1) as f32 * 255.999;
//     let g = (j as f32) / (image_height - 1) as f32 * 255.999;
//     let b = 0.0;
//     Pixel::new(r as u8, g as u8, b as u8)
// }
