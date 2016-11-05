//!An example of generating julia fractals.
extern crate num_complex;
extern crate image;
extern crate time;

use time::*;
use std::fs::File;
use std::path::Path;
use num_complex::Complex;

mod fractal;

fn main() {

    let width = 1600;
    let height = 1200;
    //let index = |(x,y) : (i32,i32)| (y * width + x) as usize;
    let deindex = |i : i32| (i % width, i / width);

    let scale = 4.0 / width as f32;

    // Create a new buffer
    let mut buffer = vec![0u8; (width*height) as usize];
    let buf = buffer.as_mut_slice();

    println!("Starting render.");
    let starttime = SteadyTime::now();

    let center = Complex::new(0f32,0f32);
    let top_left = center - Complex::new((width/2) as f32,(height/2) as f32)*scale;

    // Iterate over the coordiantes and pixels of the image
    for i in 0..width*height {
        let (x,y) = deindex(i);
        let c = Complex::new(x as f32, y as f32) * scale + top_left;
        let z = Complex::new(0.0, 0.0);
        buf[i as usize] = fractal::iterate(z,c, 255);
    }

    let duration = SteadyTime::now() - starttime;
    println!("Finished rendering.");
    println!("Elapsed time: {} ms.", duration.num_milliseconds());

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("../output/fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let res = image::png::PNGEncoder::new(fout).encode(buf,width as u32,height as u32, image::ColorType::Gray(8));
    match res {
        Ok(()) => println!("Output successfully written."),
        Err(_) => println!("Problem with output."),
    }
}