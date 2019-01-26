//! # I/O operations
//! Scope of functions contained is intended to be general and generic not specific
//! to the cartridge inserted.  The closest these operations get to being cart/system
//! specific is in setup for a system.  Calling the cart/system setup contained here
//! prepares kazzo for system specific commands.  Once complete with system specifc
//! commands come back here to 'deinitialize' access to that cartridge.
//! these commands are meant to estabilish baseline rules of i/o to
//! support calling higher level system/cart specific functions.

use super::Operation;

/// I/O USB request number
const REQUEST_IO: u8 = 2;

pub const IO_RESET: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_IO,
    value: 0,
    length: 1
};

pub const SEGA_INIT: Operation = Operation {
    direction: libusb::Direction::In,
    request: REQUEST_IO,
    value: 7,
    length: 1
};
