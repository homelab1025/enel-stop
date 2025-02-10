use std::{env, thread, time::Instant, vec};

use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    let cli_arg = env::args().nth(1).unwrap_or(String::from("45"));
    let fibonacci_count = cli_arg.parse::<u64>().unwrap();

    info!("Starting calculation for {}", fibonacci_count);
    let start = Instant::now();
    info!("fibonacci({}) = {}", fibonacci_count, fibonacci(fibonacci_count));
    info!("Elapsed: {:?}", start.elapsed());

    let start = Instant::now();
    let mut handles = vec![];
    for _x in 0..10 {
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

    info!("Elapsed: {:?}", start.elapsed());
}

fn fibonacci(nth: u64) -> u64 {
    if nth == 0 || nth == 1 {
        nth
    } else {
        fibonacci(nth - 1) + fibonacci(nth - 2)
    }
}
