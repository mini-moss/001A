use std::fs::File;
use std::io::Write;
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
    #[clap(about = "Run gdbserver")]
    GdbServer,
    #[clap(about = "Run gdbclient")]
    GdbClient,
    #[clap(about = "Run objdump")]
    Objdump,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Bin => {
            let _ = Command::new("cargo").arg("clean").status();
            let _ = Command::new("cargo")
                .env("RUSTFLAGS", "-Clink-arg=-Tkernel/arch/riscv/linker.ld -Cforce-frame-pointers=yes")
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
        },
        Commands::Qemu => {
            let _ = Command::new("qemu-system-riscv64")
                .args(["-M", "128m"])
                .args(["-machine", "virt"])
                .args(["-nographic"])
                .args(["-bios", "default"])
                .args(["-kernel", "target/riscv64gc-unknown-none-elf/debug/kernel.bin"])
                .status();
        }
        Commands::GdbServer => {
            unimplemented!();
            let _ = Command::new("qemu-system-riscv64")
                .args(["-M", "128m"])
                .args(["-machine", "virt"])
                .args(["-nographic"])
                .args(["-bios", "default"])
                .args(["-kernel", "target/riscv64gc-unknown-none-elf/debug/kernel.bin"])
                .args(["-s", "-S"])
                .status();
        },
        Commands::GdbClient => {
            unimplemented!();
            let _ = Command::new("gdb")
                .args(["-ex", "file target/riscv64gc-unknown-none-elf/debug/kernel"])
                .args(["-ex", "set arch riscv:rv64"])
                .args(["-ex", "target remote localhost:1234"])
                .status();
        },
        Commands::Objdump => {
            let output_path = "kernel.S";
            let output = Command::new("riscv64-unknown-elf-objdump")
                .args(["-d", "target/riscv64gc-unknown-none-elf/debug/kernel"])
                .output();
            if let Ok(output) = output {
                if output.status.success() {
                    let file = File::create(output_path);
                    file?.write_all(output.stdout.as_slice())?;
                }
            }
        }
    }
    Ok(())
}
