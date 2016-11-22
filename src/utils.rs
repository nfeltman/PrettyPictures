extern crate time;


pub fn start_finish_print<T, F> (start_message : &str, end_message : &str, f : F) -> T
	where F : FnOnce() -> T
{
    println!("{}", start_message);
    let starttime = time::SteadyTime::now();

    let result = f();

    let duration = time::SteadyTime::now() - starttime;
    println!("{}", end_message);
    println!("Elapsed time: {} ms.", duration.num_milliseconds());

    result
}