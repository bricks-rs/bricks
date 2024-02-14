use crate::Result;
use color_eyre::eyre::eyre;
use rcx::{tower::usb::UsbTower, Rcx};
use std::path::PathBuf;

const MAX_PROGRAM_SLOT: u8 = 9;
const DEVICE: &str = "/dev/usb/legousbtower0";

pub fn ping() -> Result<()> {
    let rcx = UsbTower::open(DEVICE)?;
    let mut rcx = Rcx::new(rcx);
    rcx.alive()?;
    println!("RCX confirmed alive!");
    Ok(())
}

pub fn version() -> Result<()> {
    let rcx = UsbTower::open(DEVICE)?;
    let mut rcx = Rcx::new(rcx);
    let versions = rcx.get_versions()?;
    println!("RCX versions: {versions}");
    Ok(())
}

pub fn program(slot: u8, _file: PathBuf) -> Result<()> {
    if slot > MAX_PROGRAM_SLOT {
        return Err(eyre!("Program slot must be 0-9"));
    }
    todo!()
}
