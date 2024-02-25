use crate::Result;
use color_eyre::eyre::eyre;
use rcx::{tower::usb::UsbTower, Rcx, Sound};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

use rcx::binfmt::RcxBin;

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

    println!("{bin}");

    // Stop any running tasks
    rcx.stop_all_tasks()?;

    // Prepare RCX for download
    rcx.set_program_number(slot)?;

    // Delete existing tasks and subroutines
    rcx.delete_all_tasks()?;
    rcx.delete_all_subroutines()?;

    // Download the program chunks
    for (idx, section) in bin.sections.iter().enumerate() {
        println!(
            "[prog {}] Downloading section {} of {} to section number {}",
            slot,
            idx + 1,
            bin.sections.len(),
            section.number,
        );
        rcx.start_task_download(
            section.number,
            section.data.len().try_into()?,
        )?;

        for (idx, data_chunk) in section.data.chunks(256).enumerate() {
            let checksum = data_chunk
                .iter()
                .copied()
                .reduce(u8::wrapping_add)
                .unwrap_or_default();
            let idx = if (idx + 1) * 256 >= section.data.len() {
                // last block
                0
            } else {
                idx as i16 + 1
            };
            println!("Chunk {}, len {}", idx, data_chunk.len());
            rcx.start_task_download(
                section.number,
                data_chunk.len().try_into()?,
            )?;
            rcx.transfer_data(
                idx,
                data_chunk.len().try_into()?,
                data_chunk.to_vec(),
                checksum,
            )?;
        }
    }

    println!("Successfully downloaded {}", file.display());
    // Play the download successful sound
    rcx.play_sound(Sound::FastUpwardTones)?;
    Ok(())
}

pub fn disasm(file: PathBuf) -> Result<()> {
    // Read in the target binary
    let bin = std::fs::read(&file)?;
    let bin = RcxBin::parse(&bin)?;

    println!("{}", rcx::disasm::print(&file, &bin));
    Ok(())
}
