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
use crate::errors::{Error};

#[derive(Debug, Clone)]
enum Message {
    
    EventOccurred(iced_native::Event),
    Toggled(bool),
    Exit,
    Connect,
    FindUsbdmEnum(Result<rusb::Device<rusb::GlobalContext>, Error>),
}

#[derive(Debug, Clone)]
enum UsbdmApp {
    
    Start,
    Connected,
    //Errored(Error),
    Errored,

}

impl Application for UsbdmApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (UsbdmApp, Command<Message>) {
        (UsbdmApp::Start, Command::none())
        
    }

    fn title(&self) -> String {
        String::from("UsbdmApp - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
    
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::close()
                } else {
                    Command::none()
                }
            }
            Message::Toggled(enabled) => {
                

                Command::none()
            }
            Message::Exit => window::close(),

            Message::Connect => 
            {    
            Command::perform(find_usbdm_as(),  Message::FindUsbdmEnum)
                
            } 

            Message::FindUsbdmEnum(Ok(_handle)) => 
            {
                UsbInterface::new(_handle).expect("Usbdm found but, cant' be configured");
                *self = UsbdmApp::Connected;
                window::minimize(true)
            } 


            Message::FindUsbdmEnum(Err(_)) =>
            {
              // *self = UsbdmApp::Errored(error);
            *self = UsbdmApp::Errored;
                window::maximize(true)
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


        let find_usbdm_button = button(
            text("Connect")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Left),
        )
        .width(Length::Units(100))
        .padding(20)
        .on_press(Message::Connect);


        let conn_error = text("Not Connected".to_string());


        let content = match self {

        UsbdmApp::Start => {
        
            Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
           // .push(events)
           // .push(toggle)
            .push(exit)
            .push(find_usbdm_button)
        }

        UsbdmApp::Connected => {
        
            column![text("Succes connect Usbdm").size(40),]
            .width(Length::Shrink)
        }

        UsbdmApp::Errored => {
        
            Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
           // .push(events)
           // .push(toggle)
            .push(exit)
            .push(find_usbdm_button)
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