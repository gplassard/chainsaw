use std::thread;
use std::io::{BufRead, BufReader};
use std::process::{ChildStdout, Command, Stdio};

use structopt::StructOpt;

use cli::Cli;

mod cli;

fn main() -> Result<(), ()> {
    let cli = Cli::from_args();

    let log_groups = list_log_groups();
    println!("Watching the following log groups : {}", log_groups.join(", "));

    let processes = log_groups
        .into_iter()
        .map(|log_group| {
            thread::spawn(move || {

                watch_log_group(&log_group)
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| println!("{} : {}", log_group, line));
            })
        });

    let failures =  processes
        .map(|p| p.join())
        .fold(Vec::new(), |mut errs, current| {
            if let Err(err) = current {
                errs.push(err);
            }
            errs
        });

    if failures.len() == 0 {
        Ok(())
    }
    else {
        failures.iter().for_each(|f| eprintln!("{:?}", f));
        Err(())
    }
}

fn list_log_groups() -> Vec<String> {
    let output = Command::new("saw")
        .arg("groups")
        .output()
        .unwrap();

    let stdout: String = String::from_utf8(output.stdout).unwrap();
    stdout.lines()
        .map(|l| l.to_string())
        .collect()
}

fn watch_log_group(log_group: &str) -> BufReader<ChildStdout> {
    let out = Command::new("saw")
        .arg("watch")
        .arg(log_group)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Stream failed")
        .stdout
        .expect("Stdout None");

    BufReader::new(out)
}

