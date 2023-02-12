use std::vec;

use iced::alignment;
use iced::executor;
use iced::widget::Row;
use iced::widget::{button, checkbox, container, text, Column, column};
use iced::window;

use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};

use iced_native::Event;
use crate::usb_interface::{UsbInterface, find_usbdm_as, Capabilities};
use rusb::{UsbContext};
use crate::errors::{Error};
use crate::enums::{BDMCommands,Vdd,Vpp };

#[derive(Debug, Clone)]
enum Message {
    
    EventOccurred(iced_native::Event),
    Toggled(bool),
    Exit,
    Connect,
    Disconnect,
    FindUsbdmEnum(Result<rusb::Device<rusb::GlobalContext>, Error>),
    SetPower,
}

#[derive(Debug, Clone)]
enum UsbdmAppStatus {
    
    Start,
    Connected,
    //Errored(Error),
    Errored,

}

struct UsbdmApp {


    usb_device  : Option<UsbInterface>,
    capabilities : Option<Capabilities>,
    status       : UsbdmAppStatus,

  //  feedback     : FeedBack,
   
    //jtag_buffer_size : u32,
    
    
    
    //state_from_bdm : BdmStatus,
    
    
    
    }

impl UsbdmApp
{

fn set_vdd(&self, power: u8 ) -> Result<(), Error>
{
    match &self.usb_device
    {   
    Some(usb_device) => usb_device.set_vdd(power)?,

    None => panic!("usb_device not found!")
    } 

    Ok(())
    
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

        status : UsbdmAppStatus::Start, 
        usb_device : None,
        capabilities : None, 
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
            
                Command::perform(find_usbdm_as(),  Message::FindUsbdmEnum)
                
            } 

            Message::SetPower => 
            {    
            
                self.set_vdd(Vdd::BDM_TARGET_VDD_5V);
                Command::none()
                
            } 

            Message::FindUsbdmEnum(Ok(_handle)) => 
            {
                self.usb_device = Some(UsbInterface::new(_handle).expect("Usbdm found but, cant' be configured"));
                self.status  = UsbdmAppStatus::Connected;
                Command::none()
            } 


            Message::FindUsbdmEnum(Err(_)) =>
            {
            
               self.status = UsbdmAppStatus::Errored;
               Command::none()
            } 
            
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
      


        let exit = button(
            text("Exit")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .width(Length::Units(100))
        .padding(10)
        .on_press(Message::Exit);


        let connect_usbdm_button = button(
            text("Connect")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Left),
        )
        .width(Length::Units(100))
        .padding(20)
        .on_press(Message::Connect);

        
        let disconnect_usbdm_button = button(
            text("Connect")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Left),
        )
        .width(Length::Units(100))
        .padding(20)
        .on_press(Message::Toggled(true));


              
        let set_power = button(
            text("Power On")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Right),
        )
        .width(Length::Units(100))
        .padding(20)
        .on_press(Message::SetPower);


        let conn_error = text("Not Connected".to_string());
        let conn_ok = text("Succes connect Usbdm".to_string());

        let content = match self.status {

        UsbdmAppStatus::Start => {
        
            Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(exit)
            .push(connect_usbdm_button)
        }

        UsbdmAppStatus::Connected => {
        
            Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(exit)
            .push(set_power)
            .push(disconnect_usbdm_button)
            .push(conn_ok)
        }

        UsbdmAppStatus::Errored => {
        
            Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(exit)
            .push(connect_usbdm_button)
            .push(conn_error)
   

                 
        }
    };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}



pub fn run() -> iced::Result {
    UsbdmApp::run(Settings {
        exit_on_close_request: false,
        ..Settings::default()
    })
}