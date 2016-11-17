use crossbeam::*;
// use utils;
// use rayon::prelude::*;

pub fn fill_colors<F> (len : usize, buffer : &mut Vec<u8>, f : F)
where F : Fn(usize) -> (u8,u8,u8)
{
	for i in 0..len {

		let (r,g,b) = f(i);
        let i = i*3;
        buffer[i] = r;
        buffer[i+1] = g;
        buffer[i+2] = b;
    }
}

/*pub fn fill_colors_rayon<F> (num_threads : u32, len : usize, buffer : &mut Vec<u8>, f : F)
where F : Fn(usize) -> (u8,u8,u8) + Sync + Send
{

}
*/
pub fn fill_colors_par<F> (num_threads : u32, len : usize, buffer : &mut Vec<u8>, f : F)
where F : Fn(usize) -> (u8,u8,u8) + Sync + Send
{
	let num_threads = num_threads as usize;

	let mut join_handles = Vec::with_capacity(num_threads);
	let mut results = Vec::with_capacity(num_threads);
	let min_size = len / num_threads;
	let remainder = len % num_threads;

	scope(|scope| {
	    let f_ref = &f;

	    // spin up a bunch of threads; each thread works on a "chunk"
	    // chunks have size of either min_size or (min_size + 1)
	    // chunks are interleaved
	    for t in 0..num_threads{
			join_handles.push(scope.spawn(move || {

				// size of this chunk
				let chunk_size = min_size + (if t < remainder {1} else {0});
				let mut local_results = Vec::with_capacity(chunk_size);
				for i in 0..chunk_size {
					let pixel = i * num_threads + t;
					local_results.push(f_ref(pixel));
				}
				local_results
			}));
		}

		// wait for all the threads and add the results to the results vector
		for h in join_handles { 
			results.push(h.join());
		}
	});

	// copy and unpack all-but-one elements in the big 
	// chunks and all elements in small chunks
	for i in 0..min_size {
		for t in 0..num_threads {
			let (r,g,b) = results[t][i];
	        let i = (i*num_threads+t)*3;
	        buffer[i] = r;
	        buffer[i+1] = g;
	        buffer[i+2] = b;
	    }
	}

	// copy and unpack the last element of the big chunks
	for t in 0..remainder {
		let (r,g,b) = results[t][min_size];
        let i = (min_size*num_threads+t)*3;
        buffer[i] = r;
        buffer[i+1] = g;
        buffer[i+2] = b;
    }
}