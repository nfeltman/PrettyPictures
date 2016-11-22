use rayon;

pub fn fill_colors_seq<F, T> (len : usize, buffer : &mut [T], f : F)
	where F : Fn(usize) -> T
{
	render_seq(buffer, 0, len, f)
}

fn render_seq<F, T> (buffer : &mut [T], start_index : usize, len : usize, f : F)
	where F : Fn(usize) -> T
{
	for i in 0..len {
		buffer[i] = f(i+start_index);
    }
}

pub fn fill_colors<F, T> (len : usize, buffer : &mut [T], f : F)
	where F : Fn(usize) -> T + Sync, T : Send
{
	render(buffer, 0, len, &f);
}

fn render<F, T>(slice: &mut [T], start_index : usize, len : usize, f : &F) 
	where F : Fn(usize) -> T + Sync, T : Send
{
    if len < 1000 {
		render_seq(slice, start_index, len, f)
    } else {
        let mid_point = len / 2;
        let (left, right) = slice.split_at_mut(mid_point);
        rayon::join(
        	|| render(left, start_index, mid_point, f), 
        	|| render(right, start_index + mid_point, len - mid_point, f)
        );
    }
}
