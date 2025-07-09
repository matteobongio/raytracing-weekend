use nalgebra::Vector3;
use rand::{
    distr::uniform::{SampleRange, SampleUniform},
    random_range,
};
pub type Color3<T> = Vector3<T>;

pub fn random_vec<T: SampleUniform, R: SampleRange<T> + Clone>(range: R) -> Vector3<T> {
    Vector3::new(
        random_range(range.clone()),
        random_range(range.clone()),
        random_range(range),
    )
}

pub fn random_unit_vec() -> Vector3<f64> {
    loop {
        let p = random_vec(-1.0..1.0);
        let lensq: f64 = p.norm_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: Vector3<f64>) -> Vector3<f64> {
    let random = random_unit_vec();
    if random.dot(&normal) > 0.0 {
        return random;
    }
    -random
}

pub fn is_near_zero(vec: Vector3<f64>) -> bool {
    let s = 1e-8;
    vec[0].abs() < s && vec[1].abs() < s && vec[2].abs() < s
}

pub fn reflect(incoming: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64>{
    return incoming - (2.0 * incoming.dot(normal) * normal);
}

pub fn scale_vecs(u: &Vector3<f64>, v: &Vector3<f64>) -> Vector3<f64> {
    return Vector3::new(u[0] * v[0], u[1] * v[1], u[2] * v[2]);
}
