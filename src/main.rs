extern crate libusb;

mod usb;
mod proto;
mod buffer;
mod util;

mod genesis;

fn main() -> libusb::Result<()>{
    let context = libusb::Context::new()?;

    let handle = usb::open_usb_device(&context)?;

    genesis::process(&handle)?;

    Ok(())
}
