
use rayon;

use std::sync::atomic::{AtomicBool,Ordering};

pub fn fill_colors_seq<F, T> (len : usize, buffer : &mut [T], f : F)
	where F : Fn(usize) -> T
{
	fill_helper_seq(buffer, 0, len, f)
}

fn fill_helper_seq<F, T> (buffer : &mut [T], start_index : usize, len : usize, f : F)
	where F : Fn(usize) -> T
{
	for i in 0..len {
		buffer[i] = f(i+start_index);
    }
}

pub fn fill_colors<F, T> (len : usize, buffer : &mut [T], f : F)
	where F : Fn(usize) -> T + Sync, T : Send
{
	fill_helper(buffer, 0, len, &f);
}

fn fill_helper<F, T>(slice: &mut [T], start_index : usize, len : usize, f : &F) 
	where F : Fn(usize) -> T + Sync, T : Send
{
    if len < 150 {
		fill_helper_seq(slice, start_index, len, f)
    } else {
        let mid_point = len / 2;
        let (left, right) = slice.split_at_mut(mid_point);
        rayon::join(
        	|| fill_helper(left, start_index, mid_point, f), 
        	|| fill_helper(right, start_index + mid_point, len - mid_point, f)
        );
    }
}


fn fill_helper_seq_cancelable<F, T> (buffer : &mut [T], start_index : usize, len : usize, f : F, cancel : &AtomicBool)
    where F : Fn(usize) -> T
{
    for i in 0..len {
        if cancel.load(Ordering::Relaxed) {return ()}
        buffer[i] = f(i+start_index);
    }
}

pub fn fill_colors_cancelable<F, T> (len : usize, buffer : &mut [T], f : F, cancel : &AtomicBool)
    where F : Fn(usize) -> T + Sync, T : Send
{
    fill_helper_cancelable(buffer, 0, len, &f, cancel);
}

fn fill_helper_cancelable<F, T>(slice: &mut [T], start_index : usize, len : usize, f : &F, cancel : &AtomicBool) 
    where F : Fn(usize) -> T + Sync, T : Send
{
    if cancel.load(Ordering::Relaxed) {return;}

    if len < 1000 {
        fill_helper_seq_cancelable(slice, start_index, len, f, cancel)
    } else {
        let mid_point = len / 2;
        let (left, right) = slice.split_at_mut(mid_point);
        rayon::join(
            || fill_helper_cancelable(left, start_index, mid_point, f, cancel), 
            || fill_helper_cancelable(right, start_index + mid_point, len - mid_point, f, cancel)
        );
    }
}

