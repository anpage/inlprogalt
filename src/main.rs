extern crate libusb;

mod usb;
mod proto;
mod buffer;
mod util;

mod genesis;
mod n64;

fn main() -> libusb::Result<()>{
    let context = libusb::Context::new()?;

    let handle = usb::open_usb_device(&context)?;

    n64::process(&handle)?;

    Ok(())
}
