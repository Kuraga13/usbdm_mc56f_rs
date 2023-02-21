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
use crate::target::{Target};
use crate::target::TargetProgramming;
use crate::gui::hexbuff_widget::{HexBufferView, HexBuffer, };
use crate::gui::{self, main_window};

#[derive(Debug, Clone)]
pub enum UsbdmAppStatus {
    
    Start,
    ConnectedPowerOn,
    ConnectedPowerOff,
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
    PowerToggle,
    TestFeedback,
    Error(Error)
}

pub struct App {

           target         : Option<Target>,
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


                selected_power : TargetVddSelect::Vdd3V3,
                target         : None,
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

            Message::Error(Error) =>
            {



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
                    let programmer = Some(Programmer::new(usb_int));
                    self.target  = Some(Target::init(programmer.expect("Programmer Lost Connection")));
                    self.target.as_mut().expect("target lost").init().unwrap_or(self.status = UsbdmAppStatus::Errored);
                    self.target.as_mut().expect("target lost").connect(self.selected_power).unwrap_or(self.status = UsbdmAppStatus::Errored);
                    self.status  = UsbdmAppStatus::ConnectedPowerOn;

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
                match &self.target
                {   
                Some(target) =>{ 
                println!("Try disconnect and drop");
            
                self.target.as_mut().expect("target lost").disconnect();
                self.target = None;
                self.status  = UsbdmAppStatus::Start; 
                
                }
                    
                None => {}
                } 
            } 

            Message::PowerToggle => 
            {    

             let mcu =  self.target.as_mut().expect("");
             let is_power = mcu.programmer.check_power().expect("Err on check power!");

              if(is_power)
              { 
                dbg!("Try Power off, from state now: {}", &is_power);
                mcu.power(TargetVddSelect::VddOff);
                self.status  = UsbdmAppStatus::ConnectedPowerOff;
              }
              else
              { 
                dbg!("Try Power On, from state now: {}", &is_power);
                mcu.power(self.selected_power);
                self.status  = UsbdmAppStatus::ConnectedPowerOn;
              } 

            }   


            Message::PowerSelect(power_select) => 
            {

              self.selected_power = power_select;

            }


            Message::TestFeedback =>
            {
                println!("Reset");
                self.target.as_mut().expect("target lost").programmer.target_power_reset();
            
              //let mcu =  self.target.as_mut().expect("");
              //mcu.power(TargetVddSelect::VddOff);
              //if let Err(_e) = mcu.programmer.refresh_feedback() { self.status = UsbdmAppStatus::Errored};
             // mcu.power(TargetVddSelect::VddDisable);
              //mcu.programmer.set_bdm_options();
              // usbdm.programmer.set_bdm_options();
              // usbdm.programmer.set_target_mc56f();
              // usbdm.dsc_connect().expect("Dsc target connect error");
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


