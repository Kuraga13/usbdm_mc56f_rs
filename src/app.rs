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
use crate::gui::hexbuffer_widget::{TableContents,table_contents };
use crate::gui::{self, main_window};
use crate::gui::error_notify_modal::{error_notify_model};

#[derive(Debug, Clone)]
pub enum UsbdmAppStatus {
    
    NotConnected,
    Connected,
    //Errored(Error),
    Errored,

}

#[derive(Debug, Clone)]
pub enum TargetStatus {
    
    NotConnected,
    Connected,
    Errored,

}

#[derive(Debug, Clone)]
pub enum PowerStatus {
    
    PowerOn,
    PowerOff,

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
    
    OkButtonPressed,

    Connect,
    Disconnect,
    PowerSelect(TargetVddSelect),
    PowerToggle,
    TestFeedback,
    
}

pub struct App {

           target           : Option<Target>,
           buff             : Vec<HexBuffer>,
           buffer_view      : Vec<HexBufferView>,
    pub    selected_power   : TargetVddSelect,
    pub    status           : UsbdmAppStatus,
    pub    power_status     : PowerStatus,
    pub    target_status    : TargetStatus,
    pub    show_error_modal : bool,
    pub    error_status     : Option<Error>,

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



pub fn check_err(err: Error, app : &mut App)
{

    app.show_error_modal = true;
    app.error_status     = Some(err);
   

}


impl App 
{



 fn check_connect_programmer(&mut self)
 {
    let mut dsc =  self.target.as_mut().expect("Try to Connect to Opt:None Target!");

    let check_connect_usb =  dsc.programmer.refresh_feedback().unwrap_or_else( |err, |  check_err(err, self));
           
 }

 fn check_power(&mut self) -> Result<PowerStatus, Error>
 {
    
    let dsc =  self.target.as_mut().expect("");
    let power = dsc.programmer.check_power();
    let power_state =  PowerStatus::PowerOff;
    match power
    {
       Ok(true) =>
       {
           Ok(PowerStatus::PowerOff)
       }

       Ok(false) =>
       {
           Ok(PowerStatus::PowerOff)
       }

       (Err(_e)) =>
       {
          Err(_e)
       }
     } 

 }

 fn check_connect_programmer_and_target(&mut self)
 {
    let check_connect_usb =  self.target.as_mut().expect("Try to Connect to Opt:None Target!");

    check_connect_usb.programmer.refresh_feedback().unwrap_or_else( |err, |  check_err(err, self));
           
    let mut dsc =  self.target.as_mut().expect("Try to Connect to Opt:None Target!");
    let check_connect_dsc =  dsc.connect(self.selected_power).unwrap_or_else( |err, |  check_err(err,  self));

 }


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

                show_error_modal : false,
                error_status     : None,     
                selected_power   : TargetVddSelect::Vdd3V3,
                target           : None,
                status           : UsbdmAppStatus::NotConnected,
                power_status     : PowerStatus::PowerOff,
                target_status    : TargetStatus::NotConnected,
                buff             : vec![HexBuffer::new()],
                buffer_view      : vec![HexBufferView::default()],

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



            Message::OkButtonPressed =>
            {

              self.show_error_modal = false;

            } 

            

            Message::Connect => 
            {  

             match self.status
             {

                UsbdmAppStatus::NotConnected =>
                {
                    // check usb low level - two type errors : can't find VID and second cannot configure descriptor (bad drivers or hw error on usb port)
                    let check_connect = find_usbdm();
                    match check_connect
                    {
                       Ok(check_connect) =>
                       {
   
                       println!("Try claim usb & configure descriptors");
                       let usb_int = UsbInterface::new(check_connect);
                       match usb_int
                       {
                        Ok(usb_int) =>
                        {
                            let programmer = Programmer::new(usb_int);
                            
                            // init programmer, here we can get errors on get version, settings feedback etc.
                            match programmer
                            {
                                Ok(programmer) =>
                                {
                                
                                self.target  = Some(Target::init(Some(programmer).expect("Option: None")));
                                }

                                Err(_e) =>

                                {

                                check_err(_e,  self);
                                //self.error_nandler (_e, UsbdmAppStatus::NotConnected, TargetStatus::NotConnected, PowerStatus::PowerOff);
                                return iced::Command::none();

                                }
                            }
                        }
                        Err(_e) =>
                        {
                      
                            check_err(_e,  self);
                            //self.error_nandler (_e, UsbdmAppStatus::NotConnected, TargetStatus::NotConnected, PowerStatus::PowerOff);
                            return iced::Command::none();

                        }
                       }
                    
                       
                       let mut dsc =  self.target.as_mut().expect("target lost");
                       dsc.init();
                       
                       if let Err(_e) = dsc.connect(self.selected_power)
                       {
                        dsc.power(TargetVddSelect::VddOff);
                        check_err(_e, self);
                        //self.error_nandler (_e, UsbdmAppStatus::Connected, TargetStatus::NotConnected, PowerStatus::PowerOff);
                        return iced::Command::none();
                       }
                       else
                       {
                        self.status         = UsbdmAppStatus::Connected;
                        self.target_status  = TargetStatus::Connected;
                        self.power_status   =  PowerStatus::PowerOn //self.check_power(); TODO POWER STAYE FROM FUNC
                       }
                       
   
                       }
                       Err(_e) =>
                       {
                       
                       check_err(_e, self);
                       //self.error_nandler (_e, UsbdmAppStatus::NotConnected, TargetStatus::NotConnected, PowerStatus::PowerOff);
                       return iced::Command::none();
                 
            
                       }
                    }
                }
                UsbdmAppStatus::Connected => 
                {  
            
                 self.check_connect_programmer_and_target();
              
                }
    
                _ => 
                {
                    return iced::Command::none(); // we should never be here!!
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
                self.status  = UsbdmAppStatus::NotConnected; 
                
                }
                    
                None => {}
                } 
            } 

            Message::PowerToggle => 
            {    

             let status = self.check_power();
             
             match status
             {

                Ok(PowerStatus::PowerOn) =>
                {
                    let dsc =  self.target.as_mut().expect("");
                    dsc.power(TargetVddSelect::VddOff);
                    self.power_status  = PowerStatus::PowerOff;

                }

                Ok(PowerStatus::PowerOff) =>

                {
    
                    let dsc =  self.target.as_mut().expect("");
                    dsc.power(self.selected_power);
                    self.power_status  = PowerStatus::PowerOn;

                }

                (Err(_e)) =>
                {   
                    let dsc =  self.target.as_mut().expect("");
                    dsc.power(TargetVddSelect::VddOff);
                    //self.error_nandler (_e, UsbdmAppStatus::Connected, TargetStatus::NotConnected, PowerStatus::PowerOff);
                    check_err(_e, self);
                    return iced::Command::none();
                }

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
        
    
    let err_view = match self.error_status
    {
      Some(err)    =>  
      {
        err
      }  
      None    =>   
      {
        Error::Unknown
      } 

    };

    // this function captutre content in c and return
    // if not errors. if error - modal window on view
    error_notify_model(self.show_error_modal, main_page.into(), err_view) 


    
   // main_page.into()

    }

 
}


