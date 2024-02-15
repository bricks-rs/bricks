use crate::Result;
use color_eyre::eyre::eyre;
use rcx::{tower::usb::UsbTower, Rcx};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

mod binfmt;

use binfmt::RcxBin;

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

pub fn compile(file: PathBuf) -> Result<()> {
    let target_dir = Path::new("target");
    let mut file_name = file.file_name().unwrap_or("out".as_ref()).to_owned();
    file_name.push(".rcx");

    let out_file = target_dir.join(file_name);

    let mut out_arg = OsString::from("-O");
    out_arg.push(&out_file);

    let output = Command::new("nqc").arg(&out_arg).arg(&file).output()?;
    if output.status.success() {
        println!(
            "Successfully compiled {} to {}",
            file.display(),
            out_file.display(),
        );
    } else {
        println!("Error compiling {}", file.display());
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

pub fn program(slot: u8, file: PathBuf) -> Result<()> {
    if slot > MAX_PROGRAM_SLOT {
        return Err(eyre!("Program slot must be 0-9"));
    }

    let rcx = UsbTower::open(DEVICE)?;
    let mut rcx = Rcx::new(rcx);

    // Read in the target binary
    let bin = std::fs::read(&file)?;
    let bin = RcxBin::parse(&bin)?;

    // Prepare RCX for download
    rcx.set_program_number(slot)?;

    // Download the program chunks
    for (idx, chunk) in bin.chunks.iter().enumerate() {
        println!(
            "Downloading chunk {} of {} to task {}",
            idx + 1,
            bin.chunks.len(),
            chunk.number
        );
        rcx.start_task_download(chunk.number, chunk.data.len().try_into()?)?;

        for (idx, data_chunk) in chunk.data.chunks(256).enumerate() {
            let mut buf = [0; 256];
            buf[..data_chunk.len()].copy_from_slice(data_chunk);
            let checksum = buf
                .iter()
                .copied()
                .reduce(u8::wrapping_add)
                .unwrap_or_default();
            let idx = if (idx + 1) * 256 >= chunk.data.len() {
                // last block
                0
            } else {
                idx as i16 + 1
            };
            rcx.transfer_data(
                idx,
                data_chunk.len().try_into()?,
                buf,
                checksum,
            )?;
        }
    }

    println!("Successfully downloaded {}", file.display());
    todo!()
}
