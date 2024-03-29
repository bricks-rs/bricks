use color_eyre::Result;

// #[macro_use]
// extern crate tracing;

mod argparse;
mod powered_up;
mod rcx;

use argparse::{BrickType, PoweredUpMode, RcxMode};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = argparse::parse_args()?;

    match args.brick_type {
        BrickType::Pup { mode } => match mode {
            PoweredUpMode::Scan => powered_up::scan::run().await,
            _ => todo!(),
        },
        argparse::BrickType::Ev3 | argparse::BrickType::Nxt => todo!(),
        argparse::BrickType::Rcx { mode } => match mode {
            RcxMode::Ping => rcx::ping(),
            RcxMode::Version => rcx::version(),
            RcxMode::Compile { file } => rcx::compile(file),
            RcxMode::Program { slot, file } => rcx::program(slot, file),
            RcxMode::Disasm { file } => rcx::disasm(file),
        },
    }
}
