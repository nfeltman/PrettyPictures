use rayon;

pub fn fill_colors_seq<F> (len : usize, buffer : &mut Vec<u8>, f : F)
	where F : Fn(usize) -> (u8,u8,u8)
{
	render_seq(buffer, 0, len, f)
}

fn render_seq<F> (buffer : &mut [u8], start_index : usize, len : usize, f : F)
	where F : Fn(usize) -> (u8,u8,u8)
{
	for i in 0..len {

		let (r,g,b) = f(i+start_index);
        let i = i*3;
        buffer[i] = r;
        buffer[i+1] = g;
        buffer[i+2] = b;
    }
}

pub fn fill_colors<F> (len : usize, buffer : &mut Vec<u8>, f : F)
	where F : Fn(usize) -> (u8,u8,u8) + Sync + Send
{
	render(buffer.as_mut_slice(), 0, len, &f);
}

fn render<F>(slice: &mut [u8], start_index : usize, len : usize, f : &F) 
	where F : Fn(usize) -> (u8,u8,u8) + Sync
{
    if len < 1000 {
		render_seq(slice, start_index, len, f)
    } else {
        let mid_point = len / 2;
        let (left, right) = slice.split_at_mut(mid_point*3);
        rayon::join(
        	|| render(left, start_index, mid_point, f), 
        	|| render(right, start_index + mid_point, len - mid_point, f)
        );
    }
}
