extern crate libusb;

mod usb;
mod proto;

use std::fs::File;
use std::io::prelude::*;

fn main() -> libusb::Result<()>{
    let context = libusb::Context::new()?;

    let handle = usb::open_usb_device(&context)?;

    process_sega(&handle)?;

    Ok(())
}

fn process_sega(handle: &libusb::DeviceHandle) -> libusb::Result<()> {
    proto::io::IO_RESET.call(handle, 0, 0)?;
    proto::io::SEGA_INIT.call(handle, 0, 0)?;

    println!("Dumping SEGA ROM...");
    dump_sega_rom(handle)?;
    println!("DONE dumping SEGA ROM");

    proto::io::IO_RESET.call(handle, 0, 0)?;

    Ok(())
}

fn dump_sega_rom(handle: &libusb::DeviceHandle) -> libusb::Result<()> {
    const ROM_SIZE_KB: u16 = 4 * 128;

    const KB_PER_BANK: u16 = 128;
    const ADDR_BASE: u16 = 0x0000;

    const NUM_READS: u16 = ROM_SIZE_KB / KB_PER_BANK;

    let mut rom: Vec<u8> = Vec::new();

    for read_count in 0..NUM_READS {
        println!("Dumping ROM part {} of {}", read_count+1, NUM_READS);

        if read_count % 4 == 0 {
            println!("Dumping ROM bank {} of {}", read_count, NUM_READS-1);
        }

        proto::sega::SET_BANK.call(handle, read_count, 0)?;

        rom.append(&mut dump_rom(handle, KB_PER_BANK/2, ADDR_BASE, proto::buffer::GENESIS_ROM_PAGE0)?);
        rom.append(&mut dump_rom(handle, KB_PER_BANK/2, ADDR_BASE, proto::buffer::GENESIS_ROM_PAGE1)?);
    }

    let mut f = File::create("rom.bin").expect("Unable to create file");

    f.write_all(rom.as_slice()).expect("Unable to write data");

    Ok(())
}

fn dump_rom(handle: &libusb::DeviceHandle, size_kb: u16, map: u16, mem: u16) -> libusb::Result<Vec<u8>> {
    const BUFFER_SIZE: u16 = 128;

    proto::operation::SET_OPERATION.call(handle, proto::buffer::RESET, 0)?;
    proto::buffer::RAW_BUFFER_RESET.call(handle, 0, 0)?;

    println!("Allocating buffers");
    allocate(handle, 2, BUFFER_SIZE)?;

    println!("Setting map n part");
    proto::buffer::SET_MEM_N_PART.call(handle, mem<<8|proto::buffer::MASKROM, 0)?;
    proto::buffer::SET_MEM_N_PART.call(handle, mem<<8|proto::buffer::MASKROM, 1)?;

    println!("Setting map n mapvar");
    proto::buffer::SET_MAP_N_MAPVAR.call(handle, map<<8|proto::buffer::NOVAR, 0)?;
    proto::buffer::SET_MAP_N_MAPVAR.call(handle, map<<8|proto::buffer::NOVAR, 1)?;

    println!("Setting operation STARTDUMP");
    proto::operation::SET_OPERATION.call(handle, proto::buffer::STARTDUMP, 0)?;

    let mut out = Vec::new();

    println!("Starting first payload");
    for _ in 1..=size_kb as u32*1024/BUFFER_SIZE as u32 {
        let mut cur_buff_status = proto::buffer::GET_CUR_BUFF_STATUS.call(handle, 0, 0)?[2];
        while cur_buff_status as u16 != proto::buffer::DUMPED {
            println!("Buffer status: {}", cur_buff_status);
            cur_buff_status = proto::buffer::GET_CUR_BUFF_STATUS.call(handle, 0, 0)?[2];
        }
        out.append(&mut proto::buffer::BUFF_PAYLOAD.call(handle, 0, 0)?);
    }

    Ok(out)
}

fn allocate(handle: &libusb::DeviceHandle, num_buffers: u8, buff_size: u16) -> libusb::Result<()> {
    let buff_0_base_bank = 0;
    let numbanks = buff_size / (proto::buffer::RAW_BANK_SIZE);
    let buff_1_base_bank = numbanks;

    let buff_0_id;
    let buff_1_id;
    let reload;
    let buff_0_firstpage;
    let buff_1_firstpage;

    if num_buffers == 2 && buff_size == 128 {
        buff_0_id = 0x00;
        buff_1_id = 0x80;
        reload = 0x01;
        buff_0_firstpage = 0x0000;
        buff_1_firstpage = 0x0000;
    } else if num_buffers == 2 && buff_size == 256 {
        buff_0_id = 0x00;
		buff_1_id = 0x00;
		reload = 0x02;
		buff_0_firstpage = 0x0000;
		buff_1_firstpage = 0x0001;
    } else {
        return Err(libusb::Error::Other);
    }

    proto::buffer::ALLOCATE_BUFFER0.call(handle, (buff_0_id<<8)|buff_0_base_bank, numbanks)?;
    proto::buffer::ALLOCATE_BUFFER1.call(handle, (buff_1_id<<8)|buff_1_base_bank, numbanks)?;

    proto::buffer::SET_RELOAD_PAGENUM0.call(handle, buff_0_firstpage, reload)?;
    proto::buffer::SET_RELOAD_PAGENUM1.call(handle, buff_1_firstpage, reload)?;

    Ok(())
}
