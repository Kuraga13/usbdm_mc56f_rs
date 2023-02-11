//use rusb::{UsbContext, Error};
use rusb::{UsbContext};
use std::time::Duration;
use packed_struct::prelude::*;


mod errors;
mod usb_interface;
mod enums;
mod feedback;
use crate::errors::{Error};
use crate::enums::{BDMCommands,Vdd,Vpp };
use crate::usb_interface::{UsbInterface, find_usbdm, find_usbdm_as, Capabilities};
use crate::feedback::{FeedBack};
use iced::alignment;
use iced::executor;
use iced::widget::{button, checkbox, container, text, Column};
use iced::window;

use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};

use iced_native::Event;
//#[allow(non_camel_case_types)]
//pub type Error = USBDMerror;


#[derive(Debug, Default)]
struct UsbdmApp  {
    last: Vec<iced_native::Event>,
    enabled: bool,
  

}


#[derive(Debug, Clone)]
enum Message {
    
    EventOccurred(iced_native::Event),
    Toggled(bool),
    Exit,
    Connect,
    FindUsbdmEnum(Result<rusb::Device<rusb::GlobalContext>, Error>),
}



pub fn main() -> iced::Result {
    UsbdmApp::run(Settings {
        exit_on_close_request: false,
        ..Settings::default()
    })
}

async fn test_main() {

    let usb_device =  find_usbdm().expect("Usbdm not found!");
    let mut usb_interface = UsbInterface::new(usb_device).expect("Usbdm found but, cant' be configured");
    usb_interface.print_UsbInterface();    
  //  let version = usb_interface
    //.get_bdm_version().expect("Error on get bdm ver");

  //  version.check_version().expect("Error on check bdm ver");
   // version.print_version();

   //usb_interface.set_vdd(Vdd::BDM_TARGET_VDD_ENABLE);
    let mut programmer = UsbdmProgrammer::new(usb_interface);
    programmer.refresh_feedback();
    programmer.set_vdd(Vdd::BDM_TARGET_VDD_5V);
    programmer.refresh_feedback();
    programmer.print_usbdm_programmer().unwrap();
    programmer.set_vdd(Vdd::BDM_TARGET_VDD_OFF);
    //programmer.set_vdd(Vdd::BDM_TARGET_VDD_DISABLE);
    //programmer.set_vdd(Vdd::BDM_TARGET_VDD_DISABLE);
    programmer.refresh_feedback();
    programmer.print_usbdm_programmer().unwrap();
    programmer.refresh_feedback();
    programmer.print_usbdm_programmer().unwrap();
   // let feedback = programmer.refresh_feedback();
//let printer = programmer.print_usbdm_programmer().unwrap();
    
}


    // NEW!!!
// example
//  
//    let target = TargetDsc::from_xml(target: from_user_input, options : from_user_select)
//    .setup_usbdm(programmer)?        // setup vdd, speed, set target,
//    .connect()?                      // reset, init jtag, read id code, enable_once
//    .read_memory(buffer: read_buff : Vec<u8>)?
//    
//    

//   


pub struct UsbdmProgrammer <T: UsbContext> {

usb_device  : UsbInterface<T>,


capabilities : Capabilities,
feedback     : FeedBack,
//jtag_buffer_size : u32,



//state_from_bdm : BdmStatus,



}



impl <T: rusb::UsbContext>  UsbdmProgrammer<T>
{

fn new(mut device : UsbInterface<T>) -> Self {


    Self{

        
        capabilities    : device.get_bdm_version().expect("Error on get bdm ver"),
        feedback        : device.get_bdm_status().expect("Error on feedback"),
        usb_device      : device,
    }

}


fn set_vdd(&self, power: u8 ) -> Result<(), Error>
{

    self.usb_device.set_vdd(power)?;
    Ok(())


}

fn set_vpp(&self, power: u8 ) -> Result<(), Error>
{

    self.usb_device.set_vpp(power)?;
    Ok(())


}

fn refresh_feedback(&mut self)
{
    self.feedback = self.usb_device.get_bdm_status().unwrap();
    println!("{}", self.feedback);
}

fn print_usbdm_programmer(&self) -> Result<(), Error>

{
   
    &self.capabilities.print_version();
    &self.feedback.print_feedback();
    
    Ok(())
}

}




impl Application for UsbdmApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (UsbdmApp, Command<Message>) {
        (UsbdmApp::default(), Command::none())
        
    }

    fn title(&self) -> String {
        String::from("UsbdmApp - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) if self.enabled => {
                self.last.push(event);

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }

                Command::none()
            }
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::close()
                } else {
                    Command::none()
                }
            }
            Message::Toggled(enabled) => {
                self.enabled = enabled;

                Command::none()
            }
            Message::Exit => window::close(),

            Message::Connect => 
            {    
            Command::perform(find_usbdm_as(),  Message::FindUsbdmEnum)
                
            } 

            Message::FindUsbdmEnum(Ok(_handle)) => 
            {
                UsbInterface::new(_handle).expect("Usbdm found but, cant' be configured");;
                window::minimize(true)
            } 


            Message::FindUsbdmEnum(Err(_)) => window::maximize(true),
            
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        let events = Column::with_children(
            self.last
                .iter()
                .map(|event| text(format!("{event:?}")).size(40))
                .map(Element::from)
                .collect(),
        );

        let toggle = checkbox(
            "Listen to runtime events",
            self.enabled,
            Message::Toggled,
        );

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

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(events)
            .push(toggle)
            .push(exit)
            .push(find_usbdm_button);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}



//Read IDCODE from JTAG TAP
//
//@param idCode   - 32-bit IDCODE returned from TAP
//@param resetTAP - Optionally resets the TAP to RUN-TEST/IDLE before reading IDCODE
//                  This will enable the MASTER TAP!
//
// @note - resetTAP=true will enable the Master TAP & disable the Code TAP
// @note - Leaves Core TAP in RUN-TEST/IDLE
//
//USBDM_ErrorCode readIDCODE(uint32_t *idCode, uint8_t commandRegLength, int resetTAP) {
 //   LOGGING_Q;
 //   uint8_t readCoreIdCodeSequence[] = {
 //      (uint8_t)JTAG_TEST_LOGIC_RESET,                          // Reset TAP
    // (uint8_t)  JTAG_REPEAT_Q(TEST_LOGIC_RESET_RECOVERY_NOPS), // ~2.26ms
    // (uint8_t)     JTAG_NOP,
    // (uint8_t)  JTAG_END_REPEAT,
//       (uint8_t)JTAG_MOVE_IR_SCAN,                              // Write IDCODE command to IR
//       (uint8_t)JTAG_SET_EXIT_SHIFT_DR,
//       (uint8_t)JTAG_SHIFT_OUT_Q(commandRegLength), (uint8_t)JTAG_IDCODE_COMMAND,
//       (uint8_t)JTAG_SET_EXIT_IDLE,                             // Read IDCODE from DR
//       (uint8_t)JTAG_SHIFT_IN_Q(32),
//       (uint8_t)JTAG_END
//    };
// 
 //   JTAG32 idcode(0,32);
  //  USBDM_ErrorCode rc;
 //
 //   if (resetTAP)
 //      readCoreIdCodeSequence[0] = JTAG_TEST_LOGIC_RESET;
 //   else
 //      readCoreIdCodeSequence[0] = JTAG_NOP;
 //
 //   rc = executeJTAGSequence(sizeof(readCoreIdCodeSequence), readCoreIdCodeSequence,
 //                            4, idcode.getData(32), false);
 //   if (rc != BDM_RC_OK) {
 //      log.print("Failed, reason = %s\n", USBDM_GetErrorString(rc));
 //      return rc;
 //   }
 //  log.print("IDCODE = %s\n", idcode.toString());
 //   *idCode = idcode;
 
 //   return rc;
 //}









