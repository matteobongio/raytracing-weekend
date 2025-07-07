pub mod camera;
pub mod definitions;
pub mod hittable;
pub mod interval;
pub mod ppm;
pub mod ray;
pub mod write_color;
use camera::Camera;
use hittable::{HittableList, Sphere};
use nalgebra::Vector3;

// struct Scene {
//     camera: Camera,
// }

fn main() {
    let image_width = 900;
    let image_height = 400;
    let camera_center = Vector3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(image_width, image_height, 1.0, 2.0, camera_center, 10);
    let mut hittables = HittableList::new();
    hittables
        .0
        .push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    // hittables
    //     .0
    //     .push(Box::new(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5)));
    hittables
        .0
        .push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -150_000_000_000_000.0), 695_508_000.0)));
    hittables.0.push(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.5, -1.0),
        1000.0,
    )));
    camera.render(&hittables);
}
