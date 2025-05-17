use std::{fs::File, io};

use ppm::{Image, Pixel};

mod ppm;
fn main() {
    let image_height = 256;
    let image_width = 256;
    let max = 255;
    let mut image: Vec<Vec<ppm::Pixel<u8>>> = Vec::new();
    for i in 0..image_height {
        let mut row = Vec::new();
        for j in 0..image_width {
            let r = (i as f32) / (image_width - 1) as f32 * 255.999;
            let g = (j as f32) / (image_height - 1) as f32 * 255.999;
            let b = 0.0;
            let pix = Pixel::new(r as u8, g as u8, b as u8);
            row.push(pix);
        }
        image.push(row);
    }
    // let mut file = File::create("out.ppm").unwrap();
    let mut stdout = io::stdout().lock();
    let image = Image::new(image_height, image_width, max, image);
    image.write_ppm(&mut stdout);
}
