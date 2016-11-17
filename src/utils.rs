extern crate time;


pub fn start_finish_print<F : FnMut()> (start_message : &str, end_message : &str, mut f : F)
{
    println!("{}", start_message);
    let starttime = time::SteadyTime::now();

    f();

    let duration = time::SteadyTime::now() - starttime;
    println!("{}", end_message);
    println!("Elapsed time: {} ms.", duration.num_milliseconds());
}