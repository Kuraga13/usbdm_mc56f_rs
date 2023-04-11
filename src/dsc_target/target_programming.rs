use super::target_factory::{ TargetProgramming, SecurityStatus, TargetDsc, DscFamily, FlashModuleStatus};
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


impl TargetProgramming for TargetDsc {

fn init(&mut self, prog : &mut Programmer) -> Result<(), Error>
{
  prog.init_usbdm_for_mc56f()?;
  println!("init prog for dsc done!");
  Ok(())  

}

fn connect(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>
{ 
  // TODO - we need reset for re-connect
  // that mean:
  // - check &reset power for certain time and check is voltage
  // - check & reset target jtag (to execution mode)
  prog.target_power_reset(power)?;
  self.power(power, prog)?;
  // need be tested! some dsc start status unknown, but looks like is in debug mode!
  prog.dsc_target_go()?;

  self.once_status = enableONCE(&prog)?;

  dbg!("Start status is: ", &self.once_status);

  self.flash_module = FlashModuleStatus::NotInited;

  let jtag_id = read_master_id_code_DSC_JTAG_ID(true, &prog)?;
  enableCoreTAP(&prog)?; 
  let core_id =  read_core_id_code(false, &prog)?; // on second not !
  
  self.family.target_family_confirmation(jtag_id, core_id)?;

 
  self.once_status = OnceStatus::UnknownMode;

  prog.dsc_target_halt()?;

  self.once_status = enableONCE(&prog)?;

  dbg!("Final status is: ", &self.once_status);

  self.security = self.family.is_unsecure(prog)?;

  dbg!(&self.security);

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
  
  if(powered != PowerStatus::PowerOn || self.once_status != OnceStatus::DebugMode || self.flash_module == FlashModuleStatus::Inited)
  {
      self.connect(power, prog)?;
  }

  if (self.security == SecurityStatus::Secured)
  {
    return Err(Error::TargetSecured)
  }

  let programm_range = self.programm_range().expect("Get mem range err App");
  let start_address = programm_range.start;
  let end_addr: usize = programm_range.end as usize;
 
  let mut block_size: u32 = ((end_addr as u32 + 1) - address) * 2;

  let max_block_size : u32 = 0x100;

  if block_size > max_block_size { 
    block_size = max_block_size;
  };
 
  let memory_read = prog.dsc_read_memory(memory_space_t::MS_PWORD, block_size,  address)?; 

  Ok(memory_read)


}


fn write_target(&mut self, power : TargetVddSelect, address : u32, data_to_write : Vec<u8>, prog :  &mut Programmer) -> Result<(), Error>
{
  
  let powered = prog.get_power_state()?;
  self.once_status = enableONCE(&prog)?;
  
  if(powered != PowerStatus::PowerOn || self.once_status != OnceStatus::DebugMode)
  {
      self.connect(power, prog)?;
  }
  if (self.security == SecurityStatus::Secured)
  {
    return Err(Error::TargetSecured)
  }

  if (self.flash_module != FlashModuleStatus::Inited)
  { 
    if let Ok(dsc_bus_freq) =  self.flash_routine.get_target_speed(prog) {
     self.flash_module = self.family.init_for_write_erase(power, prog, dsc_bus_freq)?; 
    } else {  return Err(Error::TargetWriteError); }  
  }

  if let Ok(()) = self.flash_routine.dsc_write_prog_mem(prog, data_to_write, address) 
  {
    Ok(())
  } 
   else {  return Err(Error::TargetWriteError); }  
 
}

fn verify_target(&mut self, power : TargetVddSelect, address : u32, prog : &mut Programmer) -> Result<usize, Error> {


  let powered = prog.get_power_state()?;
  self.once_status = enableONCE(&prog)?;
  
  if(powered != PowerStatus::PowerOn || self.once_status != OnceStatus::DebugMode || self.flash_module == FlashModuleStatus::Inited)
  {
      self.connect(power, prog)?;
  }

  if (self.security == SecurityStatus::Secured)
  {
    return Err(Error::TargetSecured)
  }
  
  let programm_range = self.programm_range().expect("Get mem range err App");
  let start_address = programm_range.start;
  let end_addr: usize = programm_range.end as usize;
 
  let mut block_size: u32 = ((end_addr as u32 + 1) - address);

  let max_block_size : u32 = 0x80;

  if block_size > max_block_size { 
    block_size = max_block_size;
  };

  let to_verify: Vec<u8> = self.memory_buffer.download_target_block(address as usize, block_size as usize)?; 

  let block_len = to_verify.len() as u32;

  let memory_read = prog.dsc_read_memory(memory_space_t::MS_PWORD, block_len,  address)?; 

  if(to_verify != memory_read) {

    println!("to_verify length: {} data: {:?}", to_verify.len(), to_verify);
    println!("");
    println!("memory_read length: {} data: {:?}", memory_read.len(), memory_read);

    return Err(Error::TargetVerifyError(address, address + block_size));
     
  } 

    Ok(memory_read.len())


}


fn erase_target(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>
{
  
  let powered = prog.get_power_state()?; // base init : check power and status
  self.once_status = enableONCE(&prog)?;
  
  if(powered != PowerStatus::PowerOn || self.once_status != OnceStatus::DebugMode)
  {
      self.connect(power, prog)?;
  }

  self.family.mass_erase(power, prog)?;

  self.connect(power, prog)?;

  if (self.security == SecurityStatus::Secured)
  {
    return Err(Error::TargetSecured)
  }

  Ok(())

 }

}

