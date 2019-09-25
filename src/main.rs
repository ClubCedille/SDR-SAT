
use rtlsdr;

fn main() 
{
    let number_of_devices = rtlsdr::get_device_count();

    if number_of_devices < 1
    {
        println!("There is no rtlsdr device on this system.");
    }   

    if number_of_devices >= 1
    {
        println!("There is no rtlsdr device on this system.");
    }

    
}
