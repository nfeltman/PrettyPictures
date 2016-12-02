
use rayon;

pub fn fill_colors_seq<F, T>(buffer: &mut [T], f: F)
    where F: Fn(usize) -> T
{
    fill_helper_seq(buffer, 0, f, &|| false)
}

fn fill_helper_seq<F, T, C>(buffer: &mut [T],
                                    start_index: usize,
                                    f: F,
                                    cancel: &C)
    where F: Fn(usize) -> T, 
          C: Fn() -> bool
{
    for i in 0..buffer.len() {
        if cancel() {
            return ();
        }
        buffer[i] = f(i + start_index);
    }
}

pub fn fill_colors<F, T, C>(buffer: &mut [T], cancel: C, f: F)
    where F: Fn(usize) -> T + Sync,
          T: Send,
          C: Fn() -> bool + Sync
{
    fill_helper(buffer, 0, &f, &cancel);
}

fn fill_helper<F, T, C>(slice: &mut [T],
                    start_index: usize,
                    f: &F,
                    cancel: &C)
    where F: Fn(usize) -> T + Sync,
          T: Send, 
          C: Fn() -> bool + Sync
{
    if cancel() {
        return;
    }

    if slice.len() < 1000 {
        fill_helper_seq(slice, start_index, f, cancel)
    } else {
        let mid_point = slice.len() / 2;
        let (left, right) = slice.split_at_mut(mid_point);
        rayon::join(|| fill_helper(left, start_index, f, cancel),
                    || fill_helper(right, start_index + mid_point, f, cancel)
        );
    }
}