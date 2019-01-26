use super::proto;
use super::buffer;

pub fn dump_rom(handle: &libusb::DeviceHandle, size_kb: u16, map: u16, mem: u16) -> libusb::Result<Vec<u8>> {
    const BUFFER_SIZE: u16 = 128;

    proto::operation::SET_OPERATION.call(handle, proto::buffer::RESET, 0)?;
    proto::buffer::RAW_BUFFER_RESET.call(handle, 0, 0)?;

    println!("Allocating buffers");
    buffer::allocate_buffers(handle, 2, BUFFER_SIZE)?;

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
