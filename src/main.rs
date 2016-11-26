#![allow(dead_code)]

//!An example of generating julia fractals.
extern crate num_complex;
extern crate num;
extern crate rayon;
extern crate gtk;
extern crate gdk_pixbuf;
extern crate gdk_sys;
extern crate time;

mod utils;
mod fractal;
mod matfill;
mod gui;

use fractal::*;
use std::cell::Cell;
use num_complex::Complex;

fn main() {

    let center = Complex::new(-0.6,0.6);
    let w = 800;
    let h = 600;
    let s = 0.20 / w as f64;

    let opt = RenderOptions {
        max_iter : 250,
        width : w,
        height : h,
        scale : s,
        top_left : center - Complex::new((w/2) as f64,(h/2) as f64)*s,
    };

    
    let buffer = render(&opt);

    let mut buffer2 = Vec::with_capacity(opt.width*opt.height*3);
    // Convert the vector of tuples to a long vector
    for (r,g,b) in buffer {
        buffer2.push(r);
        buffer2.push(g);
        buffer2.push(b);
    }

    let opt_cell = Cell::new(opt);

    gui::run_fractal_gui(w as i32, h as i32, buffer2, move |x,y,dir| {
        let mut opt = opt_cell.get();
        
        const ZOOM : f64 = 0.9;

        let scroll_loc = Complex::new(x as f64, y as f64) * opt.scale + opt.top_left;
        if dir {
            opt.scale *= ZOOM;
            opt.top_left = (opt.top_left - scroll_loc) * ZOOM + scroll_loc;
        }
        else {
            opt.scale /= ZOOM;
            opt.top_left = (opt.top_left - scroll_loc) / ZOOM + scroll_loc;
        }

        opt_cell.set(opt);

        utils::start_finish_print("Beginning render.", "Done with render.", ||{
            render(&opt)
        })
    });



    println!("Goodbye!");


    // // Save the image as “fractal.png”
    // let ref mut fout = File::create(&Path::new("../output/fractal.png")).unwrap();

    // // We must indicate the image’s color type and what format to save as
    // let res = image::png::PNGEncoder::new(fout).encode(&buffer2,opt.width as u32,opt.height as u32, image::ColorType::RGB(8));
    // match res {
    //     Ok(()) => println!("Output successfully written."),
    //     Err(_) => println!("Problem with output."),
    // }
}

/*
    I was reading accidentallyquadratic.tumblr.com the other day and got spooked,
    so I made this scaling test to make sure that rendering is linear in the number of pixels.
*/
fn scaling_test() {
    for i in 1..1200 {
        let center = Complex::new(-0.6,0.6);
        let w = i;
        let h = i;
        let s = 0.20 / w as f64;

        let opt = RenderOptions {
            max_iter : 250,
            width : w,
            height : h,
            scale : s,
            top_left : center - Complex::new((w/2) as f64,(h/2) as f64)*s,
        };

        let starttime = time::SteadyTime::now();
        let _ = render(&opt);
        let duration = time::SteadyTime::now() - starttime;
        println!("{:8}\t{}", i*i, duration.num_milliseconds());
    }
}

#[derive(Debug, Copy, Clone)]
struct RenderOptions {
    width : usize,
    height : usize,
    max_iter : u32,
    scale : f64,
    top_left : Complex<f64>,
}

fn render(opt : &RenderOptions) -> Vec<(u8,u8,u8)>
{
    let deindex = |i| (i % opt.width, i / opt.width);

    // when we're not especially zoomed in, we can use lower accuracy
    let low_accuracy = opt.scale > 0.0000001;
    
    // Create a new buffer
    let mut buffer = vec![(0,0,0); (opt.width*opt.height) as usize];
    matfill::fill_colors((opt.width*opt.height) as usize, &mut buffer, |i| {
        let (x,y) = deindex(i);
        let (x,y) = (x as f64, y as f64);
        let c0 = Complex::new(x, y) * opt.scale + opt.top_left;
        // let c1 = Complex::new(x, y+0.5) * scale + top_left;
        // let c2 = Complex::new(x+0.5, y) * scale + top_left;
        // let c3 = Complex::new(x+0.5, y+0.5) * scale + top_left;
        let z = Complex::new(0.0, 0.0);
        
        if low_accuracy
        {
            // f32 accuracy
            let c0 = Complex::new(c0.re as f32, c0.im as f32);
            let z = Complex::new(z.re as f32, z.im as f32);
            iterate_smooth(z,c0,opt.max_iter).colorize()
        }
        else {
            // f64 accuracy
            iterate_smooth(z,c0,opt.max_iter).colorize()
        }
    });
    buffer
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
    ((a1/2+b1/2), (a2/2+b2/2), (a3/2+b3/2))
}