use libusb::{Context, Device, DeviceHandle};
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct USBError {
    err: String,
}

impl From<libusb::Error> for USBError {
    fn from(_e: libusb::Error) -> Self
    {
        USBError {
            err: "Error in accessing USB Device!".to_string(),
        }
    }
}

impl From<std::io::Error> for USBError {
    fn from(e: std::io::Error) -> Self
    {
        USBError {
            err: e.to_string(),
        }
    }
}

struct USBList {
    list: Vec<USBDetails>,
}

#[derive(Debug)]
struct USBDetails {
    manufacturer:       String,
    product:            String,
    serial_number:      String,
    bus_number:         u8,
    device_address:     u8,
    vendor_id:          u16,
    product_id:         u16,
    maj_device_version: u8,
    min_device_version: u8,
}

impl fmt::Display for USBList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        for usb_device in &self.list {
            writeln!(f, "\nUSB Device details")?;
            writeln!(f, "Manufacturer: {}", usb_device.manufacturer)?;
            writeln!(f, "Product: {}", usb_device.product)?;
            writeln!(f, "Serial number: {}", usb_device.serial_number)?;
            writeln!(f, "Bus number: {}", usb_device.bus_number)?;
            writeln!(f, "Device address: {}", usb_device.device_address)?;
            writeln!(f, "Vendor Id: {}", usb_device.vendor_id)?;
            writeln!(f, "Product Id: {}", usb_device.product_id)?;
            writeln!(f, "Major device version: {}", usb_device.maj_device_version)?;
            writeln!(f, "Minor device version: {}", usb_device.min_device_version)?;
        }

        Ok(())
    }
}

fn write_to_file(usb: USBList) -> Result<(), USBError>
{
    let mut file_handle = File::create("usb_details.txt")?;
    write!(file_handle, "{}\n", usb)?;
    Ok(())
}

fn get_device_information(device: Device, device_handle: &DeviceHandle) 
    -> Result<USBDetails, USBError>
{
    let device_descriptor = device.device_descriptor()?;

    let timeout = Duration::from_secs(1);
    let languages = device_handle.read_languages(timeout)?;
    let language = languages[0];

    let manufacturer = device_handle
        .read_manufacturer_string(language, &device_descriptor, timeout)?;

    let product = device_handle.read_product_string(language, &device_descriptor, timeout)?;

    let product_serial_number = 
        match device_handle.read_serial_number_string(
            language, &device_descriptor, timeout) {
                Ok(serial_number) => serial_number,
                Err(_) => "Not available".to_string(),
    };

    Ok(USBDetails {
        manufacturer,
        product,
        serial_number: product_serial_number,
        bus_number: device.bus_number(),
        device_address: device.address(),
        vendor_id: device_descriptor.vendor_id(),
        product_id: device_descriptor.product_id(),
        maj_device_version:
            device_descriptor.device_version().0,
        min_device_version:
            device_descriptor.device_version().1,
    })
}
fn main() -> Result<(), USBError> {
    let context = Context::new()?;

    let mut device_list = USBList {
        list: vec![],
    };

    for device in context.devices()?.iter() {
        let device_descriptor = device.device_descriptor()?;

        let device_handle = context.open_device_with_vid_pid(
                    device_descriptor.vendor_id(), 
                    device_descriptor.product_id())
                    .unwrap();

        let usb_details = get_device_information(device, &device_handle)?;

        device_list.list.push(usb_details);
    }

    println!("\n{}", device_list);
    write_to_file(device_list)?;
    Ok(())
}
