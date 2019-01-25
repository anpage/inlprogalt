extern crate libusb;

mod usb;

fn main() -> libusb::Result<()>{
    let context = libusb::Context::new()?;

    usb::open_usb_device(&context)?;

    Ok(())
}
