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
use crate::feedback::{PowerStatus};
use crate::programmer::{Programmer};
use crate::target::{MC56f80x,TargetFactory, TargetProgramming};
use crate::gui::{self, main_window};
use crate::gui::modal_notification::{error_notify_model, about_card};
use crate::gui::hexbuffer_widget::{TableContents, HexBuffer};

#[derive(Debug, Clone, PartialEq)]
pub enum UsbdmAppStatus {
    
    NotConnected,
    Connected,
    Errored,

}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetStatus {
    
    NotConnected,
    Connected,

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

    OpenGithub,
    OkButtonPressed,
    OpenAboutCard,
    CloseAboutCard,
    ConnectionImageOpen(bool),

    Connect,
    Disconnect,
    PowerSelect(TargetVddSelect),
    PowerToggle,
    ReadTarget,
    WriteTarget,
    TestFeedback,


    
}

pub struct App {

           target           : Option<MC56f80x>,
    pub    buffer           : HexBuffer,
    pub    selected_power   : TargetVddSelect,
    pub    status           : UsbdmAppStatus,
    pub    power_status     : PowerStatus,
    pub    target_status    : TargetStatus,
    pub    show_error_modal : bool,
    pub    about_card_open  : bool,
    pub    show_conn_image  : bool,
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

pub fn show_error( app : &mut App, _e : Error) 
{

    app.show_error_modal = true;
    app.error_status     = Some(_e);

  
}

pub fn ok_or_show_error<T, U>( app : &mut App, value : Result<T, U>) ->  Result<T, U>
where Error: From<U>, U: std::marker::Copy
{
  match value
  {
    Ok(value) => 
    {
        Ok(value)
    }
    Err(err) =>
    {
        app.show_error_modal = true;
        app.error_status     = Some(Error::from(err));
        Err(err)
    }
  }
}
   


impl App 
{


 fn check_connection_programmer(&mut self)
 {
    let mut dsc =  self.target.as_mut().expect("Try to Connect to Opt:None Target!");

    let mut check_connect_usb =  dsc.programmer.refresh_feedback();
    ok_or_show_error(self, check_connect_usb);
    match check_connect_usb
    {
        Ok(_) =>
        {
            self.status = UsbdmAppStatus::Connected;

        }


        Err(_) =>
        {
            self.status = UsbdmAppStatus::NotConnected;

        }
    }
           
 }

 

 fn check_connection_target(&mut self)
 {
    
    self.check_connection_programmer();

    if(self.status == UsbdmAppStatus::NotConnected)
    {
        return;
    }

    let mut dsc =  self.target.as_mut().expect("Try to Connect to Opt:None Target!");
    let mut check_connect_dsc =  dsc.connect(self.selected_power);
    ok_or_show_error(self, check_connect_dsc);
    match check_connect_dsc
    {

        Ok(_) =>
        {
            self.target_status = TargetStatus::Connected;

        }


        Err(_) =>
        {
            self.target_status = TargetStatus::NotConnected;

        }
    }

 }

 
 fn check_power_state(&mut self)
 {

    self.check_connection_programmer();

    if(self.status == UsbdmAppStatus::NotConnected)
    {
        return;
    }

    let mut dsc =  self.target.as_mut().expect("Try to Connect to Opt:None Target!");
    let mut power_state =  dsc.programmer.get_power_state();
    match power_state
    {

        Ok(power_state) =>
        {
            self.power_status = power_state;

        }


        Err(err) =>
        {
            show_error(  self, err);
            return;

        }
    }         
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
                
                buffer           : HexBuffer::default(),
                show_error_modal : false,
                show_conn_image  : false,
                about_card_open  : false,
                error_status     : None,     
                selected_power   : TargetVddSelect::Vdd3V3,
                target           : None,
                status           : UsbdmAppStatus::NotConnected,
                target_status    : TargetStatus::NotConnected,
                power_status     : PowerStatus::PowerOff,

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


            Message::OpenGithub => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer")
                    .arg("https://github.com/Kuraga13/usbdm_mc56f_rs")
                    .spawn()
                    .unwrap();
     
            }

            Message::ConnectionImageOpen(show) => {

                self.show_conn_image = show;
           
     
            }

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

            Message::CloseAboutCard | Message::OpenAboutCard => {
                self.about_card_open = !self.about_card_open;
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
                            // init programmer, here we can get errors on get version, settings feedback etc.

                            let programmer = Programmer::new(usb_int);     
                            match programmer
                            {
                                Ok(programmer) =>
                                {
                                
            
                                  let dsc:Option<MC56f80x>  = Some(MC56f80x::create_target( programmer, 0x7FFF, 0x000, "MC56f8035".to_string()));
                                  self.target = dsc;
                                
                                }

                                Err(_e) =>

                                {
                                show_error(  self, _e);
                                return iced::Command::none();

                                }
                            }
                        }
                        Err(_e) =>
                        {
                            show_error(  self, _e);
                            return iced::Command::none();

                        }
                       }
                    

                       let mut dsc =  self.target.as_mut().expect("target lost");
                       dsc.init();

                       self.check_connection_target();
                       
                       self.check_power_state();
                    
                    }
                    Err(_e) =>
                    {
                       show_error(  self, _e);
                       self.status = UsbdmAppStatus::NotConnected;
                       return iced::Command::none();
                 
                    }
                  }
                }



                UsbdmAppStatus::Connected => 
                {  

                    self.check_connection_target(); 
                    self.check_power_state();

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

            let dsc =  self.target.as_mut().expect("");
            self.check_power_state();

               match self.power_status
               {

                 PowerStatus::PowerOn =>
                 {
                    let dsc =  self.target.as_mut().expect("");
                    dsc.power(TargetVddSelect::VddOff);
                    self.check_power_state();

                 }

                 PowerStatus::PowerOff =>

                 {
    
                    let dsc =  self.target.as_mut().expect("");
                    dsc.power(self.selected_power);
                    self.check_power_state();

                 }
              }
            }   


            Message::PowerSelect(power_select)  => 
            {

              self.selected_power = power_select;

            }

            Message::ReadTarget  => 
            {
              
              let dsc = self.target.as_mut().expect("target lost");
              let read = dsc.read_target(self.selected_power);

              match read
              {
                Ok(read) => 
                {

                self.buffer.upload(read);
                self.check_power_state();

                }
                Err(_e) =>
                {

                show_error(self, _e);
                println!("ReadTarget error");
                self.check_power_state();
                return iced::Command::none();

               }
              }
            }

            Message::WriteTarget  => 
            {
              
              let dsc = self.target.as_mut().expect("target lost");
            

              let write = self.buffer.download_in_one();  
              let write_target = dsc.write_target(self.selected_power, write);

              match write_target
              {
                Ok(_) => 
                {
                self.check_power_state();
                println!("target write ok!");
    
                }
                Err(_e) =>
                {
                show_error(self, _e);
                self.check_power_state();
                println!("WriteTarget error");
                return iced::Command::none();
               }
              }
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
    //error_notify_model(self.show_error_modal, main_page.into(), err_view) 
    if self.about_card_open 
    {
        about_card(self.about_card_open, main_page.into())
     }
     else
     {
        error_notify_model(self.show_error_modal, main_page.into(), err_view) 
     }
    

   
    
   // main_page.into()

    }

 
}


