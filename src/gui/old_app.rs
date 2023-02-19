#![allow(unused)]

use iced::executor;
use iced::widget::Row;
use crate::styling::{LineStyle};
use iced_lazy::responsive;

use iced::widget::rule;
use iced::widget::{Container, button, checkbox, horizontal_rule, vertical_rule,row, container, text, Column, Scrollable, Rule, column, pick_list};
use iced::window;

use iced::alignment::{self, Alignment};
use iced::{
    Application, Command, Element, Length, Settings, Subscription,
    Sandbox, Size,
};


use iced::theme::{self, Theme};
use iced_native::{Event, Widget};

use iced_aw::menu::{ItemHeight, ItemWidth, MenuBar, MenuTree, PathHighlight};
use iced_aw::quad;

use crate::usb_interface::{UsbInterface, find_usbdm_as};
use crate::errors::{Error};
use crate::settings::{TargetVddSelect};
use crate::programmer::{Programmer};
use crate::jtag::{JtagInterface};
use crate::hexbuff_widget::{HexBufferView};


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
    TestFeedback,
}

#[derive(Debug, Clone)]
enum UsbdmAppStatus {
    
    Start,
    Connected,
    //Errored(Error),
    Errored,

}

pub struct UsbdmApp 
{

  programmer     : Option<Programmer>,
  jtag           : JtagInterface,
  status         : UsbdmAppStatus,
  selected_power : Option<TargetVddSelect>,
  buff           : HexBufferView,
  buff_upd       : bool,  
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
        programmer     : None,
        jtag           : None,
        status         : UsbdmAppStatus::Start, 
        selected_power : None,
        buff           : HexBufferView::new(),
        buff_upd       : true,

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
                Some(_programmer) =>{ 
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
                TargetVddSelect::VddOff     => {if let Err(_e) =  usbdm.set_vdd_off()  {Command::none()} else {Command::none()}}
                TargetVddSelect::Vdd3V3     => {if let Err(_e) =  usbdm.set_vdd_3_3v() {Command::none()} else {Command::none()}}
                TargetVddSelect::Vdd5V      => {if let Err(_e) =  usbdm.set_vdd_5v()   {Command::none()} else {Command::none()}}
                TargetVddSelect::VddEnable  => {if let Err(_e) =  usbdm.set_vdd_off()  {Command::none()} else {Command::none()}}
                TargetVddSelect::VddDisable => {if let Err(_e) =  usbdm.set_vdd_off()  {Command::none()} else {Command::none()}}
                
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
               self.jtag = init(self.programmer); 
               self.status  = UsbdmAppStatus::Connected;
               Command::none()
            } 


            Message::FindUsbdmEnum(Err(_)) =>
            {
            
               self.status = UsbdmAppStatus::Errored;
               Command::none()
            } 

            Message::TestFeedback =>
            {
               
               self.check_connection().expect(" Programmer Lost Connection");
               let usbdm =  self.programmer.as_mut().expect("");
               if let Err(_e) = usbdm.refresh_feedback() { return Command::none() };
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





        let test_feedback = button(
            text("test_feedback")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Right),
        )
        .width(Length::Units(100))
        .padding(20)
        .on_press(Message::TestFeedback);


        let conn_error = text("Not Connected".to_string());
        let conn_ok = text("Succes connect Usbdm".to_string());


        //let  buffer_row = self.buff.view();
        

        //let  ascii_row2 = HexBuffer::new().ascii_one_row_line_view();
       // let  address_row = HexBuffer::new().adress_row();
       // let  address_row_demo = HexBuffer::new().adress_row();
       // let  demo_row = HexBuffer::new().demo_row();

     

         let footer = match self.status {

            UsbdmAppStatus::Start => {
            
                Column::new()
                .push(Row::new()
                .push(iced::widget::Button::new(iced::widget::Text::new("VDD")).style(theme::Button::Secondary))
                .spacing(15)
                .push(pick_list))
                
            }
    
            UsbdmAppStatus::Connected => {
            
                Column::new()
                .spacing(15)
                .push(Row::new()
                .push(iced::widget::Button::new(iced::widget::Text::new("VDD")).style(theme::Button::Primary).on_press(Message::SetPower))
                .spacing(15)
                .push(pick_list))
               
            }
    
            UsbdmAppStatus::Errored => {

                Column::new()
                .push(Row::new()
                .push(iced::widget::Button::new(iced::widget::Text::new("VDD")).style(theme::Button::Secondary))
                .spacing(15)
                .push(pick_list))
                     
            }
        };

        let header = match self.status {

        UsbdmAppStatus::Start => {
        
            Column::new()
            .spacing(20)
            .push(connect_usbdm_button)
            
        
        }

        UsbdmAppStatus::Connected => {
        
            Column::new()
            .spacing(20)
            .push(disconnect_usbdm_button)
            .push(conn_ok)
            .push(test_feedback)
        }

        UsbdmAppStatus::Errored => {
        
            Column::new()
            .spacing(20)
            .push(row![connect_usbdm_button, conn_error])
                 
        }
    };

    let page_footer = {Row::new()
        .push(iced::widget::row![footer.width(Length::Fill).padding(10).align_items(Alignment::End)])
        .push(horizontal_rule(10))
        };

    let page_header = {Row::new()
    .push(iced::widget::row![header.width(Length::Fill).padding(10).align_items(Alignment::Start)])
    .push(horizontal_rule(10))
    };


    //let vertical_line = Rule::vertical(1)
    //.style(theme::Rule::Custom(Box::new(LineStyle::new(1))));


    let page_buffer = column![self.buff.view()];
  

    let page_buff_scroll = Scrollable::new(page_buffer);

    let programmer_page = Column::new()
        .push(page_header)
        .push(page_footer)
        .push(page_buff_scroll);
      
                
    Container::new(programmer_page)
    .into()
       
    }
}


