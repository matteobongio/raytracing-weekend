use std::f64::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn new_inf() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, item: f64) -> bool {
        self.min < item && item < self.max
    }
}
