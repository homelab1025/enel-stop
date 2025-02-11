use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
    thread,
    time::Instant,
};

use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    let cli_arg = env::args().nth(1).unwrap_or(String::from("45"));
    let fibonacci_count = cli_arg.parse::<u64>().unwrap();

    info!("single thread: Starting calculation for {}", fibonacci_count);
    let start = Instant::now();
    info!("fibonacci({}) = {}", fibonacci_count, fibonacci(fibonacci_count));
    info!("Elapsed: {:?}", start.elapsed());

    let core_count = get_core_count().expect("Could not detect number of cores");

    info!("Detected {} cores.", core_count);

    run_fibonacci(core_count / 2, fibonacci_count);
    run_fibonacci(core_count, fibonacci_count);
    run_fibonacci(core_count * 2, fibonacci_count);
}

fn run_fibonacci(core_count: u8, fibonacci_count: u64) {
    info!("Running with {} cores.", core_count);

    let start = Instant::now();
    let mut handles = vec![];
    for _x in 0..core_count {
        let n = fibonacci_count;
        let handle = thread::spawn(move || {
            fibonacci(n)
            // info!("fibonacci({})-{} is {}", n, x, fibonacci(n));
        });
        handles.push(handle);
    }

    for handle in handles {
        let _r = handle.join();
    }

    info!("Elapsed: {:?} for {} cores.", start.elapsed(), core_count);
}

fn get_core_count() -> Result<u8, Error> {
    let cpuinfo_file = File::open("/proc/cpuinfo")?;

    let mut count: u8 = 0;

    let reader = BufReader::new(cpuinfo_file);
    for line in reader.lines() {
        if line?.starts_with("processor\t:") {
            count += 1;
        }
    }

    Ok(count)
}

fn fibonacci(nth: u64) -> u64 {
    if nth == 0 || nth == 1 {
        nth
    } else {
        fibonacci(nth - 1) + fibonacci(nth - 2)
    }
}
