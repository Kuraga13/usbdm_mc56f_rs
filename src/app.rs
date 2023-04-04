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

use std::sync::{Arc, RwLock};

use crate::usbdm::usb_interface::{UsbInterface, find_usbdm_as, find_usbdm,};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use crate::usbdm::programmer::{Programmer};
use crate::dsc_target::target_factory::{TargetProgramming, TargetDsc, TargetSelector, MemorySegment, TargetYaml};
use crate::dsc_target::test_programming::*;
use crate::gui::{self, main_window};
use crate::gui::modal_notification::{error_notify_model, about_card, connection_image_modal, progress_bar_modal, erase_write_confirm_modal};
use crate::gui::hexbuffer_widget::{TableContents, HexBuffer};
use crate::file_buffer::hex_file::{load_buffer_from_file, save_buffer_to_file, FileFormat};
use crate::errors::{Error};
use crate::utils::*;
#[derive(Debug, Clone, PartialEq)]
pub enum UsbdmAppStatus {
    
    NotConnected,
    Connected,
    Errored,

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TargetStatus {
    
    NotConnected,
    Connected,
    InProgrammingRead,
    InProgrammingWrite,
    InProgrammingVerify,
    InProgrammingErase,
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

    WriteEraseConfirmation(bool),
    
    TargetSelect(TargetSelector),
    ConnectionImageOpen(bool),
    Connect,
    Disconnect,
    PowerSelect(TargetVddSelect),
    PowerToggle,
    ReadTarget,
    ReadTargetProgress(f32),
    WriteTarget,
    WriteTargetProgress(f32),
    VerifyTarget,
    VerifyTargetProgress(f32),
    EraseTarget,
    EraseTargetProgress(f32),
    TestFeedback,
    
}

pub struct App {

    pub    target             : TargetDsc,
           target_database    : TargetYaml,

           programmer         : Option<Programmer>,
           programmer2        : Option<Arc<RwLock<Programmer>>>,

    pub    buffer             : HexBuffer,
           buffer_path        : String,
    pub    selected_power     : TargetVddSelect,
    pub    status             : UsbdmAppStatus,
    pub    power_status       : PowerStatus,
    pub    target_status      : TargetStatus,
    pub    show_error_modal   : bool,
    pub    show_confirmation  : bool,
    pub    about_card_open    : bool,
    pub    show_conn_image    : bool,
    pub    show_p_progress    : bool,
    pub    error_status       : Option<Error>,
    pub    theme              : iced::Theme,
    pub    dark_mode          : bool,
    pub    progress_bar_value : f32,
    pub    title              : String,
    
    pub    number_cycles      : u32,            // for debug
    pub    counter            : u32,            // for debug
           progr_buff         : Vec<Vec<u8>>,   // for debug
           progr_address     : u32,            // for debug


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
    let mut prog =  self.programmer.as_mut().expect("Try to Connect to Opt:None Target!");

    let mut check_connect_usb =  prog.refresh_feedback();
    ok_or_show_error(self, check_connect_usb.clone());
    match check_connect_usb
    {
        Ok(_) =>
        {
            self.status = UsbdmAppStatus::Connected;

        }


        Err(_) =>
        {
            self.title =  "usbdm_mc56f_rs ".to_string() + &"not connected ".to_string();
            self.status = UsbdmAppStatus::NotConnected;
            self.target_status = TargetStatus::NotConnected;

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

    let mut dsc =  Box::new(&mut self.target);// self.target.as_mut().expect("Try to Connect to Opt:None Target!");
    let prog = self.programmer.as_mut().expect("Try to Connect to Opt:None Programmer!");

    let mut check_connect_dsc =  dsc.connect(self.selected_power, prog);
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
            println!("check_connection_target error");

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

    let mut prog =  self.programmer.as_mut().expect("Try to Connect to Opt:None Target!");
    let mut power_state =  prog.get_power_state();
    match power_state
    {

        Ok(power_state) =>
        {
            self.power_status = power_state;

        }


        Err(err) =>
        {
            println!("check_power_state error");
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

        let database    = TargetYaml::init_target_db().expect("Err on Yaml Database!");

        (           
            Self {
         
                theme,
                dark_mode          : false,  
                buffer             : HexBuffer::default(),
                buffer_path        : "".to_string(),
                show_error_modal   : false,
                show_confirmation  : false,
                show_conn_image    : false,
                about_card_open    : false,
                show_p_progress    : false,
                error_status       : None,     
                selected_power     : TargetVddSelect::Vdd3V3,
                target             : TargetDsc::target_from_selector(TargetSelector::Tester56f8035, database.clone()).expect("Target Builder Fault!"),
                target_database    : database,
                programmer         : None,
                programmer2        : None,
                status             : UsbdmAppStatus::NotConnected,
                target_status      : TargetStatus::NotConnected,
                power_status       : PowerStatus::PowerOff,
                progress_bar_value : 0.0,
                title              : "usbdm_mc56f_rs ".to_string() + &"not connected ".to_string(),
                number_cycles      : 0,
                counter            : 0,
                progr_buff         : vec![vec![0;0]],
                progr_address     : 0,

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

            Message::TargetSelect(target) => 
            {
                                 
             /* TargetSelect for new target programming interface, with abstract factory */
             let selected = TargetDsc::target_from_selector(target, self.target_database.clone());
             match selected {

                Ok(mut target) => 
                { 
                    self.target = target;
                    self.target_status = TargetStatus::NotConnected;
                    return iced::Command::none();
                }
                Err(_e) => 
                {                
                    show_error(  self, _e);
                    return iced::Command::none();
                } 
              };
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

            let programm_range = self.target.programm_range().expect("Get mem range err App"); 
    
            start_addr = programm_range.start as u32;
            size = (programm_range.end - programm_range.start) as usize;

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

            let programm_range = self.target.programm_range().expect("Get mem range err App"); 
    
            start_addr = programm_range.start as u32;
            size = (programm_range.end - programm_range.start) as usize;
            
    
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

            
            Message::WriteEraseConfirmation(confirmation) =>
            {

              if(confirmation == true)
              {
                self.show_confirmation = false;
                match self.target_status
                {
                    TargetStatus::InProgrammingWrite => 
                    {
                    return iced::Command::perform(handle_progress( 0.0), Message::WriteTargetProgress);
                    }
                    TargetStatus::InProgrammingErase => 
                    {
                    return iced::Command::perform(handle_progress( 0.0), Message::EraseTargetProgress);
                    }
                    _ =>
                    {
                    return iced::Command::none();
                    }
                }
              }
              else
              {
                self.target_status = TargetStatus::Connected;
                self.show_confirmation = false;
              }
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
                                
                                  self.programmer = Some(programmer);
                                  let handle =  self.programmer.as_mut().expect("");
                                  self.title =  "usbdm_mc56f_rs ".to_string() + &"connected ".to_string() + &handle.name.clone() +  &handle.get_string_version().clone();
                                
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
                       self.target_status = TargetStatus::NotConnected;

                       let prog = self.programmer.as_mut().expect("Try to Connect to Opt:None Programmer!");

                       let init = self.target.init(prog);

                       match init
                       {
                           Ok(_) =>
                           {
                            self.check_connection_target();
                            self.check_power_state();
                           }

                           Err(_e) =>

                           {
                           show_error(  self, _e);
                           println!("Error on init programmer for Target!");
                           return iced::Command::none();

                           }
                       }

              
                    
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
             
        
                println!("Try disconnect and drop");
                self.target.disconnect();
                self.status  = UsbdmAppStatus::NotConnected; 
                
            } 

            Message::PowerToggle => 
            {    

            self.check_power_state();

               match self.power_status
               {

                 PowerStatus::PowerOn =>
                 {
                    let dsc =  Box::new(&mut self.target);
                    let prog = self.programmer.as_mut().expect("Try to Connect to Opt:None Programmer!");
                    dsc.power(TargetVddSelect::VddOff, prog);
                    self.check_power_state();

                 }

                 PowerStatus::PowerOff =>

                 {
    
                    let dsc =  Box::new(&mut self.target);
                    let prog = self.programmer.as_mut().expect("Try to Connect to Opt:None Programmer!");
                    dsc.power(self.selected_power, prog);
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

                self.progr_buff.clear();

             self.show_p_progress = true;

             self.target_status = TargetStatus::InProgrammingRead;
  
             self.progress_bar_value = 0.0;
  
             let dsc = Box::new(&mut self.target);
  
             let programm_range = self.target.programm_range().expect("Get mem range err App"); 
      
             self.progr_address = programm_range.start as u32;

             return iced::Command::perform(handle_progress( self.progress_bar_value), Message::ReadTargetProgress);
              
            }

            Message::ReadTargetProgress(x)  => 
            {

              self.target_status = TargetStatus::InProgrammingRead;
              let dsc = Box::new(&mut self.target);
              let prog = self.programmer.as_mut().expect("Try to Connect to Opt:None Programmer!");

              let max_block_size : u32 = 0x100;
              
              let program_range = dsc.programm_range().expect("Get mem range err App"); 
              let last_address: usize = program_range.end as usize;

              let mut block_size: u32 = ((last_address as u32 + 1) - self.progr_address) * 2;
            
              if block_size > max_block_size { 
                block_size = max_block_size;
              };

                if (last_address + 1) > self.progr_address as usize {

                  let read = dsc.read_target(self.selected_power, self.progr_address, prog, block_size);
                  match read
                  {
                    Ok(read) => 
                    {
                     self.progr_buff.push(read);
                     self.progress_bar_value = ((self.progr_address as f32 / last_address  as f32) * 100.00) as f32;
                     self.progr_address += block_size / 2; 
                     return iced::Command::perform(handle_progress( self.progress_bar_value), Message::ReadTargetProgress);
                    }    

                    Err(_e) =>
                    {
                    self.show_p_progress = false;
                    show_error(self, _e);
                    println!("ReadTarget Error!");
                    self.check_power_state();
                    return iced::Command::none();
                    }
                  }
                }
                
            dbg!("Read Target End!");
            self.buffer.upload_packed(self.progr_buff.clone());
            self.show_p_progress = false;
            self.target_status = TargetStatus::Connected;
            return iced::Command::none();

            }

            Message::WriteTarget  => 
            {

              self.target_status = TargetStatus::InProgrammingWrite;
              
              self.show_confirmation = true;
  
              self.progress_bar_value = 0.0;
  
              let dsc = Box::new(&mut self.target);
  
              let programm_range = self.target.programm_range().expect("Get mem range err App"); 
      
              self.progr_address = programm_range.start as u32;

              return iced::Command::none();
            }

            Message::WriteTargetProgress(x)  => 
            {
              
              self.show_p_progress = true;
              self.target_status = TargetStatus::InProgrammingWrite;
              let dsc = Box::new(&mut self.target);
              let prog = self.programmer.as_mut().expect("Try to Connect to Opt:None Programmer!");

              let block_size : u32 = 0x500;
              let offset = self.progr_address * 2;

              let start_block = offset as usize;
              let mut end_block = (offset + block_size) as usize; 
              let mut buff: Vec<u8> = self.buffer.download_in_one();
              
              let last_address: usize = buff.len();
              if end_block > last_address { 
                  end_block = last_address;
              };

                if end_block > start_block {

                  let to_write: Vec<u8> = buff.drain(start_block..end_block).collect(); 

                  let write_target = dsc.write_target(self.selected_power,  self.progr_address, to_write, prog);
                  match write_target
                  {
                    Ok(write_target) => 
                    {
                    self.progress_bar_value = ((start_block as f32 / last_address  as f32) * 100.00) as f32;
                    self.progr_address += block_size / 2; 
                    return iced::Command::perform(handle_progress( self.progress_bar_value), Message::WriteTargetProgress);
                    }
                    Err(_e) =>
                    {
                    self.show_p_progress = false;
                    show_error(self, _e);
                    println!("WriteTarget error");
                    self.check_power_state();
                    return iced::Command::none();
                    }
                  }
                }
                
            dbg!("Write Target End!");
            self.show_p_progress = false;
            self.target_status = TargetStatus::Connected;
            return iced::Command::none();
                
            }

            Message::VerifyTarget  => 
            {

              self.show_p_progress = true;

              self.target_status = TargetStatus::InProgrammingVerify;
  
              self.progress_bar_value = 0.0;
  
              let dsc = Box::new(&mut self.target);
  
              let programm_range = self.target.programm_range().expect("Get mem range err App"); 
      
              self.progr_address = programm_range.start as u32;

              return iced::Command::perform(handle_progress( self.progress_bar_value), Message::VerifyTargetProgress);
            }

            Message::VerifyTargetProgress(x)  => 
            {

              self.target_status = TargetStatus::InProgrammingVerify;
              let dsc = Box::new(&mut self.target);
              let prog = self.programmer.as_mut().expect("Try to Connect to Opt:None Programmer!");

              let block_size : u32 = 0x40;
              let offset = self.progr_address * 2;

              let start_block = offset as usize;
              let mut end_block = (offset + block_size) as usize; 
              let mut buff: Vec<u8> = self.buffer.download_in_one();
              
              let last_address: usize = buff.len();
              if end_block > last_address { 
                  end_block = last_address;
              };

                if end_block > start_block {

                  let to_verify: Vec<u8> = buff.drain(start_block..end_block).collect(); 
                  let block_size_read = (end_block - start_block) as u32;

                  let verify = dsc.read_target(self.selected_power, self.progr_address, prog, block_size_read);

                  match verify
                  {
                    Ok(verify) => 
                    {
                     if(to_verify == verify)
                     {
                     self.progress_bar_value = ((start_block as f32 / last_address  as f32) * 100.00) as f32;
                     self.progr_address += block_size / 2; 
                     return iced::Command::perform(handle_progress( self.progress_bar_value), Message::VerifyTargetProgress);
                     } else {   
                        self.show_p_progress = false;
                        show_error(self, Error::TargetVerifyError);
                        println!("error on VerifyTarget ");
                        self.check_power_state();
                        return iced::Command::none(); }
                    }
                    Err(_e) =>
                    {
                    self.show_p_progress = false;
                    show_error(self, _e);
                    println!("error on VerifyTarget ");
                    self.check_power_state();
                    return iced::Command::none();
                    }
                  }
                }
                
            dbg!("Verify Target End!");
            self.show_p_progress = false;
            self.target_status = TargetStatus::Connected;
            return iced::Command::none();

            }

            Message::EraseTarget  => 
            {
            
            self.target_status = TargetStatus::InProgrammingErase;
            self.show_confirmation = true;
            return iced::Command::none();

      
            }

            
            Message::EraseTargetProgress(x)  => 
            {
              
              self.show_p_progress = true;
              self.target_status = TargetStatus::InProgrammingErase;
              self.progress_bar_value = 0.0;
              
              let dsc = Box::new(&mut self.target);
            
              let erase_target = dsc.erase_target(self.selected_power, self.programmer.as_mut().expect("Try to Connect to Opt:None Target!"));

              match erase_target
              {
                Ok(_) => 
                {
                self.check_power_state();
                println!("target erase ok!");
    
                }
                Err(_e) =>
                {
                show_error(self, _e);
                self.check_power_state();
                println!("erase_target error");
                return iced::Command::none();
               }
              }

              dbg!("Erase Target End!");
              self.show_p_progress = false;
              self.target_status = TargetStatus::Connected;
              return iced::Command::none();
            }


            Message::TestFeedback =>
            {
                
                println!("TestFeedback");


                let mut dsc =  Box::new(&mut self.target);
                
                let prog = self.programmer.as_mut().expect("Try to Connect to Opt:None Programmer!");


                let test_debug = dsc.test_get_speed_routine(self.selected_power, prog);

                match test_debug
                {
                  Ok(_) => 
                  {
                  self.check_power_state();
                  println!("test_get_speed_routine ok!");
      
                  }
                  Err(_e) =>
                  {
                  show_error(self, _e);
                  self.check_power_state();
                  println!("test_get_speed_routine error");
                  return iced::Command::none();
                 }
                }
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
    if self.show_confirmation 
    {
        erase_write_confirm_modal(self.show_confirmation, main_page.into(), self.target_status)
    }
    else if self.show_p_progress 
    {
        progress_bar_modal(self.show_p_progress, main_page.into(), self.progress_bar_value, self.target_status)
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


