use rtlsdr;
use std::vec::Vec;

mod rtlsdr_init;

fn main() {
    let ret = rtlsdr_init::init().unwrap();
}
