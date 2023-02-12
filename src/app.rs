use iced::alignment;
use iced::executor;
use iced::widget::Row;
use iced::widget::{button, checkbox, container, text, Column, column, pick_list};
use iced::window;

use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};

use iced_native::Event;
use crate::usb_interface::{UsbInterface, find_usbdm_as};
use crate::errors::{Error};
use crate::programmer::{Programmer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerSwitchGui {
    Off,
    _3_3v,
    _5_0v,

}

impl PowerSwitchGui {
    const ALL: [PowerSwitchGui; 3] = [
        PowerSwitchGui::Off,
        PowerSwitchGui::_3_3v,
        PowerSwitchGui::_5_0v,

    ];
}

impl std::fmt::Display for PowerSwitchGui {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PowerSwitchGui::Off => "Off",
                PowerSwitchGui::_3_3v => "3.3v",
                PowerSwitchGui::_5_0v => "5.0v",
            }
        )
    }
}



#[derive(Debug, Clone)]
enum Message {
    
    EventOccurred(iced_native::Event),
    Toggled(bool),
    Exit,
    Connect,
    Disconnect,
    FindUsbdmEnum(Result<rusb::Device<rusb::GlobalContext>, Error>),
    SetPower,
    PowerSelect(PowerSwitchGui),
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
  selected_power: Option<PowerSwitchGui>,
    
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

             match &self.selected_power
             {   
             
             Some(selected_power) => match selected_power
             {
                PowerSwitchGui::Off => { &self.programmer.as_ref().expect("Not Programmer").set_vdd_off(); Command::none()}
                PowerSwitchGui::_3_3v => { &self.programmer.as_ref().expect("Not Programmer").set_vdd_3_3v(); Command::none()}
                PowerSwitchGui::_5_0v => { &self.programmer.as_ref().expect("Not Programmer").set_vdd_5v();  Command::none()}
                
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
               let usb_int = UsbInterface::new(_handle).expect("Usbdm found but, cant' be configured");
               self.programmer = Some(Programmer::new(usb_int)); 
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
        
        let pick_list = pick_list(
            &PowerSwitchGui::ALL[..],
            self.selected_power,
            Message::PowerSelect,
        )
        .placeholder("Power:");


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
            .push(pick_list)
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