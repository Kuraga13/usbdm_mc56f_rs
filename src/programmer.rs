#![allow(unused)]

use crate::usb_interface::{UsbInterface, BdmInfo};
use crate::errors::{Error};
use crate::feedback::{FeedBack};
use crate::settings::{BdmSettings, TargetVddSelect, TargetType};
use crate::enums::{bdm_commands,vdd};


use crate::jtag::*;



pub struct Programmer {


    usb_device   : UsbInterface,
    bdm_info     : BdmInfo,
    feedback     : FeedBack,
    settings     : BdmSettings,

   
    //jtag_buffer_size : u32,
    
    
    
    //state_from_bdm : BdmStatus,
    
    
    
    }

impl Drop for Programmer{

    fn drop(&mut self) {
        let _ = self.set_vdd_off();
        drop(&mut self.usb_device);
        println!("Programmer dropped");
    }
}

impl Programmer
{


pub fn new(mut device : UsbInterface) -> Self {


        Self{
    
            
            bdm_info        : device.get_bdm_version().expect("Error on get bdm ver"),
            feedback        : device.get_bdm_status().expect("Error on feedback"),
            settings        : BdmSettings::default(),
            usb_device      : device,

        }
    
    }

pub fn set_vdd_off(&mut self) -> Result<(), Error>
{
            
  self.usb_device.set_vdd(vdd::BDM_TARGET_VDD_OFF)?;
  self.settings.target_voltage = TargetVddSelect::VddOff;
  Ok(())
            
            
}
pub fn set_vdd_3_3v(&mut self) -> Result<(), Error>
{
        
  self.usb_device.set_vdd(vdd::BDM_TARGET_VDD_3V3)?;
  self.settings.target_voltage = TargetVddSelect::Vdd3V3;
  Ok(())
        
        
}

pub fn set_vdd_5v(&mut self) -> Result<(), Error>
{
    
    self.usb_device.set_vdd(vdd::BDM_TARGET_VDD_5V)?;
    self.settings.target_voltage = TargetVddSelect::Vdd5V;
    Ok(())
     
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

pub fn refresh_feedback(&mut self) -> Result<(), Error>
{
    //self.feedback = self.usb_device.get_bdm_status().unwrap();
    //usbdm.set_bdm_options();

    self.get_full_capabilities()?;
    println!("{}", self.bdm_info);
    //self.feedback.print_feedback();
    //println!("{}", self.feedback);
    Ok(())
}

fn print_usbdm_programmer(&self) -> Result<(), Error>

{
   
    self.bdm_info.print_version();
    self.feedback.print_feedback();
    
    Ok(())
}

pub fn dsc_connect(&self) -> Result<(), Error>

{
    let id_code_sequence = jtag::read_master_id_code(true); // first id request we reset CORE TAP

    self.usb_device.exec_jtag_seq(id_code_sequence, 0x04);

    let enable_core_tap_sequense = jtag::enableCoreTAP(); // on second not

    self.usb_device.exec_jtag_seq(enable_core_tap_sequense, 0);

    let id_code_sequence2 = jtag::read_master_id_code(true); // on second not

    
    Ok(())
}



pub fn set_target_mc56f(&mut self) -> Result<(), Error>{

    let mut usb_buf  = [0; 3];
    let mc56_target  =  7;  // byte to set command
    let command = "CMD_USBDM_SET_TARGET".to_string();

    usb_buf[0] = 3;            // lenght of command
    usb_buf[1] = bdm_commands::CMD_USBDM_SET_TARGET;
    usb_buf[2] = mc56_target;  
 
    let bit = 0x80;           
    let bitter = usb_buf[1] | bit;
    usb_buf[1] = bitter;

    self.usb_device.write(&usb_buf,1500)?;                                    // write command
    let answer = self.usb_device.read().expect("Can't read answer");          // read status from bdm
    let status = self.usb_device.check_usbm_return_code( &answer)?;    // check is status ok

    self.settings.target_type = TargetType::MC56F80xx;

    Ok(status)
}

pub fn set_bdm_options(&mut self) -> Result<(), Error>{

    let mut usb_buf  = [0; 6];

    usb_buf[0] = 6;            // lenght of command
    usb_buf[1] = bdm_commands::CMD_USBDM_SET_OPTIONS;
    
    let mut options: u8 = 0;
    if self.settings.cycle_vdd_on_reset {
        options |= 1<<0;
    }
    if self.settings.cycle_vdd_on_connect {
        options |= 1<<1;
    }
    if self.settings.leave_target_powered {
        options |= 1<<2;
    }
    if self.settings.guess_speed {
        options |= 1<<3;
    }
    if self.settings.use_reset_signal {
        options |= 1<<4;
    }
    
    usb_buf[2] = options;  
    usb_buf[3] = self.settings.target_voltage as u8;
    usb_buf[4] = self.settings.bdm_clock_source as u8;
    usb_buf[5] = self.settings.auto_reconnect as u8;

    self.usb_device.write(&usb_buf,1500)?;                                    // write command
    let answer = self.usb_device.read().expect("Can't read answer");          // read status from bdm
    let status = self.usb_device.check_usbm_return_code( &answer)?;    // check is status ok
    Ok(status)
}

/// `get_full_capabilities` - get all capabilities, if rx_size > 5 we need upgrade Capabilities struct...
pub fn get_full_capabilities(&mut self) -> Result<(), Error>{
      
    let mut usb_buf = [0; 2];
    usb_buf[0] = 2;  // lenght
    usb_buf[1] = bdm_commands::CMD_USBDM_GET_CAPABILITIES;
    let command = "CMD_USBDM_GET_CAPABILITIES".to_string();

    let bit = 0x80;
    let bitter = usb_buf[1] | bit;
    usb_buf[1] = bitter;

    self.usb_device.write(&usb_buf,1500)?;        // write command
    let answer: Vec<u8> = self.usb_device.read()?;                   //  read
    
    self.usb_device.check_usbm_return_code( &answer)?;  

    if answer.len() >= 3 {
        let capabilities: u16 = ((answer[1] as u16) << 8) | answer[2] as u16 ^ ((1<<5) | (1<<6));
        self.bdm_info.capabilities.parse(capabilities);
    }

    if answer.len() >= 5 {
        let mut buffer_size: u16 = ((answer[3] as u16) << 8) + answer[4] as u16;
        let max_packet_size: u16 = 255;
        if buffer_size > max_packet_size {
            buffer_size = max_packet_size;
        }
        let jtag_header_size: u16 = 5;

        self.bdm_info.command_buffer_size = buffer_size;
        self.bdm_info.jtag_buffer_size = buffer_size - jtag_header_size;
     }
                                                  
    
    
    Ok(())

   }
  
}



