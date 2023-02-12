#![allow(unused)]

//use rusb::{UsbContext, Error};
//use rusb::{UsbContext};
//use std::time::Duration;
//use packed_struct::prelude::*;


mod errors;
mod usb_interface;
mod enums;
mod feedback;
mod app;
use crate::errors::{Error};
use crate::enums::{bdm_commands,vdd,vpp};
use crate::usb_interface::{UsbInterface, find_usbdm, find_usbdm_as, Capabilities};
use crate::feedback::{FeedBack};
use crate::app::{run};
//#[allow(non_camel_case_types)]
//pub type Error = USBDMerror;



pub fn main() -> iced::Result {
    
         run()
    
}

async fn test_main() {

    let usb_device =  find_usbdm().expect("Usbdm not found!");
    let mut usb_interface = UsbInterface::new(usb_device).expect("Usbdm found but, cant' be configured");
    usb_interface.print_usb_interface();    
  //  let version = usb_interface
    //.get_bdm_version().expect("Error on get bdm ver");

  //  version.check_version().expect("Error on check bdm ver");
   // version.print_version();

   //usb_interface.set_vdd(Vdd::BDM_TARGET_VDD_ENABLE);
    let mut programmer = UsbdmProgrammer::new(usb_interface);
    programmer.refresh_feedback();
    programmer.set_vdd(vdd::BDM_TARGET_VDD_5V);
    programmer.refresh_feedback();
    programmer.print_usbdm_programmer().unwrap();
    programmer.set_vdd(vdd::BDM_TARGET_VDD_OFF);
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



pub struct UsbdmProgrammer {

usb_device  : UsbInterface,


capabilities : Capabilities,
feedback     : FeedBack,
//jtag_buffer_size : u32,



//state_from_bdm : BdmStatus,



}


impl UsbdmProgrammer
{

fn new(mut device : UsbInterface) -> Self {


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









