use super::Operation;

/// SEGA USB request number
const REQUEST_N64: u8 = 15;

/// operand = A16-31 for next address latch, this merely updates a firmware variable
pub const N64_SET_BANK: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_N64,
    value: 2,
    length: 1
};

/// take ALE_L/H high to end the access
pub const N64_RELEASE_BUS: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_N64,
    value: 4,
    length: 1
};
