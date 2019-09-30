use rtlsdr;
use std::str;
use std::thread;
use std::vec::Vec;
/*
 * Opens the USB devices.
 * Returns a vector with all the device handles in them
 *
 */
fn _get_devices() -> Result<Vec<rtlsdr::RTLSDRDevice>, &'static str> {
    /* Get the total number of rtlsdr devices on the local machine */
    let number_of_devices = rtlsdr::get_device_count();

    /* this section could be replaced by a match*/
    if number_of_devices < 1 {
        return Err("No device found on this machine");
    }

    if number_of_devices == 1 {
        println!(
            "There is {} rtlsdr device(s) on this machine",
            number_of_devices
        );
    }

    /* open all devices and order them in a vector */
    let mut device_list: Vec<rtlsdr::RTLSDRDevice> = Vec::new();

    for i in 0..number_of_devices {
        println!(
            "Opening {} {}/{}",
            rtlsdr::get_device_name(i),
            i,
            number_of_devices
        );

        device_list.push(rtlsdr::open(i).unwrap());
    }
    Ok(device_list)
}

fn _get_descriptors() -> Result<Vec<rtlsdr::USBStrings>, &'static str> {
    /* Get the total number of rtlsdr devices on the local machine */
    let number_of_devices = rtlsdr::get_device_count();

    if number_of_devices < 1 {
        return Err("No device found on this machine");
    }
    /* Prepare the return vector */
    let mut _info_list: Vec<rtlsdr::USBStrings> = Vec::new();

    /* iterate through all the devices and get the description strings */
    for i in 0..number_of_devices {
        _info_list.push(rtlsdr::get_device_usb_strings(i).unwrap())
    }

    Ok(_info_list)
}

pub fn init() -> Result<(Vec<rtlsdr::RTLSDRDevice>, Vec<rtlsdr::USBStrings>), &'static str> {
    /* Get the total number of devices */

    let number_of_devices = rtlsdr::get_device_count();
    /* Fail if no devices were found */
    if number_of_devices < 1 {
        return Err("No device found on this machine");
    }
    let info_thread_handle = thread::spawn(move || _get_descriptors());

    Ok((
        _get_devices().unwrap(),
        info_thread_handle.join().unwrap().unwrap(),
    ))
}
