use crate::Result;
use rcx::{tower::usb::UsbTower, Rcx};

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
