use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long, default_value = "info")]
    level: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    Serve { port: u16 },
    Check,
}

fn main() {
    let cli = Cli::parse();
    println!("{cli:?}");
}
