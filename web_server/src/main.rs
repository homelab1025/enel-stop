use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
    thread,
    time::{Instant, SystemTime},
};

use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use tokio::{io::AsyncReadExt, sync::watch};

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    // let cli_arg = env::args().nth(1).unwrap_or(String::from("45"));
    // let fibonacci_count = cli_arg.parse::<u64>().unwrap();

    let core_count = get_core_count().expect("Could not detect number of cores");

    info!("Detected {} cores.", core_count);

    // in one task watch over the /proc/loadavg file and send notif if the file changes
    // in the other task wait for the notification and if received read the file and log the
    // contents
    let (tx, mut rx) = watch::channel(0u64);
    let path = Path::new("/tmp/test123");

    tokio::spawn(watch_loadavg_file(tx, path));
    let receiver = tokio::spawn(load_receiver(rx, path));

    let _ = receiver.await;

    // run_fibonacci(core_count, fibonacci_count);
    // run_fibonacci(core_count / 2, fibonacci_count);
    // run_fibonacci(core_count * 2, fibonacci_count);
}

async fn load_receiver(mut rx: watch::Receiver<u64>, path: &Path) {
    loop {
        let _ = rx.changed().await;

        info!("received notif");
        if let Ok(contents) = read_loadavg_file(path.to_str().unwrap()).await {
            info!("loadavg: {}", contents);
        }
    }
    // let content = rx.borrow_and_update().push_str
}

async fn watch_loadavg_file(tx: watch::Sender<u64>, path: &Path) {
    let mut last_modified = None;
    loop {
        if let Ok(metdata) = path.metadata() {
            let modified = metdata.modified().unwrap();

            if last_modified != Some(modified) {
                last_modified = Some(modified);
                // let contents = read_loadavg_file(path.to_str().unwrap()).await.unwrap();
                info!("change was done");
                let r = tx.send(modified.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
                info!("result {:?}", r);
            }
        }

        // This is needed as there is no other await in the method and so the other task will
        // simply not get to run because we are running an infinite loop in this task, hence it
        // never ends.
        // With await we are telling the runtime that it's fine to take on another task.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn read_loadavg_file(file_path: &str) -> Result<String, Error> {
    let mut load_file = tokio::fs::File::open(file_path).await?;
    let mut contents = String::new();

    load_file.read_to_string(&mut contents).await?;
    Ok(contents)
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
