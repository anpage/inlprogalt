use super::Operation;

/// Buffer USB request number
const REQUEST_BUFFER: u8 = 5;

pub const RAW_BANK_SIZE: u16 = 32;

pub const GENESIS_ROM_PAGE0: u16 = 0x28;
pub const GENESIS_ROM_PAGE1: u16 = 0x29;
pub const N64_ROM_PAGE: u16 = 0x30;
pub const MASKROM: u16 = 0xDD;

pub const NOVAR: u16 = 0;

pub const RESET: u16 = 0x01;
pub const DUMPING: u16 = 0xD0;
pub const STARTDUMP: u16 = 0xD2;
pub const DUMPED: u16 = 0xD8;

pub const RAW_BUFFER_RESET: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_BUFFER,
    value: 0x0,
    length: 1
};

pub const SET_MEM_N_PART: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_BUFFER,
    value: 0x30,
    length: 1
};

pub const SET_MAP_N_MAPVAR: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_BUFFER,
    value: 0x32,
    length: 1
};

pub const GET_CUR_BUFF_STATUS: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_BUFFER,
    value: 0x61,
    length: 3
};

pub const BUFF_PAYLOAD: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_BUFFER,
    value: 0x70,
    length: 128
};

pub const ALLOCATE_BUFFER0: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_BUFFER,
    value: 0x80,
    length: 1
};

pub const ALLOCATE_BUFFER1: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_BUFFER,
    value: 0x81,
    length: 1
};

pub const SET_RELOAD_PAGENUM0: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_BUFFER,
    value: 0x90,
    length: 1
};

pub const SET_RELOAD_PAGENUM1: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_BUFFER,
    value: 0x91,
    length: 1
};
