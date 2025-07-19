use clap::Parser;
use verbosio::*;

#[derive(Parser)]
struct Args {
    /// Verbosity level (default: 1)
    #[clap(short, long, default_value_t = 1)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    set_verbosity!(args.verbose);
    run_ls();
}

use std::process::Command;
fn run_ls() {
    verbose!(@lvl 1, "Running directory listing command...");

    let mut cmd: Command;

    #[cfg(target_family = "unix")]
    {
        cmd = Command::new("ls");
        cmd.arg("-la");
    }


    #[cfg(target_family = "windows")]
    {
        cmd = Command::new("cmd");
        cmd.arg("/C").arg("dir");
    }

    let output = cmd.output().expect("Failed to execute command");

    if output.status.success() {
        verbose!(@lvl 1, "Command output:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        verror!(@lvl 1, "Command failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}
