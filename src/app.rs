use iced::widget::{column as col, Column};
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,
};
use iced::{alignment, theme, Application, Color, Element, Length, Subscription};

use iced_aw::menu::{ItemHeight, ItemWidth, MenuBar, MenuTree, PathHighlight};
use iced_aw::quad;

use std::ffi::{OsStr, OsString};
use std::path::Path;
use native_dialog::{FileDialog, MessageDialog, MessageType};


use crate::usbdm::usb_interface::{UsbInterface, find_usbdm_as, find_usbdm,};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use crate::usbdm::programmer::{Programmer};
use crate::target_dsc::target_factory::{TargetFactory, TargetProgramming};
use crate::target_dsc::mc56f80x::MC56f80x;
use crate::gui::{self, main_window};
use crate::gui::modal_notification::{error_notify_model, about_card, connection_image_modal, progress_bar_modal};
use crate::gui::hexbuffer_widget::{TableContents, HexBuffer};
use crate::file_buffer::hex_file::{load_buffer_from_file, save_buffer_to_file, FileFormat};
use crate::errors::{Error};

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
    InProgramming,
    EndProgramming,

}
// just for test! In work verstion need implement async trait in programmer + subscribe method on iced app side
pub async fn handle_progress(val : f32) -> f32
{
    val
}


#[derive(Debug, Clone)]
pub enum Message {

    Menu,
    
    TargetProgramminEnd,
    ColorChange(Color),  
    ThemeChange(bool), 
    TestBufferDoubleClick,
    OpenGithub,
    OpenFile,
    SaveFile,


    OkButtonPressed,
    OpenAboutCard,
    CloseAboutCard,
 
    ConnectionImageOpen(bool),
    Connect,
    Disconnect,
    PowerSelect(TargetVddSelect),
    PowerToggle,
    ReadTarget,
    ProgrammingInProgress(f32),
    ReadTargetProgress(f32),
    WriteTarget,
    TestFeedback,
    
}

pub struct App {

           target             : Option<MC56f80x>,
    pub    buffer             : HexBuffer,
           buffer_path        : String,
    pub    selected_power     : TargetVddSelect,
    pub    status             : UsbdmAppStatus,
    pub    power_status       : PowerStatus,
    pub    target_status      : TargetStatus,
    pub    show_error_modal   : bool,
    pub    about_card_open    : bool,
    pub    show_conn_image    : bool,
    pub    show_p_progress    : bool,
    pub    error_status       : Option<Error>,
    pub    theme              : iced::Theme,
    pub    dark_mode          : bool,
    pub    progress_bar_value : f32,
    
    pub    number_to_read     : u32,            // for debug
    pub    counter            : u32,            // for debug
           read_buff          : Vec<Vec<u8>>,   // for debug
           read_adddress      : u32,            // for debug


}

pub fn show_error( app : &mut App, _e : Error) 
{

    app.show_error_modal = true;
    app.error_status     = Some(_e);

  
}

pub fn ok_or_show_error<T, U>( app : &mut App, value : Result<T, U>) ->  Result<T, U>
where Error: From<U>, U: std::clone::Clone
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
        app.error_status     = Some(Error::from(err.clone()));
        Err(err.clone())
    }
  }
}
   


impl App 
{
    /// Open the file dialog to select a file
    ///
    /// # Example
    ///
    /// ```rust
    /// let selected_file_result: Result<Option<String>, OsString> = open_file_dialog()
    /// ```
    ///
    /// # Returns
    ///
    /// The optional `String` that contains the path of the selected file or an `OsString` error
    fn open_file_dialog() -> Result<Option<String>, OsString> {
        let path = FileDialog::new()
            .add_filter(".bin", &["bin"])
            .add_filter(".s19_usbdm_format", &["s19"])
            //.add_filter("All files", &["*"])
            .show_open_single_file()
            .unwrap();

        let path = match path {
            Some(path) => path,
            None => return Ok(None),
        };

        match path.into_os_string().into_string() {
            Ok(d) => Ok(Some(d)),
            Err(e) => Err(e),
        }
    }

      fn save_file_dialog() -> Result<Option<String>, OsString> {
        
       let path  =  FileDialog::new();

       let path_configured = 
        path
        .add_filter(".bin", &["bin"])
        .add_filter(".s19_usbdm_format", &["s19"])
        .show_save_single_file()
        .unwrap();



        let path_configured = match path_configured {
            Some(path_configured) => path_configured,
            None => return Ok(None),
        };
    

        match path_configured.into_os_string().into_string() {
            Ok(d) => Ok(Some(d)),
            Err(e) => Err(e),
        }
    }
    

     /// Display a native alert
    ///
    /// # Example
    ///
    /// ```rust
    /// display_alert("hello", "world", MessageType::Info)
    /// ```
    ///
    /// # Arguments
    ///
    /// * `title` - The alert title
    /// * `content` - the content of the alert
    /// * `message_type` - The `MessageType` for the alert
    fn display_alert(&self, title: &str, content: &str, message_type: MessageType) {
        MessageDialog::new()
            .set_type(message_type)
            .set_title(title)
            .set_text(content)
            .show_alert()
            .unwrap();
    }

  fn check_connection_programmer(&mut self)
  {
    let mut dsc =  self.target.as_mut().expect("Try to Connect to Opt:None Target!");

    let mut check_connect_usb =  dsc.programmer.refresh_feedback();
    ok_or_show_error(self, check_connect_usb.clone());
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
    ok_or_show_error(self, check_connect_dsc.clone());
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
         
                theme,
                dark_mode          : false,  
                buffer             : HexBuffer::default(),
                buffer_path        : "".to_string(),
                show_error_modal   : false,
                show_conn_image    : false,
                about_card_open    : false,
                show_p_progress    : false,
                error_status       : None,     
                selected_power     : TargetVddSelect::Vdd3V3,
                target             : None,
                status             : UsbdmAppStatus::NotConnected,
                target_status      : TargetStatus::NotConnected,
                power_status       : PowerStatus::PowerOff,
                progress_bar_value : 0.0,
                
                number_to_read     : 0,
                counter            : 0,
                read_buff          : vec![vec![0;0]],
                read_adddress      : 0,

            },
            iced::Command::none(),
        )
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
       "Usbdm_rs".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {

          
            Message::ProgrammingInProgress(x) => 
            {
                
                self.progress_bar_value = x;
                return iced::Command::perform(handle_progress(x), Message::ReadTargetProgress);

            }
            
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

            Message::Menu => {}

  
            Message::ColorChange(c) => {
                self.theme = iced::Theme::custom(theme::Palette {
                    primary: c,
                    ..self.theme.palette()
                });
        
            }

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
    
         

            Message::TestBufferDoubleClick =>
            {

            } 

            Message::OpenFile => 
            {

            let path = match App::open_file_dialog() {
            
            Ok(res) => match res {
                            Some(d) => d,
                            None => return iced::Command::none(),},

            Err(e) => {
                App::display_alert(
                                &self,
                                "usbdm_mc56f_rs",
                                &format!("Error while selecting file!\n{:?}", e),
                                MessageType::Error,);
                                return iced::Command::none();  }  };
    
           

            let mut start_addr : u32;
            let mut size : usize;
    
              match &self.target 
              {
                None => 
                {

                start_addr  = 0x0;
                size  = 0xFFFF;

                }
                Some(dsc) =>
                {

                 start_addr = dsc.memory_map.memory_start_address();
                 size = dsc.memory_map.memory_size();

                 }
              }

            self.buffer_path = path;

            let result = load_buffer_from_file(self.buffer_path.clone(), start_addr, size,self);
             match result
             {
                Ok(_) =>
                {
                return iced::Command::none();
                }
                Err(_e) =>
                {
                show_error(  self, _e);
                return iced::Command::none();
                }
             }

            }

            Message::SaveFile => 
            {

             let path = match App::save_file_dialog() {
            
                    Ok(res) => match res {
                                    Some(d) => d,
                                    None => return iced::Command::none(),},
        
                    Err(e) => {
                        App::display_alert(
                                        &self,
                                        "usbdm_mc56f_rs",
                                        &format!("Error while save file!\n{:?}", e),
                                        MessageType::Error,);
                                        return iced::Command::none();  }  };
            
                   
        
            let mut start_addr : u32;
            let mut size : usize;
            
             match &self.target 
             {
                None => 
                {
                start_addr  = 0x0;
                size  = 0xFFFF;
                }

                Some(dsc) =>
                {
                         start_addr = dsc.memory_map.memory_start_address();
                         size = dsc.memory_map.memory_size();
        
                }
              }
        
            self.buffer_path = path;
        
            save_buffer_to_file(self.buffer_path.clone(), start_addr, size,self);

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
                                
            
                                  let dsc:Option<MC56f80x>  = Some(MC56f80x::create_target( programmer, 0x8000, 0x0000, "MC56f8035".to_string()));
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

              self.show_p_progress = true;

              self.target_status = TargetStatus::InProgramming;

              self.progress_bar_value = 0.0;

              let dsc = self.target.as_mut().expect("target lost");
            
              self.read_adddress = dsc.memory_map.memory_start_address();
              let size = dsc.memory_map.memory_size();
              let end_addr = self.read_adddress + size as u32;

              self.read_buff.clear();

              self.number_to_read = end_addr * 2 / 64;
              dbg!(&self.number_to_read);
              self.counter = 0;
              return iced::Command::perform(handle_progress( self.progress_bar_value), Message::ProgrammingInProgress);
              
            }

            Message::ReadTargetProgress(x)  => 
            {
                
                //self.show_p_progress = true;

                self.target_status = TargetStatus::InProgramming;
                let dsc = self.target.as_mut().expect("target lost");
  
                let counter = self.counter;
                //dbg!(counter);
                //dbg!(self.number_to_read );

                for counter in counter..self.number_to_read 
                {
                  let read = dsc.read_target(self.selected_power, self.read_adddress);
                    match read
                    {
                      Ok(read) => 
                      {
                      self.read_buff.push(read);
                      self.progress_bar_value = ((counter as f32 / self.number_to_read  as f32) * 100.00) as f32;
                      self.read_adddress += 0x40 / 2; 
                      self.counter = counter + 1;
                      return iced::Command::perform(handle_progress( self.progress_bar_value), Message::ProgrammingInProgress);
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
                    
              dbg!("Read Target End!");
              self.buffer.upload_packed(self.read_buff.clone());
              self.show_p_progress = false;
              self.target_status = TargetStatus::Connected;
              return iced::Command::none();
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
                
                println!("TestFeedback");
     
            } 

            Message::TargetProgramminEnd =>
            {
                
               // self.show_p_progress = false;
              //  self.target_status = TargetStatus::Connected;
              //  let dsc =  self.target.as_mut().expect("");
              //  dsc.power(TargetVddSelect::VddOff);
              //  self.check_power_state();
                //return  iced::Command::none();
     
            } 


        }
        iced::Command::none()
    }

/* 
    fn subscription(&self) -> Subscription<Message> {
        match self.target_status {
            _ => Subscription::none(),
            TargetStatus::Programming { .. } => {
                time::every(Duration::from_millis(10)).map(Message::Tick)
            }
        }
    }
*/

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
    
    let main_page = main_window::main_page(self);
        
    
    let err_view = match self.error_status.clone()
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
    if self.show_p_progress 
    {
        progress_bar_modal(self.show_p_progress, main_page.into(), self.progress_bar_value)
    }
    else if self.about_card_open 
    {
        about_card(self.about_card_open, main_page.into())
     }
    else if self.show_conn_image
    {

        connection_image_modal(1300, self.show_conn_image, main_page.into())
    }
    else
    {
        error_notify_model(self.show_error_modal, main_page.into(), err_view) 
    }
    

   
    
   // main_page.into()

    }

 
}


