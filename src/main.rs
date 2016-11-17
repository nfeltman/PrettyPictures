#![allow(dead_code)]
#![allow(unused_variables)]

//!An example of generating julia fractals.
extern crate num_complex;
extern crate image;
extern crate rayon;
extern crate crossbeam;

mod utils;
mod fractal;
mod matfill;

use fractal::*;
use std::fs::File;
use std::path::Path;
use num_complex::Complex;

fn main() {

    let max = 200;
    let width = 3200;
    let height = 2400;
    //let index = |(x,y) : (i32,i32)| (y * width + x) as usize;
    let deindex = |i| (i % width, i / width);

    let scale = 0.20 / width as f32;

    // Create a new buffer
    let mut buffer = vec![0u8; (width*height*3) as usize];

    let center = Complex::new(-0.6,0.6);
    let top_left = center - Complex::new((width/2) as f32,(height/2) as f32)*scale;

    for _ in 0..1 {
    utils::start_finish_print("Beginning render.", "Done with render.", ||
        // Iterate over the coordiantes and pixels of the image
        matfill::fill_colors((width*height) as usize, &mut buffer, |i| {
            let (x,y) = deindex(i);
            let c = Complex::new(x as f32, y as f32) * scale + top_left;
            let z = Complex::new(0.0, 0.0);
            
            if true {
                iterate_smooth(z,c,max).colorize()
            }
            else 
            {
                match iterate_smooth_diffc(z, c, max) {
                    IterDiffResult::Diverge(n,diff) => tri_colorize(n*0.5),
                        // if diff * scale > 2.0 
                        // {
                        //     (200,200,200)
                        // } 
                        // else 
                        // {
                        //     tri_colorize(n)
                        // },
                    res => res.colorize(scale)
                } 
            }
            
        })
    );}

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("../output/fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let res = image::png::PNGEncoder::new(fout).encode(&buffer,width as u32,height as u32, image::ColorType::RGB(8));
    match res {
        Ok(()) => println!("Output successfully written."),
        Err(_) => println!("Problem with output."),
    }
}

fn tri_colorize (n : f32) -> (u8,u8,u8) {
    let phi = 3.0 * f32::fract(n * 0.333333333);
    if phi < 1.0 {
        let rem = phi;
        let (a,b) = ((rem * 256.0) as u8, ((1.0-rem) * 256.0) as u8);
        (a,b,255)
    } 
    else if phi < 2.0 {
        let rem = phi - 1.0;
        let (a,b) = ((rem * 256.0) as u8, ((1.0-rem) * 256.0) as u8);
        (255,a,b)
    } 
    else {
        let rem = phi - 2.0;
        let (a,b) = ((rem * 256.0) as u8, ((1.0-rem) * 256.0) as u8);
        (b,255,a)
    }
}

impl IterResult {
    fn colorize (self : IterResult) -> (u8,u8,u8) {
        match self {
            IterResult::Diverge (n) => tri_colorize(n*0.1),
            IterResult::MaxIter => (0,0,0),
        }
    }
}


impl IterDiffResult {
    fn colorize (self : IterDiffResult, scale : f32) -> (u8,u8,u8) {
        match self {
            IterDiffResult::Diverge (n,d) => {
                let phi = (n/4.0) % 2.0;
                let rem = phi % 1.0;
                let (a,b) = ((rem * 255.0) as u8, ((1.0-rem) * 255.0) as u8);
                let rho = if d * scale > 0.05 {255} else {0};
                if phi < 1.0 {
                    (a,b,rho)
                } 
                else {
                    (b,a,rho)
                }
            },
            IterDiffResult::MaxIter => (0,0,0),
        }
    }
}

fn blend((a1,a2,a3) : (u8,u8,u8), (b1,b2,b3) : (u8,u8,u8)) -> (u8,u8,u8)
{
    ((a1+b1) / 2, (a2+b2) / 2, (a3+b3) / 2)
}