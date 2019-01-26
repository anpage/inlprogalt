//! Definitions for USB protocol

use super::usb;

pub mod io;
pub mod buffer;
pub mod operation;
pub mod sega;
pub mod n64;

/// Represents a possible USB request
///
/// # Arguments
///
/// * `direction` - direction of data transfer
/// * `request` - dictionary reference number
/// * `value` - opcode
/// * `length` - return data length
pub struct Operation {
    direction: libusb::Direction,
    request: u8,
    value: u16,
    length: u16,
}

impl Operation {
    pub fn call(&mut self, handle: &libusb::DeviceHandle, operand: u16, misc: u16) -> libusb::Result<Vec<u8>> {
        usb::vendor_transfer(handle, self.direction, self.request, misc<<8|self.value, operand, self.length)
    }
}
