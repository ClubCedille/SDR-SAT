
use rtlsdr;
use std::vec::Vec;
use std::str;

pub fn get_devices() -> Result<Vec<rtlsdr::RTLSDRDevice>, &'static str >
{
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