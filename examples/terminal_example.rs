use std::thread;
use std::time::Duration;
use verbosio::*;

fn main() {
    set_verbosity!(2);

    vsection!(@lvl 2, "Starting process");

    let spinner = status_line!("Working on task...")
        .expect("status feature not enabled");

    thread::sleep(Duration::from_secs(2));

    thread::sleep(Duration::from_secs(2));

    spinner.stop();

    status_line_done!("✅ Done processing task!");

    vsection!("Finalizing");

    verbose!(@lvl 1, "Finishing up...");

    if let Some(spinner2) = status_line!(@lvl 3, "This won’t show unless verbosity >= 3"){
        thread::sleep(Duration::from_secs(2));
        spinner2.stop();
    }

    status_line_clear!();
}