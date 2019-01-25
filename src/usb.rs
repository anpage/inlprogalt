use std::time::Duration;

pub fn open_usb_device(context: &libusb::Context) -> libusb::Result<libusb::DeviceHandle> {
    let timeout = Duration::from_secs(1);
    const INL_VID: u16 = 0x16C0;
    const INL_PID: u16 = 0x05DC;
    const INL_MANF: &str = "InfiniteNesLives.com";
    const INL_PROD: &str = "INL Retro-Prog";
    const MIN_MAJ_VER: u8 = 2;

    let devices = context.devices()?;

    for device in devices.iter() {
        let device_desc = device.device_descriptor()?;

        if device_desc.vendor_id() == INL_VID && device_desc.product_id() == INL_PID  {
            let handle = device.open()?;
            let version = device_desc.device_version();
            let language = handle.read_languages(Duration::from_secs(1))?[0];
            let manufacturer = handle.read_manufacturer_string(language, &device_desc, timeout)?;
            let product = handle.read_product_string(language, &device_desc, timeout)?;

            if manufacturer == INL_MANF && product == INL_PROD {
                println!("Found INL Retro-Prog v{}.{}.{}",
                    version.major(),
                    version.minor(),
                    version.sub_minor());

                if version.major() >= MIN_MAJ_VER {
                    return Ok(handle)
                } else {
                    println!("Firmware version too old.")
                }
            }
        }
    }

    println!("Error communicating with programmer. Is it plugged in and the driver installed?");
    Err(libusb::Error::Other)
}
