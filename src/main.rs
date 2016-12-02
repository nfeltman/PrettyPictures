#![allow(dead_code)]

//! An example of generating julia fractals.
extern crate num_complex;
extern crate num_traits;
extern crate num;
extern crate time;
extern crate image;
extern crate rayon;
extern crate threadpool;

mod utils;
mod fractal;
mod matfill;
mod gui;
mod sampler;

use threadpool::ThreadPool;
use fractal::*;
use num_complex::Complex;
use std::fs::File;
use std::path::Path;
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use num_traits::Zero;

#[derive(Debug, Copy, Clone)]
struct RenderOptions {
    width: usize,
    height: usize,
    max_iter: u32,
    scale: f64,
    top_left: Complex<f64>,
}

type RenderReceiver = mpsc::Receiver<ColorBuffer>;

struct RuntimeData {
    pool: ThreadPool,
    options: RenderOptions,
    current_job: Option<(RenderReceiver, Arc<AtomicBool>, time::SteadyTime)>,
}


fn main() {
    run_interactive()
}

fn run_interactive() {

    impl gui::FractalGUIHandler for RuntimeData {
        fn handle_init(&mut self, disp: &gui::DisplayHandle) {
            let buffer = render(&self.options, &AtomicBool::new(false));
            disp.display(&buffer);
        }

        fn handle_scroll(&mut self, _: &gui::DisplayHandle, x: f64, y: f64, zoom_in: bool) {
            const ZOOM: f64 = 0.9;

            {
                // this scope limits the borrow of opt
                let opt = &mut self.options;
                let scroll_loc = Complex::new(x as f64, y as f64) * opt.scale + opt.top_left;
                if zoom_in {
                    opt.scale *= ZOOM;
                    opt.top_left = (opt.top_left - scroll_loc) * ZOOM + scroll_loc;
                } else {
                    opt.scale /= ZOOM;
                    opt.top_left = (opt.top_left - scroll_loc) / ZOOM + scroll_loc;
                }
            }

            // if there's a currently-running rendering job, kill it
            if let Some((_, ref kill_switch, _)) = self.current_job {
                kill_switch.store(true, Ordering::Relaxed);
            }

            // set up the new rendering job and register it
            let (sender, receiver) = mpsc::channel();
            let kill_flag = Arc::new(AtomicBool::new(false));

            self.current_job = Some((receiver, kill_flag.clone(), time::SteadyTime::now()));

            let opt2 = self.options.clone();
            self.pool.execute(move || {
                utils::start_finish_print("render", || {
                    let _ = sender.send(render(&opt2, &kill_flag));
                });
            });
        }

        fn handle_idle(&mut self, disp: &gui::DisplayHandle) {
            let replace = match &self.current_job {
                &Some((ref r, _, start_time)) => {
                    match r.try_recv() {
                        Ok(buffer) => {
                            disp.display(&buffer);

                            let duration = time::SteadyTime::now() - start_time;
                            println!("Total render time: {} ms.", duration.num_milliseconds());
                            println!("==============");
                            true
                        }
                        Err(mpsc::TryRecvError::Empty) => false,
                        Err(mpsc::TryRecvError::Disconnected) => panic!("subthread died"),
                    }
                }
                &None => false,
            };

            if replace {
                self.current_job = None
            }
        }
    }

    // set up some defaults
    let w = 800;
    let h = 600;
    let s = 4.0 / w as f64;

    let opt = RenderOptions {
        max_iter: 250,
        width: w,
        height: h,
        scale: s,
        top_left: -Complex::new((w / 2) as f64, (h / 2) as f64) * s,
    };

    gui::run_fractal_gui(w as i32,
                         h as i32,
                         RuntimeData {
                             pool: ThreadPool::new(8),
                             options: opt,
                             current_job: None,
                         });

    println!("Goodbye!");
}

// I was reading accidentallyquadratic.tumblr.com the other day and got spooked,
// so I made this scaling test to make sure that rendering is linear in the number of pixels.
//
fn scaling_test() {

    let rayon_config = rayon::Configuration::new();
    // let rayon_config = rayon_config.set_num_threads(8);
    rayon::initialize(rayon_config).expect("failed configuring rayon");

    for i in 1..1201 {
        let center = Complex::new(-0.6, 0.6);
        let w = i;
        let h = i;
        let s = 0.20 / w as f64;

        let opt = RenderOptions {
            max_iter: 250,
            width: w,
            height: h,
            scale: s,
            top_left: center - Complex::new((w / 2) as f64, (h / 2) as f64) * s,
        };

        let starttime = time::SteadyTime::now();
        let _ = render(&opt, &AtomicBool::new(false));
        let duration = time::SteadyTime::now() - starttime;
        println!("{:8}\t{}", i * i, duration.num_milliseconds());
    }
}

fn big_test() {

    let rayon_config = rayon::Configuration::new();
    rayon::initialize(rayon_config).expect("failed configuring rayon");

    let center = Complex::new(-0.6, 0.6);
    let w = 1800;
    let h = 1200;
    let s = 0.20 / w as f64;

    let opt = RenderOptions {
        max_iter: 250,
        width: w,
        height: h,
        scale: s,
        top_left: center - Complex::new((w / 2) as f64, (h / 2) as f64) * s,
    };

    let cancel = AtomicBool::new(false);

    // let cancel_clone = cancel.clone();
    // let thread = std::thread::spawn(move|| {
    //     std::thread::sleep(std::time::Duration::from_millis(200));
    //     cancel_clone.store(true, Ordering::SeqCst);
    // });

    let buffer = utils::timing_stats(1000, "render", || render(&opt, &cancel));

    // if let Err(panic) = thread.join() {
    //     println!("Thread had an error: {:?}", panic);
    // }

    let mut buffer2 = Vec::with_capacity((w * h * 3) as usize);
    for (r, g, b) in buffer {
        buffer2.push(r);
        buffer2.push(g);
        buffer2.push(b);
    }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("../output/fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let res = image::png::PNGEncoder::new(fout)
        .encode(&buffer2, w as u32, h as u32, image::ColorType::RGB(8));
    match res {
        Ok(()) => println!("Output successfully written."),
        Err(_) => println!("Problem with output."),
    }
}

type ColorBuffer = Vec<(u8, u8, u8)>;

fn render(opt: &RenderOptions, cancel: &AtomicBool) -> ColorBuffer {

    // when we're not especially zoomed in, we can use lower accuracy
    let low_accuracy = opt.scale > 0.0000001;

    // Create a new buffer
    let mut buffer = vec![(0,0,0); (opt.width*opt.height) as usize];

    sampler::sample(opt.width, opt.height, &mut buffer, || cancel.load(Ordering::Relaxed),
    |x,y| {
        let c = Complex::new(x, y) * opt.scale + opt.top_left;

        if low_accuracy {
            let c = Complex::new(c.re as f32, c.im as f32);
            iterate_smooth::<f32>(Complex::zero(), c, opt.max_iter).colorize()
        } else {
            iterate_smooth::<f64>(Complex::zero(), c, opt.max_iter).colorize()
        }
    });
    buffer
}

fn tri_colorize(n: f32) -> (u8, u8, u8) {
    let phi = 3.0 * f32::fract(n * 0.333333333);
    if phi < 1.0 {
        let rem = phi;
        let (a, b) = ((rem * 256.0) as u8, ((1.0 - rem) * 256.0) as u8);
        (a, b, 255)
    } else if phi < 2.0 {
        let rem = phi - 1.0;
        let (a, b) = ((rem * 256.0) as u8, ((1.0 - rem) * 256.0) as u8);
        (255, a, b)
    } else {
        let rem = phi - 2.0;
        let (a, b) = ((rem * 256.0) as u8, ((1.0 - rem) * 256.0) as u8);
        (b, 255, a)
    }
}

impl IterResult {
    fn colorize(self: IterResult) -> (u8, u8, u8) {
        match self {
            IterResult::Diverge(n) => tri_colorize(n * 0.1),
            IterResult::MaxIter => (0, 0, 0),
        }
    }
}


impl IterDiffResult {
    fn colorize(self: IterDiffResult, scale: f32) -> (u8, u8, u8) {
        match self {
            IterDiffResult::Diverge(n, d) => {
                let phi = (n / 4.0) % 2.0;
                let rem = phi % 1.0;
                let (a, b) = ((rem * 255.0) as u8, ((1.0 - rem) * 255.0) as u8);
                let rho = if d * scale > 0.05 { 255 } else { 0 };
                if phi < 1.0 { (a, b, rho) } else { (b, a, rho) }
            }
            IterDiffResult::MaxIter => (0, 0, 0),
        }
    }
}

fn blend((a1, a2, a3): (u8, u8, u8), (b1, b2, b3): (u8, u8, u8)) -> (u8, u8, u8) {
    ((a1 / 2 + b1 / 2), (a2 / 2 + b2 / 2), (a3 / 2 + b3 / 2))
}
