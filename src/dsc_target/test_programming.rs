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