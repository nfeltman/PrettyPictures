extern crate num_complex;

use num_complex::Complex;

/*pub fn iterate(z0 : Complex<f32>, c : Complex<f32>, max : u8) -> Option<u8> {
	
	let mut i = 0;
	let mut z = z0;

    for t in 0..max {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
        i = t;
    }

    None
}*/

pub enum IterResult{
    Diverge(f32),
    MaxIter,
}

pub fn iterate_smooth(z0 : Complex<f32>, c : Complex<f32>, max : u32) -> IterResult {
    
    let mut z = z0;

    let bailout = (2<<16) as f32;

    for t in 0..max {
        if z.norm_sqr() > bailout {
            let nu = f32::log2( f32::log2( z.norm_sqr() ) / 2.0 );
            return IterResult::Diverge((t as f32) - nu);
        }
        z = z * z + c;
    }

    IterResult::MaxIter
}

pub enum IterDiffResult{
    Diverge(f32, f32),
    MaxIter,
}

pub fn iterate_smooth_diffc(z0 : Complex<f32>, c : Complex<f32>, max : u32) -> IterDiffResult {
    
    let mut z = z0;
    let mut diffz = Complex::new(0.0,0.0);

    let bailout = (2<<16) as f32;

    for t in 0..max {
        let z_norm_sqr = z.norm_sqr();
        if z_norm_sqr > bailout {
            let z_norm = z_norm_sqr.sqrt();
            let nu = f32::log2(f32::log2(z.norm()));
            let d = diffz.norm() / z_norm / f32::ln(z_norm) / f32::ln(2.0);
            //println!("diff {}", d);
            return IterDiffResult::Diverge((t as f32) - nu, d);
        }
        diffz = 2.0 * diffz * z + Complex::new(1.0,1.0);
        z = z * z + c;
    }

    IterDiffResult::MaxIter
}