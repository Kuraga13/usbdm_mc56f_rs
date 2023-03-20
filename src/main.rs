#![allow(unused)]
//#![windows_subsystem = "windows"]

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

mod gui;
mod app;
mod errors;
mod usbdm;
mod dsc_target;
mod file_buffer;

use std::vec;
use iced::window::Icon;
use iced::window;
use image::GenericImageView;
use iced::{ Application, Settings, };
use crate::app::{App};


pub fn main() -> iced::Result {
    
        
    let bytes = include_bytes!("gui/resources/icon.png");
    let img = image::load_from_memory(bytes).unwrap();
    let img_dims = img.dimensions();
    let img_raw = img.into_rgba8().into_raw();


    let icon = window::Icon::from_rgba(img_raw, img_dims.0, img_dims.1).unwrap();

    let settings = Settings {
        default_text_size: 15.0,
        window: iced::window::Settings{
            size: (800, 500),
            resizable: true,
            decorations: true,
            min_size: Some((800, 600)),
            max_size: None,
            transparent: false,
            always_on_top: false,
            icon: Some(icon),
            visible: true,
            ..Default::default()
            //position: Default::default(),
        },
        //antialiasing: true,
        ..Default::default()
    };

    App::run(settings)
 
        
}
