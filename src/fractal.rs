extern crate num_complex;

use num_complex::Complex;

pub fn iterate(z0 : Complex<f32>, c : Complex<f32>, max : u8) -> u8 {
	
	let mut i = 0;
	let mut z = z0;

    for t in 0..max {
        if z.norm_sqr() > 4.0 {
            break
        }
        z = z * z + c;
        i = t;
    }

    i
}