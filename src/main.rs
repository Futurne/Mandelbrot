use image::RgbImage;
use ndarray::{Array, Ix2, Ix3, Zip};
use num::complex::Complex;

type ComplexArray = Array<Complex<f32>, Ix2>;

fn init_c_array(x_range: (f32, f32), y_range: (f32, f32), im_size: (usize, usize)) -> ComplexArray {
    let mut array = Array::zeros(im_size);
    let x_delta = (x_range.1 - x_range.0) / im_size.1 as f32;
    let y_delta = (y_range.1 - y_range.0) / im_size.0 as f32;

    // TODO: Initialize using higher order function (map, iter...)
    for i in 0..im_size.0 {
        for j in 0..im_size.1 {
            let x = x_range.0 + j as f32 * x_delta;
            let y = y_range.0 + i as f32 * y_delta;
            array[(i, j)] = Complex::new(x, y);
        }
    }
    array
}

fn array_to_image(array: ComplexArray) -> RgbImage {
    let (height, width) = array.dim();
    let mut rgb_array: Array<u8, Ix3> = Array::zeros((height, width, 3));

    // TODO: Initialize using higher order function
    for ((y, x), v) in array.indexed_iter() {
        let value = if v.is_nan() { 255 } else { 0 };
        for i in 0..3 {
            rgb_array[(y, x, i)] = value;
        }
    }
    let raw = rgb_array.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn main() {
    let x_range = (-2.0, 1.0);
    let y_range = (-1.0, 1.0);
    let im_size = (320, 480); // height x width
    let total_iter = 100;

    let c = init_c_array(x_range, y_range, im_size);
    let mut z: ComplexArray = Array::zeros(im_size);

    // TODO: is it possible to optimize this?
    for _ in 0..total_iter {
        Zip::from(&mut z)
            .and(&c)
            .for_each(|z, &c| *z = (*z) * (*z) + c);
    }

    let image = array_to_image(z);
    image.save("out.png").expect("Save failed!");
}
