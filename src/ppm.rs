use std::{fmt::Display, io::Write};
use crate::write_color::WriteColor;
use crate::definitions::Color3;

pub struct Pixel<T: Display> {
    red: T,
    green: T,
    blue: T,
}

impl<W: Write, T: Display> WriteColor<W> for &Pixel<T> {
    fn write_color(&self, writer: &mut W) -> std::io::Result<()> {
        let output = format!("{} {} {}\n", self.red, self.green, self.blue);
        writer.write_all(output.as_bytes())
    }
}

impl<T: Display> Pixel<T> {
    pub fn new(red: T, green: T, blue: T) -> Self {
        Self { red, green, blue }
    }
}

impl From<Color3<f64>> for Pixel<u8> {
    fn from(value: Color3<f64>) -> Self {
        Self { red: (value.x * 255.999) as u8, green: (value.y * 255.999) as u8, blue: (value.z * 255.999) as u8 }
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
                p.write_color(writer).unwrap();
                // write_color(writer, p);
            }
        }
    }
}

// fn write_color<W: Write, T: Display>(writer: &mut W, p: &Pixel<T>) {
//     let output = format!("{} {} {}\n", p.red, p.green, p.blue);
//     writer.write_all(output.as_bytes()).unwrap();
// }
