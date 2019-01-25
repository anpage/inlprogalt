use std::time::Duration;

static TIMEOUT: Duration = Duration::from_secs(1);

const MAX_VUSB: u16 = 254;

const INL_VID: u16 = 0x16C0;
const INL_PID: u16 = 0x05DC;
const INL_MANF: &str = "InfiniteNesLives.com";
const INL_PROD: &str = "INL Retro-Prog";
const MIN_MAJ_VER: u8 = 2;

pub fn open_usb_device(context: &libusb::Context) -> libusb::Result<libusb::DeviceHandle> {
    for device in context.devices()?.iter() {
        let device_desc = device.device_descriptor()?;

        if device_desc.vendor_id() == INL_VID && device_desc.product_id() == INL_PID {
            let handle = device.open()?;
            let version = device_desc.device_version();
            let language = handle.read_languages(Duration::from_secs(1))?[0];
            let manufacturer = handle.read_manufacturer_string(language, &device_desc, TIMEOUT)?;
            let product = handle.read_product_string(language, &device_desc, TIMEOUT)?;

            if manufacturer == INL_MANF && product == INL_PROD {
                println!(
                    "Found INL Retro-Prog v{}.{}.{}",
                    version.major(),
                    version.minor(),
                    version.sub_minor()
                );

                if version.major() >= MIN_MAJ_VER {
                    return Ok(handle);
                } else {
                    println!("Firmware version too old.")
                }
            }
        }
    }

    println!("Error communicating with programmer. Is it plugged in and the driver installed?");
    Err(libusb::Error::Other)
}

#[allow(unused)]
fn usb_vendor_transfer(
    handle: libusb::DeviceHandle,
    direction: libusb::Direction,
    request: u8,
    value: u16,
    index: u16,
    length: u16,
) -> libusb::Result<Vec<u8>> {
    assert!(
        length as u16 <= MAX_VUSB,
        "Can't transfer more than {} bytes!",
        MAX_VUSB
    );

    let mut vec = Vec::<u8>::with_capacity(length as usize);
    let buf =
        unsafe { std::slice::from_raw_parts_mut((&mut vec[..]).as_mut_ptr(), vec.capacity()) };

    match direction {
        libusb::Direction::In => match handle.read_control(
            libusb::request_type(
                direction,
                libusb::RequestType::Vendor,
                libusb::Recipient::Device,
            ),
            request,
            value,
            index,
            buf,
            TIMEOUT,
        ) {
            Ok(len) => {
                unsafe { vec.set_len(len) };
            }
            Err(e) => return Err(e),
        },
        libusb::Direction::Out => match handle.write_control(
            libusb::request_type(
                direction,
                libusb::RequestType::Vendor,
                libusb::Recipient::Device,
            ),
            request,
            value,
            index,
            buf,
            TIMEOUT,
        ) {
            Ok(len) => {
                unsafe { vec.set_len(len) };
            }
            Err(e) => return Err(e),
        },
    }

    return Ok(vec);
}
