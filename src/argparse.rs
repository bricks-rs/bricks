use std::path::PathBuf;

use crate::Result;
use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub brick_type: BrickType,
}

#[derive(Debug, Subcommand)]
pub enum BrickType {
    #[command(about = "PoweredUp/RobotInventor/SpikePrime")]
    Pup {
        #[command(subcommand)]
        mode: PoweredUpMode,
    },
    #[command(about = "[todo]")]
    Ev3,
    #[command(about = "[todo]")]
    Nxt,
    #[command(about = "RCX")]
    Rcx {
        #[command(subcommand)]
        mode: RcxMode,
    },
}

#[derive(Debug, Subcommand)]
pub enum PoweredUpMode {
    #[command(about = "Scan for PoweredUp bricks")]
    Scan,
    #[command(about = "Download new program")]
    Download,
    #[command(about = "Load new firmeware onto brick")]
    Flash,
}

#[derive(Debug, Subcommand)]
pub enum RcxMode {
    #[command(about = "Check whether the RCX is reachable")]
    Ping,
    #[command(about = "Report ROM and FW versions")]
    Version,
    #[command(about = "Compile an NQC program to RCX bytecode")]
    Compile {
        #[clap(help = "Source file")]
        file: PathBuf,
    },
    #[command(about = "Download a program to the specified slot")]
    Program {
        #[clap(help = "Program slot (0-9)")]
        slot: u8,
        #[clap(help = "Program file")]
        file: PathBuf,
    },
    #[command(about = "Disassemble an RCX binary")]
    Disasm {
        #[clap(help = "Program file")]
        file: PathBuf,
    },
}

pub fn parse_args() -> Result<Args> {
    Ok(Args::parse())
}
