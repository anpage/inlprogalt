use std::fs::File;
use std::io::prelude::*;

use super::proto;
use super::util;

pub fn process(handle: &libusb::DeviceHandle) -> libusb::Result<()> {
    proto::io::IO_RESET.call(handle, 0, 0)?;
    proto::io::N64_INIT.call(handle, 0, 0)?;

    println!("Dumping N64 ROM...");
    dump_rom(handle)?;
    println!("DONE dumping N64 ROM");

    proto::io::IO_RESET.call(handle, 0, 0)?;

    Ok(())
}

fn dump_rom(handle: &libusb::DeviceHandle) -> libusb::Result<()> {
    const ROM_SIZE_KB: u16 = 64 * 128;

    const KB_PER_BANK: u16 = 64;
    const ADDR_BASE: u16 = 0x0000;

    const BANK_BASE: u16 = 0x1000;

    const NUM_READS: u16 = ROM_SIZE_KB / KB_PER_BANK;

    let mut rom: Vec<u8> = Vec::new();

    for read_count in 0..NUM_READS {
        println!("Dumping ROM part {} of {}", read_count+1, NUM_READS);

        if read_count % 8 == 0 {
            println!("Dumping ROM bank {} of {}", read_count, NUM_READS-1);
        }

        proto::n64::N64_SET_BANK.call(handle, BANK_BASE+read_count, 0)?;

        rom.append(&mut util::dump_rom(handle, KB_PER_BANK, ADDR_BASE, proto::buffer::N64_ROM_PAGE)?);

        proto::n64::N64_RELEASE_BUS.call(handle, 0, 0)?;
    }

    let mut f = File::create("rom.bin").expect("Unable to create file");

    f.write_all(rom.as_slice()).expect("Unable to write data");

    Ok(())
}
