pub mod camera;
pub mod definitions;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod ppm;
pub mod ray;
pub mod write_color;
use std::rc::Rc;

use camera::Camera;
use definitions::Color3;
use hittable::{HittableList, Sphere};
use material::{Dielectric, Lambertian, Metal};
use nalgebra::Vector3;

// struct Scene {
//     camera: Camera,
// }

fn make_camera() -> Camera {
    let image_width = 900;
    let image_height = 400;
    let camera_center = Vector3::new(0.0, 0.0, 0.0);
    Camera::new(image_width, image_height, 1.0, 2.0, camera_center, 10, 50)
}

fn main() {
    let camera = make_camera();

    let mut hittables = HittableList::new();
    let lambertian_blue = Rc::new(Lambertian::new(Color3::new(0.1, 0.2, 0.5)));
    let lambertian_green = Rc::new(Lambertian::new(Color3::new(0.1, 0.5, 0.2)));
    let metal = Rc::new(Metal::new(Color3::new(0.8, 0.8, 0.8), 0.01));
    let dielectic = Rc::new(Dielectric::new(1.33));
    let bubble = Rc::new(Dielectric::new(1.0 / 1.33));
    hittables.0.push(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        lambertian_blue,
    )));
    hittables.0.push(Box::new(Sphere::new(
        Vector3::new(-1.1, 0.0, -1.0),
        0.5,
        dielectic,
    )));
    hittables.0.push(Box::new(Sphere::new(
        Vector3::new(-1.1, 0.0, -1.0),
        0.4,
        bubble,
    )));
    hittables.0.push(Box::new(Sphere::new(
        Vector3::new(1.1, 0.0, -1.0),
        0.5,
        metal,
    )));
    // hittables.0.push(Box::new(Sphere::new(
    //     Vector3::new(0.0, 0.0, -150_000_000_000_000.0),
    //     695_508_000.0,
    //     lambertian_green,
    // )));
    hittables.0.push(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.5, -1.0),
        1000.0,
        lambertian_green,
    )));

    camera.render(&hittables);
}
