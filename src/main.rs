mod errors;
mod usb_interface;
mod enums;
mod feedback;
mod programmer;
mod hexbuffer;
mod app;
mod settings;


use std::vec;

use crate::hexbuffer::{HexBuffer};
use crate::app::{run};




pub fn main() -> iced::Result {
    
         let mut test = HexBuffer::default();
        // let v: Vec<u8>= vec![1, 0x2A, 3];
        // test.fill_buffer(&v,0x0);
         test.get_byte_in_address(0xFFDC).expect("Can't find address!");
        
         run()
        
}
