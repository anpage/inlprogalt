use super::proto;

pub fn allocate_buffers(handle: &libusb::DeviceHandle, num_buffers: u8, buff_size: u16) -> libusb::Result<()> {
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
