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
    #[clap(about = "Build code")]
    Bin,
    #[clap(about = "Run qemu")]
    Qemu,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Qemu => {
            let _ = Command::new("qemu-system-riscv64")
                .args(["-M", "128m"])
                .args(["-machine", "virt"])
                .args(["-nographic"])
                .args(["-bios", "default"])
                .args(["-device", "loader,file=kernel.bin,addr=0x80200000"])
                .status();
        }
        Commands::Bin => {
            let _ = Command::new("cargo")
                .arg("+nightly")
                .arg("build")
                .args(["-Z", "build-std=core,alloc"])
                .args(["--target", "riscv64gc-unknown-none-elf"])
                .args(["--manifest-path", "kernel/Cargo.toml"])
                .status();
            let _ = Command::new("rust-objcopy")
                .args([
                    "--binary-architecture=riscv64",
                    "target/riscv64gc-unknown-none-elf/debug/kernel",
                ])
                .args(["--strip-all", "-O", "binary"])
                .arg("target/riscv64gc-unknown-none-elf/debug/kernel.bin")
                .status();
        }
    }
    Ok(())
}
