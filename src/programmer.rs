#![allow(unused)]

mod memory_access;
pub mod jtag;
mod bdm_info;
use bdm_info::BdmInfo;

use crate::usb_interface::{UsbInterface};
use crate::errors::{Error};
use crate::feedback::{FeedBack, PowerState, PowerStatus};
use crate::settings::{BdmSettings, TargetVddSelect, TargetType};
use crate::enums::{bdm_commands};
use std::{thread, time};


use crate::target::{Target};



#[derive(Debug)]
pub struct Programmer {

    usb_device     : UsbInterface,
    bdm_info       : BdmInfo,
    feedback       : FeedBack,
    settings       : BdmSettings,      
}



impl Drop for Programmer{

    fn drop(&mut self) {
        let _ = self.set_vdd(TargetVddSelect::VddOff);
        drop(&mut self.usb_device);
        println!("Programmer dropped");
    }
}

impl Programmer
{
    pub fn new(mut device : UsbInterface) -> Result<Self, Error> {
        let mut prog = Self {
            usb_device      : device,
            bdm_info        : BdmInfo::default(),
            feedback        : FeedBack::default(),
            settings        : BdmSettings::default(), };
        prog.get_bdm_info()?;
        prog.feedback = prog.usb_device.get_bdm_status()?;
        Ok(prog)
    }

pub fn set_vdd(&mut self, power: TargetVddSelect ) -> Result<(), Error>{
    

    let current_power_status =   self.get_power_state()?;
    let expected_power = self.expected_power_status(power);
    
    if(current_power_status == expected_power)
    {
        return Ok(());                               // Reserve filter on trying double-set power. 
        dbg!("set_vdd double filter!");              // Needed for right sequence: first you power off, second you setup new power
    }

    let mut usb_buf  = [0; 4];
    let command = "CMD_USBDM_SET_VDD".to_string();
  
    usb_buf[0] = 4;
    usb_buf[1] = bdm_commands::CMD_USBDM_SET_VDD;
    usb_buf[2] = u8::from(power);  
    usb_buf[3] = u8::from(power);  
  
    let bit = 0x80;
    let bitter = usb_buf[1] | bit;
    usb_buf[1] = bitter;
  
    self.usb_device.write(&usb_buf,1500)?;                                    // write command
    let answer = self.usb_device.read()?;
    self.settings.target_voltage = power;
    Ok(())
  
            // read status from bdm
       // self.check_usbm_return_code(command, &answer)?;               // check is status o
}




pub fn get_power_state(&mut self) -> Result<PowerStatus, Error>
{
    
    self.refresh_feedback()?;

    let power_status =   Result::from(self.feedback.power_state)?;

    Ok(power_status) 

}


pub fn expected_power_status(&mut self, user_power_query : TargetVddSelect) -> PowerStatus
{
    
    match user_power_query
    {
        TargetVddSelect::VddOff     => PowerStatus::PowerOff,
        TargetVddSelect::Vdd3V3     => PowerStatus::PowerOn,
        TargetVddSelect::Vdd5V      => PowerStatus::PowerOn,
        TargetVddSelect::VddEnable  => PowerStatus::PowerOn,
        TargetVddSelect::VddDisable => PowerStatus::PowerOff,
    }

}


pub fn check_expected_power(&mut self, user_power_query : TargetVddSelect) -> Result<(), Error>
{
    

    let current_power_status =   self.get_power_state()?;
    let expected_power = self.expected_power_status(user_power_query);
    
    if(current_power_status == expected_power)
    {
        Ok(())
    }
    else
    {
       Err(Error::PowerStateError)
    }
}



pub fn set_vpp(&mut self, power: TargetVddSelect ) -> Result<(), Error>{
      
        let mut usb_buf  = [0; 4];
        let command = "CMD_USBDM_SET_VPP".to_string();
  
        usb_buf[0] = 3;
        usb_buf[1] = bdm_commands::CMD_USBDM_SET_VPP;
        usb_buf[2] = u8::from(power);  
    
  
        let bit = 0x80;
        let bitter = usb_buf[1] | bit;
        usb_buf[1] = bitter;
  
        self.usb_device.write(&usb_buf,1500)?;                                    // write command
        let answer = self.usb_device.read()?;         // read status from bdm
       // self.check_usbm_return_code(command, &answer)?;               // check is status ok

        self.settings.target_voltage = power;
        Ok(())
      }


pub fn refresh_feedback(&mut self) -> Result<(), Error>
{
    self.feedback = self.usb_device.get_bdm_status()?;

    //println!("{}", self.feedback);
    Ok(())
}

fn print_usbdm_programmer(&self) -> Result<(), Error>

{
   
    self.bdm_info.print_version();
    self.feedback.print_feedback();
    
    Ok(())
}





pub fn set_target_mc56f(&mut self) -> Result<(), Error>{

    let mut usb_buf  = [0; 3];
    let mc56_target  =  0x07;  // byte to set command
    let command = "CMD_USBDM_SET_TARGET".to_string();

    usb_buf[0] = 3;            // lenght of command
    usb_buf[1] = bdm_commands::CMD_USBDM_SET_TARGET;
    usb_buf[2] = mc56_target;  
 
    let bit = 0x80;           
    let bitter = usb_buf[1] | bit;
    usb_buf[1] = bitter;

    self.usb_device.write(&usb_buf,1500)?;                                    // write command
    let answer = self.usb_device.read()?;        // read status from bdm
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
    let answer = self.usb_device.read()?;         // read status from bdm
    let status = self.usb_device.check_usbm_return_code( &answer)?;    // check is status ok
    Ok(status)
}



   pub fn exec_jtag_seq(&self, mut jtag_seq : Vec<u8>,  answer_lenght : u8) -> Result<(Vec<u8>), Error>{
      
    
    let command = "CMD_USBDM_JTAG_EXECUTE_SEQUENCE".to_string();

    let command_leght : u8 = 0x4 + jtag_seq.len() as u8;

    let mut full_command : Vec<u8> = Vec::new();
    full_command.push(command_leght);
    full_command.push(bdm_commands::CMD_USBDM_JTAG_EXECUTE_SEQUENCE | 0x80);
    full_command.push(answer_lenght);
    full_command.push(jtag_seq.len() as u8);
    full_command.append(&mut jtag_seq);


    self.usb_device.write(&full_command.as_slice(),1500)?;   // write command
    let answer: Vec<u8> = self.usb_device.read()?;         // read status from bdm
    self.usb_device.check_usbm_return_code(&answer)?;   
    Ok((answer))
  } 

  pub fn bdm_control_pins(&mut self, control: u16) -> Result<(), Error>{

    let mut usb_buf  = [0; 4];

    usb_buf[0] = 4;            // lenght of command
    usb_buf[1] = bdm_commands::CMD_USBDM_CONTROL_PINS;
    usb_buf[2] = (control>>8) as u8;  
    usb_buf[3] = control as u8;

    self.usb_device.write(&usb_buf,1500)?;                                    // write command
    let answer = self.usb_device.read()?;      // read status from bdm
    let status = self.usb_device.check_usbm_return_code(&answer)?;    // check is status ok
    Ok(status)
}

pub fn target_hardware_reset(&mut self) -> Result<(), Error>{
    const PIN_RESET_LOW : u16 = 2<<2;   // Set Reset low
    const PIN_RELEASE   : u16 = 0xffff; // Release all pins (go to default for current target)
    self.bdm_control_pins(PIN_RESET_LOW)?;
    thread::sleep(time::Duration::from_millis(self.settings.reset_duration));
    self.bdm_control_pins(PIN_RELEASE)?;
    thread::sleep(time::Duration::from_millis(self.settings.reset_recovery_interval));
    Ok(())
}

pub fn target_power_reset(&mut self) -> Result<(), Error>{
    const PIN_RESET_LOW : u16 = 2<<2;   // Set Reset low
    const PIN_RELEASE   : u16 = 0xffff; // Release all pins (go to default for current target)
    let previous_power = self.settings.target_voltage;
    self.set_vdd(TargetVddSelect::VddOff);
    thread::sleep(time::Duration::from_millis(self.settings.reset_duration));
    //self.bdm_control_pins(PIN_RESET_LOW)?;
    self.set_vdd(previous_power);
    //thread::sleep(time::Duration::from_millis(self.settings.reset_release_interval));
    //self.bdm_control_pins(PIN_RELEASE)?;
    thread::sleep(time::Duration::from_millis(self.settings.reset_recovery_interval));
    Ok(())
}
  
}



