use iced::alignment;
use iced::executor;
use iced::widget::Row;
use iced::widget::{button, checkbox, horizontal_rule,vertical_rule,row,  container, text, Column, column, pick_list};
use iced::window;


use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Sandbox,
};
use iced::window::Icon;
use image::GenericImageView;
use iced::theme::{self, Theme};

use iced_native::Event;
use crate::usb_interface::{UsbInterface, find_usbdm_as};
use crate::errors::{Error};
use crate::settings::{TargetVddSelect};
use crate::programmer::{Programmer};
use crate::hexbuff_widget::{HexBuffer};


#[derive(Debug, Clone)]
pub enum Message {
    
    EventOccurred(iced_native::Event),
    Toggled(bool),
    Exit,
    Connect,
    Disconnect,
    FindUsbdmEnum(Result<rusb::Device<rusb::GlobalContext>, Error>),
    SetPower,
    PowerSelect(TargetVddSelect),
    test_feedback,
}

#[derive(Debug, Clone)]
enum UsbdmAppStatus {
    
    Start,
    Connected,
    //Errored(Error),
    Errored,

}

struct UsbdmApp 
{

  programmer   : Option<Programmer>,
  status       : UsbdmAppStatus,
  selected_power: Option<TargetVddSelect>,
  buff          : HexBuffer,
    
}

impl UsbdmApp
{

// TODO:
// 1. this should be on usb interface mean low level!
// 2. before any query by programmer level we should query this function and return simple error, which can be handled
// without panic etc.
// 3. this one not work now, like example
// 4. we need something like "check not disconnected by user" or don't use option! 
fn check_connection(&self) -> Result<&Programmer, Error>
{
    match &self.programmer
    {
    Some(programmer) => Ok(programmer),
              
    None => Err(Error::LostConnection),
   
    }
    
}

}
    

impl Application for  UsbdmApp 

{
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (UsbdmApp, Command<Message>) {
        (   
        UsbdmApp
        {
        programmer : None,
        status : UsbdmAppStatus::Start, 
        selected_power : None,
        buff           : HexBuffer::new(),

        },
        Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("UsbdmApp - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
    
            Message::EventOccurred(event) => 
            {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::close()
                } else {
                    Command::none()
                }
            }
            Message::Toggled(enabled) => 
            {
                
                Command::none()
            }
            Message::Exit => window::close(),

            Message::Connect => 
            {    
                
                Command::perform(find_usbdm_as(),  Message::FindUsbdmEnum)
                
            } 

            Message::Disconnect => 
            {    
            
                match &self.programmer
                {   
                Some(programmer) =>{ 
                println!("Try disconnect and drop");
                drop(&mut self.programmer);
                self.programmer = None;
                self.status  = UsbdmAppStatus::Start; 
                Command::none()
                }
                    
                None => Command::none()
                } 
                
            } 

            Message::SetPower => 
            {    
            
             let usbdm =  self.programmer.as_mut().expect("");
             match &self.selected_power
             {   
             
             Some(selected_power) => match selected_power
             {
                TargetVddSelect::VddOff => {   usbdm.set_vdd_off(); Command::none()}
                TargetVddSelect::Vdd3V3 => { usbdm.set_vdd_3_3v(); Command::none()}
                TargetVddSelect::Vdd5V => { usbdm.set_vdd_5v();  Command::none()}
                TargetVddSelect::VddEnable => { usbdm.set_vdd_off(); Command::none()}
                TargetVddSelect::VddDisable => { usbdm.set_vdd_off();  Command::none()}
                
             } 
                       
             None => Command::none()          
            
             }
            }   


            Message::PowerSelect(power_select) => 
            {

                self.selected_power = Some(power_select);
                Command::none()

            }

            Message::FindUsbdmEnum(Ok(_handle)) => 
            {
               println!("Try claim usb");
               let usb_int = UsbInterface::new(_handle).expect("Programmer Lost Connection");
               self.programmer = Some(Programmer::new(usb_int)); 
               self.status  = UsbdmAppStatus::Connected;
               Command::none()
            } 


            Message::FindUsbdmEnum(Err(_)) =>
            {
            
               self.status = UsbdmAppStatus::Errored;
               Command::none()
            } 

            Message::test_feedback =>
            {
               
               self.check_connection().expect(" Programmer Lost Connection");
               let usbdm =  self.programmer.as_mut().expect("");
               usbdm.refresh_feedback();
               usbdm.set_bdm_options();
        
               Command::none()
            } 
            
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        

        let pick_list = pick_list(
            &TargetVddSelect::ALL[..],
            self.selected_power,
            Message::PowerSelect,
        )
        .placeholder("Power:");



        let connect_usbdm_button = button(
            text("Connect")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Left),
        )
        .width(Length::Units(100))
        .padding(20)
        .on_press(Message::Connect);

        
        let disconnect_usbdm_button = button(
            text("Disconnect")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Left),
        )
        .width(Length::Units(100))
        .padding(20)
        .on_press(Message::Disconnect);


              
        let set_power = button(
            text("Set Power")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Right),
        )
        .width(Length::Units(100))
        .padding(20)
        .style(theme::Button::Secondary)
        .on_press(Message::SetPower);


        let test_feedback = button(
            text("test_feedback")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Right),
        )
        .width(Length::Units(100))
        .padding(20)
        .on_press(Message::test_feedback);


        let conn_error = text("Not Connected".to_string());
        let conn_ok = text("Succes connect Usbdm".to_string());


        let  buffer_row = self.buff.view();
        let  ascii_row2 = HexBuffer::new().ascii_one_row_line_view();

        let footer =  {Row::new()
        .align_items(Alignment::Center) 
        .push(pick_list)  
        .push(set_power)    };

        let content = match self.status {

        UsbdmAppStatus::Start => {
        
            Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(connect_usbdm_button)
        
        }

        UsbdmAppStatus::Connected => {
        
            Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(disconnect_usbdm_button)
            .push(conn_ok)
            .push(test_feedback)
        }

        UsbdmAppStatus::Errored => {
        
            Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(connect_usbdm_button)
            .push(conn_error)
   

                 
        }
    };
    container( column![

        horizontal_rule(10),

        iced::widget::row![
            content.width(Length::Fill).align_items(iced::Alignment::Start),
        ],
        
        horizontal_rule(10),
  

        column![
            row![
            buffer_row.height(Length::Shrink),
            ascii_row2.height(Length::Shrink),]
        ].width(Length::Fill),
       
        horizontal_rule(10),

 
        iced::widget::column![
            footer.width(Length::Fill).align_items(iced::Alignment::End),
        ],

        
   

    ]
    .align_items(iced::Alignment::Center)
     //.height(Length::Shrink
    )
    .into()


       
    }
}



pub fn run() -> iced::Result {

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