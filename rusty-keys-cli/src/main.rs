use std::{io::Write, os::unix::net::UnixStream, time::SystemTime};
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    socket: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Delete {
        #[arg(short, long)]
        pid: u32,
    },
    Monitor {
        #[arg(short, long)]
        pid: u32,
    },
    Logs {
        #[arg(short, long)]
        pid: u32,
    },
    Import {
        #[arg(short, long)]
        file: String, 
    }
}


fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Delete {pid} ) => {
        },

        _ => {
            todo!() 
        }
    }
}
