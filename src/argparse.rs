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
    #[command(about = "[todo]")]
    Rcx,
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

pub fn parse_args() -> Result<Args> {
    Ok(Args::parse())
}
