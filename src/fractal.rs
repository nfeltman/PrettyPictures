
use num_complex::Complex;
use num::Float;

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

pub fn iterate_smooth<F>(z0 : Complex<F>, c : Complex<F>, max : u32) -> IterResult
    where F : Float
{
    
    let mut z = z0;

    let bailout = F::from(2<<16).expect("simple conversion failed");
    let half = F::from(0.5).expect("simple conversion failed");

    for t in 0..max {
        if z.norm_sqr() > bailout {
            let nu = (z.norm_sqr().log2() * half).log2().to_f32().expect("simple conversion failed");
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