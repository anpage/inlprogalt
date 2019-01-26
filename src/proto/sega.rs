use super::Operation;

/// SEGA USB request number
const REQUEST_SEGA: u8 = 14;

pub const SET_BANK: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_SEGA,
    value: 2,
    length: 1
};
