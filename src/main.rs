
use rtlsdr;
use std::vec::Vec;
use std::str;

mod rtlsdrRoutines;

fn main() {
    
    match rtlsdrRoutines::get_devices() {
        Ok(_list) => println!("Device found !"),
        Err(e) => println!("{}", e),
    }
}
