#![allow(unused)]

mod errors;
mod usb_interface;
mod enums;
mod feedback;
mod programmer;
mod hexbuffer;
mod styling;
mod app;
mod settings;
mod hexbuff_widget;

use std::vec;
use iced::window::Icon;
use iced::window;
use image::GenericImageView;
use iced::{ Application, Settings, };
use crate::app::{UsbdmApp};

pub fn main() -> iced::Result {
    
        
    let bytes = include_bytes!("resources/icon.png");
    let img = image::load_from_memory(bytes).unwrap();
    let img_dims = img.dimensions();
    let img_raw = img.into_rgba8().into_raw();


    let icon = window::Icon::from_rgba(img_raw, img_dims.0, img_dims.1).unwrap();

    let settings = Settings {
        window: window::Settings {
            size: (1024, 768),
            resizable: true,
            decorations: true,
            min_size: Some((800, 600)),
            max_size: None,
            transparent: false,
            always_on_top: false,
            icon: Some(icon),
            visible: true,
            position: Default::default(),
          
        },
        antialiasing: true,
        ..Default::default()
    };

    UsbdmApp::run(settings)
        
}
