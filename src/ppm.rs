use std::{fmt::Display, io::Write};

pub struct Pixel<T: Display> {
    red: T,
    green: T,
    blue: T,
}

impl<T: Display> Pixel<T> {
    pub fn new(red: T, green: T, blue: T) -> Self {
        Self {red, green, blue}
    }
}


pub struct Image<T: Display> {
    image_height: usize,
    image_width: usize,
    max: usize,
    data: Vec<Vec<Pixel<T>>>,
}

impl<T: Display> Image<T> {
    pub fn new(
        image_height: usize,
        image_width: usize,
        max: usize,
        data: Vec<Vec<Pixel<T>>>,
    ) -> Self {
        Self {
            image_height,
            image_width,
            max,
            data,
        }
    }
    pub fn write_ppm<W: Write>(&self, writer: &mut W) {
        let header = format!(
            "P3\n{} {}\n{}\n",
            self.image_width, self.image_height, self.max
        );
        writer
            .write_all(header.as_bytes())
            .expect("cannot write PPM header to file.");
        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let p = &self.data[i][j];
                let output = format!("{} {} {}\n", p.red, p.green, p.blue);
                writer.write_all(output.as_bytes()).unwrap();
            }
        }
    }
}
