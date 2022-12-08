use color_eyre::Result;

mod argparse;
mod powered_up;

use argparse::{BrickType, PoweredUpMode};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = argparse::parse_args()?;

    match args.brick_type {
        BrickType::Pup { mode } => match mode {
            PoweredUpMode::Scan => powered_up::scan::run().await,
            _ => todo!(),
        },
        argparse::BrickType::Ev3
        | argparse::BrickType::Nxt
        | argparse::BrickType::Rcx => todo!(),
    }
}
