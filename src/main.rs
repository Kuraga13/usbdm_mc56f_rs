#![windows_subsystem = "windows"]

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
    

  
        
         run()
        
}
