use iced::widget::{column as col, Column};
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,
};
use iced::{alignment, theme, Application, Color, Element, Length};

use iced_aw::menu::{ItemHeight, ItemWidth, MenuBar, MenuTree, PathHighlight};
use iced_aw::quad;


use crate::usb_interface::{UsbInterface, find_usbdm_as, find_usbdm,};
use crate::errors::{Error};
use crate::settings::{TargetVddSelect};
use crate::programmer::{Programmer};
use crate::jtag::{JtagInterface};
use crate::target::{Target};
use crate::gui::hexbuff_widget::{HexBufferView, HexBuffer, };
use crate::gui::{self, main_window};

#[derive(Debug, Clone)]
pub enum UsbdmAppStatus {
    
    Start,
    Connected,
    //Errored(Error),
    Errored,

}



#[derive(Debug, Clone)]
pub enum Message {
    Debug(String),
    ValueChange(u8),
    CheckChange(bool),
    ToggleChange(bool),
    ColorChange(Color),
    Flip,
    ThemeChange(bool),
    TextChange(String),
    SizeOption(main_window::SizeOption),

    Init,
    Start,
    Connect,
    Disconnect,
    PowerSelect(TargetVddSelect),
    SetPower,
    TestFeedback,
}

pub struct App {


           programmer     : Option<Programmer>,
           buff           : Vec<HexBuffer>,
           buffer_view    : Vec<HexBufferView>,
    pub    selected_power : TargetVddSelect,
    pub    status         : UsbdmAppStatus,
    pub    title: String,
    pub    value: u8,
    pub    check: bool,
    pub    toggle: bool,
    pub    theme: iced::Theme,
    pub    flip: bool,
    pub    dark_mode: bool,
    pub    text: String,
    pub    size_option: main_window::SizeOption,
}
impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {


        let theme = iced::Theme::custom(theme::Palette {
            primary: Color::from([0.23, 0.61, 0.81]),
            danger: Color::from([1.0, 0.00, 0.00]),
            success: Color::from([0.70, 0.75, 0.02]),
            ..iced::Theme::Light.palette()
        });

        (           
            Self {
                title: "Usbdm_rs".to_string(),
                value: 0,
                check: false,
                toggle: false,
                theme,
                flip: false,
                dark_mode: false,
                text: "Text Input".into(),
                size_option: main_window::SizeOption::Static,


                selected_power : TargetVddSelect::VddOff,
                programmer     : None,
                status         : UsbdmAppStatus::Start,
                buff           : vec![HexBuffer::new()],
                buffer_view    : vec![HexBufferView::default()],
            },
            iced::Command::none(),
        )
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Debug(s) => {
                self.title = s.clone();
            }
            Message::ValueChange(v) => {
                self.value = v;
                self.title = v.to_string();
            }
            Message::CheckChange(c) => {
                self.check = c;
                self.title = c.to_string();
            }
            Message::ToggleChange(t) => {
                self.toggle = t;
                self.title = t.to_string();
            }
            Message::ColorChange(c) => {
                self.theme = iced::Theme::custom(theme::Palette {
                    primary: c,
                    ..self.theme.palette()
                });
                self.title = format!("[{:.2}, {:.2}, {:.2}]", c.r, c.g, c.b);
            }
            Message::Flip => self.flip = !self.flip,
            Message::ThemeChange(b) => {
                self.dark_mode = b;
                let primary = self.theme.palette().primary;
                if b {
                    self.theme = iced::Theme::custom(theme::Palette {
                        primary,
                        ..iced::Theme::Dark.palette()
                    })
                } else {
                    self.theme = iced::Theme::custom(theme::Palette {
                        primary,
                        ..iced::Theme::Light.palette()
                    })
                }
            }
            Message::TextChange(s) => {
                self.text = s.clone();
                self.title = s;
            }
            Message::SizeOption(so) => {
                self.size_option = so;
                self.title = self.size_option.to_string();
            }

            Message::Init => 
            {
                
                self.status  = UsbdmAppStatus::Start;

            }

            Message::Start => 
            {
                
            

            }
            

            Message::Connect => 
            {    
              let check_connect = find_usbdm();

              match check_connect
                 {
                    Ok(check_connect) =>
                    {

                    println!("Try claim usb");
                    let usb_int = UsbInterface::new(check_connect).expect("Programmer Lost Connection");
                    self.programmer = Some(Programmer::new(usb_int));
                    self.status  = UsbdmAppStatus::Connected;


                    }
                    Err(_e) =>
                    {
                    dbg!("Programmer not connected");
                    self.status = UsbdmAppStatus::Errored;
                    self.title =  String::from("Programmer not found! Check connection").clone();
                    }
                 }
            } 

            Message::Disconnect => 
            {    
            
                match &self.programmer
                {   
                Some(_programmer) =>{ 
                println!("Try disconnect and drop");
                drop(&mut self.programmer);
                self.programmer = None;
                self.status  = UsbdmAppStatus::Start; 
                
                }
                    
                None => {}
                } 
                
            } 


            Message::SetPower => 
            {    
            
             let usbdm =  self.programmer.as_mut().expect("");
             match &self.selected_power{   

                TargetVddSelect::VddOff     => {if let Err(_e) =  usbdm.set_vdd_off()  {} else {}}
                TargetVddSelect::Vdd3V3     => {if let Err(_e) =  usbdm.set_vdd_3_3v() {} else {}}
                TargetVddSelect::Vdd5V      => {if let Err(_e) =  usbdm.set_vdd_5v()   {} else {}}
                TargetVddSelect::VddEnable  => {if let Err(_e) =  usbdm.set_vdd_off()  {} else {}}
                TargetVddSelect::VddDisable => {if let Err(_e) =  usbdm.set_vdd_off()  {} else {}}}

            }   


            Message::PowerSelect(power_select) => 
            {

                self.selected_power = power_select;

            }


            Message::TestFeedback =>
            {
               
               let usbdm =  self.programmer.as_mut().expect("");
               if let Err(_e) = usbdm.refresh_feedback() {  };
               usbdm.set_bdm_options();
               usbdm.set_target_mc56f();
               //let jtag = jtag.init(usbdm);
               //let target = init(jtag);
               //target.dsc_connect();
    
            } 

        }
        iced::Command::none()
    }





    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
    
    let main_page = main_window::main_page(self);

    main_page.into()

    }

 
}


