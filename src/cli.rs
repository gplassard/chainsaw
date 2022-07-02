use structopt::StructOpt;

#[derive(StructOpt)]
/// Aggregates logs of multiple cloudwatch log groups by merging output from multiple "saw" commands
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    Watch
}
