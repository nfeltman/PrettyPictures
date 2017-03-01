
use std::ops::Add;
use matfill::*;

pub trait Averageable
{
	type I : Add<Output = Self::I>;
	fn zero () -> Self::I;
	fn lift (self) -> Self::I;
	fn finish (Self::I) -> Self;
}

pub struct WColor (u16,u16,u16,u16);
impl Add for WColor {
	type Output = WColor;
	fn add (self, o : WColor) -> WColor {
		WColor(self.0 + o.0, self.1 + o.1, self.2 + o.2, self.3 + o.3)
	}
}

impl Averageable for (u8,u8,u8) {
    
	type I = WColor;
	fn zero () -> WColor 
	{
		WColor(0,0,0,0)
	}
	fn lift (self : (u8,u8,u8)) -> WColor {
		WColor(self.0 as u16, self.1 as u16, self.2 as u16, 1)
	}
	fn finish (WColor(r,g,b,w) : Self::I) -> (u8,u8,u8) {
		((r/w) as u8, (g/w) as u8, (b/w) as u8)
	}
}

pub fn sample<F, T, C>(w : usize, h : usize, buffer: &mut [T], cancel: C, f: F)
    where F: Fn(f64,f64) -> T + Sync,
          T: Send,
          C: Fn() -> bool + Sync
{
	assert_eq!(w*h, buffer.len());
    fill_colors(buffer, cancel, move |i| {
		let x = (i % w) as f64;
		let y = (i / w) as f64;
    	f(x,y)
    });
}

pub fn sample4<F, T, C>(w : usize, h : usize, buffer: &mut [T], cancel: C, f: F)
    where F: Fn(f64,f64) -> T + Sync,
          T: Averageable + Send,
          C: Fn() -> bool + Sync
{
	assert_eq!(w*h, buffer.len());
    fill_colors(buffer, cancel, move |i| {
		let x = (i % w) as f64;
		let y = (i / w) as f64;
    	T::finish(f(x,y).lift() + f(x+0.5,y).lift() + f(x,y+0.5).lift() + f(x+0.5,y+0.5).lift())
    });
}