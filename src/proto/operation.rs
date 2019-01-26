use super::Operation;

/// Buffer manager operation USB request number
const REQUEST_OPER: u8 = 7;

pub const SET_OPERATION: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_OPER,
    value: 0x0,
    length: 1
};
