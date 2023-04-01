use super::target_factory::{ TargetProgramming, SecurityStatus, TargetDsc, DscFamily};
use crate::errors::Error;
use crate::utils::*;
use crate::usbdm::jtag::*;
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use crate::usbdm::constants::{memory_space_t};
use super::flash_routine::FlashRoutine;

use std::{thread, time};
use std::time::Duration;

impl TargetDsc {


  pub fn test_ram_rw(&mut self, ram_start_add: u32, power: TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>
  {

    let powered = prog.get_power_state()?;
    self.once_status = enableONCE(&prog)?;
    
    if(powered != PowerStatus::PowerOn && self.once_status != OnceStatus::DebugMode)
    {
        self.connect(power, prog)?;
    }
  
    if (self.security == SecurityStatus::Secured)
    {
      return Err(Error::TargetSecured)
    }

    let ram_test_data = vec![0x55, 0x55, 0xAA, 0xAA, 0xFF, 0x2A, 0x5C, 0x23, 0x21, 0x11];
    let mut ram_addr = ram_start_add;
    for retry_test_ram in 0..10
    {
      prog.dsc_write_memory(memory_space_t::MS_XWORD, ram_test_data.clone(), ram_addr)?;

      let compare = prog.dsc_read_memory(memory_space_t::MS_XWORD, ram_test_data.len() as u32,  ram_addr)?;

      if (compare != ram_test_data)
      {
        return Err(Error::RamRWTestFault)
      }
      ram_addr += 0x20;
    }
 
      Ok(())

  }

pub fn test_rw_programm_counter(&mut self, power: TargetVddSelect, prog : &mut Programmer) -> Result<(), Error> {

  let powered = prog.get_power_state()?;
  self.once_status = enableONCE(&prog)?;
  
  if(powered != PowerStatus::PowerOn && self.once_status != OnceStatus::DebugMode)
  {
      self.connect(power, prog)?;
  }

  if (self.security == SecurityStatus::Secured)
  {
    return Err(Error::TargetSecured)
  }

  let pc_start = prog.dsc_read_pc()?;

  dbg!(pc_start);

  prog.dsc_write_pc(0x008000);

  let pc_writed = prog.dsc_read_pc()?;

  dbg!(pc_writed);

  if(pc_writed != 0x008000)
  {

    return Err(Error::InternalError("test_rw_programm_counter Test Failed! Pc mismatch!".to_string()));

  }

  Ok(())

 }

 pub fn test_rw_debug_target(&mut self, power: TargetVddSelect, prog : &mut Programmer) -> Result<(), Error> {

  let powered = prog.get_power_state()?;
  self.once_status = enableONCE(&prog)?;
  
  if(powered != PowerStatus::PowerOn && self.once_status != OnceStatus::DebugMode)
  {
      self.connect(power, prog)?;
  }

  if (self.security == SecurityStatus::Secured)
  {
    return Err(Error::TargetSecured)
  }

  let pc_start = prog.dsc_read_pc()?;

  dbg!(pc_start);

  let pc_start2 = prog.dsc_read_pc()?;

  dbg!(pc_start2);

  prog.dsc_target_go()?;

  thread::sleep(time::Duration::from_millis(20));

  prog.dsc_target_halt()?;

  let pc_after_execution = prog.dsc_read_pc()?;

  dbg!(pc_after_execution);
  
  let pc_after_execution2 = prog.dsc_read_pc()?;

  dbg!(pc_after_execution2);

  self.power(TargetVddSelect::VddOff, prog)?;

  Ok(())

 }

 pub fn test_get_speed_routine(&mut self, power: TargetVddSelect, prog : &mut Programmer) -> Result<(), Error> {

  let powered = prog.get_power_state()?;
  self.once_status = enableONCE(&prog)?;
  
  if(powered != PowerStatus::PowerOn && self.once_status != OnceStatus::DebugMode)
  {
      self.connect(power, prog)?;
  }

  if (self.security == SecurityStatus::Secured)
  {
    return Err(Error::TargetSecured)
  }

  let dsc_bus_freq = self.flash_routine.get_target_speed(prog)?;

  self.family.init_for_write_erase(power, prog, dsc_bus_freq)?;

  //self.flash_routine.dsc_write_prog_mem(prog)?;

  //self.power(TargetVddSelect::VddOff, prog)?;

  Ok(())

 }


}


impl TargetProgramming for TargetDsc {

fn init(&mut self, prog : &mut Programmer) -> Result<(), Error>
{
  prog.init_usbdm_for_mc56f()?;
  println!("dsc init done!");
  Ok(())  

}

fn connect(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>
{

  prog.target_power_reset()?;
  self.power(power, prog)?;

  let dsc_jtag_id_code = read_master_id_code_DSC_JTAG_ID(true, &prog)?;

  enableCoreTAP(&prog); 

  let target_device_id = read_core_id_code(false, &prog)?; // on second not

  self.security = self.family.is_unsecure(prog, dsc_jtag_id_code, self.jtag_id_code)?;
  dbg!(&self.security);
  self.once_status = OnceStatus::UnknownMode;

  prog.dsc_target_halt()?;

  self.once_status = enableONCE(&prog)?;
  dbg!("Final status is: ", &self.once_status);

  
  Ok(())
    
}

fn power(&mut self, user_power_query : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>
{
                                                                        
  prog.set_vdd(user_power_query)?;           // If we try double-set power, filter in set_vdd just return ok
  prog.check_expected_power(user_power_query)?;    // Check power is setted
  Ok(())

}


fn disconnect(&mut self) 
{
    
 drop(self);
  
}

fn read_target(&mut self, power : TargetVddSelect, address : u32, prog : &mut Programmer) -> Result<Vec<u8>, Error>
{

  let powered = prog.get_power_state()?;
  self.once_status = enableONCE(&prog)?;
  
  if(powered != PowerStatus::PowerOn && self.once_status != OnceStatus::DebugMode)
  {
      self.connect(power, prog)?;
  }

  if (self.security == SecurityStatus::Secured)
  {
    return Err(Error::TargetSecured)
  }
 
 // let test_addr = 0x7f40;
 // let test_mem_access_type =  *self.memory_map.get_memory_space_type(address)?;
  let memory_read = prog.dsc_read_memory(memory_space_t::MS_PWORD, 0x40,  address)?;
  

  //self.programmer.target_power_reset()?;
 // self.programmer.refresh_feedback()?;
  //self.power(TargetVddSelect::VddOff)?;

  Ok(memory_read)


}


fn write_target(&mut self, power : TargetVddSelect, data_to_write : Vec<u8>, prog :  &mut Programmer) -> Result<(), Error>
{
  
  let powered = prog.get_power_state()?;
  self.once_status = enableONCE(&prog)?;
  
  if(powered != PowerStatus::PowerOn && self.once_status != OnceStatus::DebugMode)
  {
      self.connect(power, prog)?;
  }
  if (self.security == SecurityStatus::Secured)
  {
    return Err(Error::TargetSecured)
  }

  dbg!(&self.once_status);
  
  let dsc_bus_freq = self.flash_routine.get_target_speed(prog)?;

  self.family.init_for_write_erase(power, prog, dsc_bus_freq)?;

  self.flash_routine.dsc_write_prog_mem(prog)?;
 
 // let test_write = vec![0xAA; 0x40];
  //let mem_write = prog.dsc_write_memory(memory_space_t::MS_PWORD, test_write, 0x0000)?;

  prog.target_power_reset()?;
  prog.refresh_feedback()?;
  self.power(TargetVddSelect::VddOff, prog)?;
 
  Ok(())
    
}

fn erase_target(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>
{
  
  let powered = prog.get_power_state()?; // base init : check power and status
  self.once_status = enableONCE(&prog)?;
  
  if(powered != PowerStatus::PowerOn && self.once_status != OnceStatus::DebugMode)
  {
      self.connect(power, prog)?;
  }

  let dsc_bus_freq = self.flash_routine.get_target_speed(prog)?;

  self.family.init_for_write_erase(power, prog, dsc_bus_freq)?;

  self.flash_routine.dsc_erase_routine(prog)?;

  prog.target_power_reset()?;

  self.connect(power, prog)?;

  if (self.security == SecurityStatus::Secured)
  {
    return Err(Error::TargetSecured)
  }

  Ok(())

 }

}

