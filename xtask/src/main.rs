use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about="Build tools for the project", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Run the qemu")]
    Qemu,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Qemu => {
            let _ = Command::new("echo").arg("qemu done").status();
        }
    }
    Ok(())
}
