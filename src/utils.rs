extern crate time;
extern crate pbr;

use self::pbr::ProgressBar;

pub fn start_finish_print<T, F>(name: &str, f: F) -> T
    where F: FnOnce() -> T
{
    println!("Beginning {}.", name);
    let starttime = time::SteadyTime::now();

    let result = f();

    let duration = time::SteadyTime::now() - starttime;
    println!("Finished {}. Elapsed time: {} ms.",
             name,
             duration.num_milliseconds());

    result
}

pub fn timing_stats<T, F>(num_trials: u32, name: &str, f: F) -> T
    where F: Fn() -> T
{
    if num_trials == 0 {
        panic!("Must run at least once.");
    }
    let mut durations = Vec::with_capacity(num_trials as usize);
    let mut result = None;

    println!("Beginning {}, {} iterations.", name, num_trials);

    let mut pb = ProgressBar::new(num_trials as u64);

    for _ in 1..num_trials {

        let starttime = time::SteadyTime::now();
        let r = f();
        let finishtime = time::SteadyTime::now();

        pb.inc();
        result = Some(r);
        // this unwrap will succeed, unless the test took 250+ years
        durations.push((finishtime - starttime).num_microseconds().unwrap() as f32 / 1000.0);
    }


    let num_trials = num_trials as f32;
    let mu = durations.iter().sum::<f32>() / num_trials;
    let std = (durations.iter().map(|x| (x - mu)).map(|x| x * x).sum::<f32>() / (num_trials - 1.0))
        .sqrt();

    pb.finish_println(&format!("Finised {}\nAverage time: {} ms.\nStandard Deviation: {} ms.\n",
                               name,
                               mu,
                               std));

    result.unwrap()
}
