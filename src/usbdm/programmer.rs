#![allow(unused)]

use crate::errors::{Error};
use crate::usbdm::usb_interface::{UsbInterface};
use crate::usbdm::feedback::{FeedBack, PowerState, PowerStatus};
use crate::usbdm::settings::{BdmSettings, TargetVddSelect, TargetType};
use crate::usbdm::constants::{bdm_commands};
use crate::usbdm::bdm_info::BdmInfo;
use crate::usbdm::jtag::*;
use crate::dsc_algo::mc56f80x::{MC56f80x};
use std::{thread, time};
use std::time::Duration;

#[derive(Debug)]
pub struct Programmer {

    pub usb_device     : UsbInterface,
    pub name           : String,
    pub bdm_info       : BdmInfo,
    pub feedback       : FeedBack,
    pub settings       : BdmSettings,      
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
            name            : "?".to_string(),
            bdm_info        : BdmInfo::default(),
            feedback        : FeedBack::default(),
            settings        : BdmSettings::default(), };
        prog.get_bdm_info()?;
        prog.bdm_info.check_version()?;
        prog.bdm_info.capabilities.check_dsc_supported()?;
        prog.name = prog.usb_device.model.clone();
        prog.feedback = prog.get_bdm_feedback()?;
        prog.force_vdd_off()?;
        prog.bdm_info.print_version2();
        prog.bdm_info.print_capabilities();
        //prog.get_bdm_string_descripton()?;
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
  
    self.usb_device.write(&usb_buf)?;                                    // write command
    let answer = self.usb_device.read(1)?;
    self.settings.target_voltage = power;
    Ok(())
  
}

pub fn force_vdd_off(&mut self ) -> Result<(), Error>{
    
    println!("force vdd_off");
    let mut usb_buf  = [0; 4];
    let command = "CMD_USBDM_SET_VDD".to_string();
    

    usb_buf[0] = 4;
    usb_buf[1] = bdm_commands::CMD_USBDM_SET_VDD;
    usb_buf[2] = u8::from(TargetVddSelect::VddOff);  
    usb_buf[3] = u8::from(TargetVddSelect::VddOff);  
  
    let bit = 0x80;
    let bitter = usb_buf[1] | bit;
    usb_buf[1] = bitter;
  
    self.usb_device.write(&usb_buf)?;                                    // write command
    let answer = self.usb_device.read(1)?;
    self.settings.target_voltage = TargetVddSelect::VddOff;
    Ok(())
  
}



pub fn get_power_state(&mut self) -> Result<PowerStatus, Error>
{
   
    self.refresh_feedback()?;

    //dbg!(self.feedback.power_state);

    let power_status =   Result::from(self.feedback.power_state)?;
    //dbg!(&power_status);

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


pub fn check_expected_power(&mut self, user_power_query : TargetVddSelect) -> Result<PowerStatus, Error>
{
    

    let current_power_status =   self.get_power_state()?;
    let expected_power = self.expected_power_status(user_power_query);
    
    if(current_power_status == expected_power)
    {
        Ok(current_power_status)
    }
    else
    {
       dbg!("check_expected_power retry sleep");
       thread::sleep(time::Duration::from_millis(50));
       let retry_power_status =   self.get_power_state()?;
       if(retry_power_status == expected_power)
       {
           Ok(retry_power_status)
       }
       else
       {
        Err(Error::PowerStateError)
       }
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
  
        self.usb_device.write(&usb_buf)?;                                    // write command
        let answer = self.usb_device.read(1)?;         // read status from bdm

        self.settings.target_voltage = power;
        Ok(())
    }


pub fn refresh_feedback(&mut self) -> Result<(), Error>
{
    self.feedback = self.get_bdm_feedback()?;
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

    self.usb_device.write(&usb_buf)?;                                    // write command
    let answer = self.usb_device.read(1)?;        // read status from bdm
    self.settings.target_type = TargetType::MC56F80xx;

    Ok(())
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

    self.usb_device.write(&usb_buf)?;                                    // write command
    let answer = self.usb_device.read(1)?;         // read status from bdm
    Ok(())
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


    self.usb_device.write(&full_command.as_slice())?;   // write command
    let mut answer: Vec<u8> = self.usb_device.read((answer_lenght + 1).into())?;         // read status from bdm 
    answer.remove(0);
    Ok((answer))
  } 

  pub fn bdm_control_pins(&mut self, control: u16) -> Result<(), Error>{

    let mut usb_buf  = [0; 4];

    usb_buf[0] = 4;            // lenght of command
    usb_buf[1] = bdm_commands::CMD_USBDM_CONTROL_PINS;
    usb_buf[2] = (control>>8) as u8;  
    usb_buf[3] = control as u8;

    self.usb_device.write(&usb_buf)?;                                    // write command
    let answer = self.usb_device.read(1)?;      // read status from bdm
    Ok(())
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
    self.set_vdd(TargetVddSelect::VddOff)?;
    thread::sleep(time::Duration::from_millis(self.settings.reset_duration));
    //self.bdm_control_pins(PIN_RESET_LOW)?;
    self.set_vdd(previous_power)?;
    //thread::sleep(time::Duration::from_millis(self.settings.reset_release_interval));
    //self.bdm_control_pins(PIN_RELEASE)?;
    thread::sleep(time::Duration::from_millis(self.settings.reset_recovery_interval));
    Ok(())
}

pub fn get_bdm_info(&mut self) -> Result<(), Error> {
    self.get_bdm_version()?;
    self.get_bdm_capabilities()?;
    Ok(())
}

fn get_bdm_version(&mut self) -> Result<(), Error>{
    let request_type = 64; //LIBUSB_REQUEST_TYPE_VENDOR
    let request_type = request_type| &self.usb_device.read_ep;
    
    let request  = bdm_commands::CMD_USBDM_GET_VER; // command
    let value    = 100;
    let index    = 0;
    let timeout  = Duration::from_millis(2500);
    let rx_size  = 10;
     
    let version = self.usb_device.control_transfer(
        request_type,
        request,
        value,
        index,
        rx_size)?;                                    

    let raw_bdm_software_version = u32::from (version[1]);
    let calculation = ((raw_bdm_software_version&0xF0)<<12) + ((raw_bdm_software_version&0x0F)<<8);

    self.bdm_info.bdm_software_version = calculation;
    self.bdm_info.bdm_hardware_version  = version[2];
    self.bdm_info.icp_software_version  = version[3];
    self.bdm_info.icp_hardware_version  = version[4];
    Ok(())
}

fn get_bdm_capabilities(&mut self) -> Result<(), Error>{
    let mut usb_buf = [0; 2];
    usb_buf[0] = 2;  // lenght
    usb_buf[1] = bdm_commands::CMD_USBDM_GET_CAPABILITIES;
    let command = "CMD_USBDM_GET_CAPABILITIES".to_string();

    let bit = 0x80;
    let bitter = usb_buf[1] | bit;
    usb_buf[1] = bitter;

    self.usb_device.write(&usb_buf)?;        // write command
    let answer: Vec<u8> = self.usb_device.read(8)?;                   //  read

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

    if answer.len() >= 8 {
        // Newer BDMs report extended software version
        self.bdm_info.bdm_software_version = ((answer[5] as u32) << 16)+((answer[6] as u32) << 8)+answer[7] as u32;
    }

    // Calculate permitted read & write length in bytes
    // Allow for JTAG header + USB header (5 bytes) & make multiple of 4
    self.bdm_info.dsc_max_memory_read_size  = (self.bdm_info.jtag_buffer_size - JTAG_READ_MEMORY_HEADER_SIZE  - 5) & !3;
    self.bdm_info.dsc_max_memory_write_size = (self.bdm_info.jtag_buffer_size - JTAG_WRITE_MEMORY_HEADER_SIZE - 5) & !3;
                                        
    Ok(())
}

pub fn get_string_version(&self) -> String {

    let str_ver = self.bdm_info.version_in_string().clone();
    str_ver
} 
}



