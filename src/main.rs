mod errors;
mod usb_interface;
mod enums;
mod feedback;
mod programmer;
mod app;


use crate::app::{run};




pub fn main() -> iced::Result {
    
         run()
    
}



