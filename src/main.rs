use std::thread;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{ChildStdout, Command, Stdio};
use aws_sdk_cloudwatchlogs::Client;

use structopt::StructOpt;

use cli::Cli;

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::from_args();

    let log_groups = list_log_groups().await?;
    println!("Watching the following log groups : {}", log_groups.join(", "));

    let processes = log_groups
        .into_iter()
        .map(|log_group| {
            thread::spawn(move || {
                println!("watching {}", log_group);

                match watch_log_group(&log_group) {
                    Some(buffer) => {
                        buffer
                            .lines()
                            .filter_map(|line| line.ok())
                            .for_each(|line| println!("{} : {}", log_group, line))
                    }
                    None => ()
                }
            })
        });
    println!("after processes");

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
        panic!("Failure")
    }
}

async fn list_log_groups() -> Result<Vec<String>, Box<dyn Error>> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.describe_log_groups();
    let resp = req.send().await?;

    // TODO : handle pagination
    let groups = resp.log_groups.unwrap_or(Vec::new())
        .into_iter()
        .filter_map(|lg| lg.log_group_name)
        .collect();

    Ok(groups)
}

fn watch_log_group(log_group: &str) -> Option<BufReader<ChildStdout>> {
    let out = Command::new("tail")
        .arg("-f")
        .arg("-n")
        .arg("-10")
        .arg(log_group)
        .stdout(Stdio::null())
        .spawn()
        .expect("Stream failed")
        .stdout?;

    Some(BufReader::new(out))
}

