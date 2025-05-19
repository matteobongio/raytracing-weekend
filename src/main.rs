pub mod camera;
pub mod definitions;
pub mod ppm;
pub mod ray;
pub mod write_color;
use camera::Camera;
use definitions::Color3;
use nalgebra::Vector3;
use ppm::{Image, Pixel};
use ray::Ray;
use std::io;

fn hit_sphere(center: Vector3<f64>, radius: f64, ray: &Ray<f64>) -> f64 {
    let oc = center - ray.origin();
    let a = ray.direction().norm_squared();
    let h = ray.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant >= 0.0 {
        return (h - discriminant.sqrt()) / a;
    }
    -1.0
}

fn color_ray(r: &Ray<f64>) -> Color3<f64> {
    let sphere_center = Vector3::new(0.0, 0.0, -1.0);
    let hit = hit_sphere(sphere_center, 0.5, r);
    if hit > 0.0 {
        let normal = (r.at(hit) - Vector3::new(0.0, 0.0, -1.0)).normalize();
        return normal.add_scalar(1.0).scale(0.5);
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
    let max = 255;
    let mut image: Vec<Vec<ppm::Pixel<u8>>> = Vec::new();

    let pixel_delta_u = camera.get_pixel_delta_u();
    let pixel_delta_v = camera.get_pixel_delta_v();
    let pixel00_loc = camera.get_pixel00_loc();

    for j in 0..image_height {
        eprintln!("scanlines remaining: {}\n", image_height - j);
        let mut row = Vec::new();
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + pixel_delta_u.scale(i as f64) + pixel_delta_v.scale(j as f64);
            let ray_dir = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_dir);
            let color = color_ray(&ray);
            row.push(Pixel::from(color));
        }
        image.push(row);
    }

    // let mut file = File::create("out.ppm").unwrap();
    let mut stdout = io::stdout().lock();
    let image = Image::new(image_height, image_width, max, image);
    image.write_ppm(&mut stdout);
}

fn draw_gradient(i: usize, j: usize, image_width: usize, image_height: usize) -> Pixel<u8> {
    let r = (i as f32) / (image_width - 1) as f32 * 255.999;
    let g = (j as f32) / (image_height - 1) as f32 * 255.999;
    let b = 0.0;
    Pixel::new(r as u8, g as u8, b as u8)
}
