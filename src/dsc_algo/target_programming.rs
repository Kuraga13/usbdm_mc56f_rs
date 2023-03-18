use super::target_factory::{TargetFactory,TargetProgramming, MemoryMap, SecurityStatus,TargetDsc};
use crate::errors::Error;
use crate::usbdm::jtag::*;
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use crate::usbdm::constants::{memory_space_t};


impl TargetDsc
{
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
      prog.write_memory_block(memory_space_t::MS_XWORD, ram_test_data.clone(), ram_addr)?;

      let compare = prog.dsc_read_memory(memory_space_t::MS_XWORD, ram_test_data.len() as u32,  ram_addr)?;

      if (compare != ram_test_data)
      {
        return Err(Error::RamRWTestFault)
      }
      ram_addr += 0x20;
    }
 
      Ok(())

  }
}


impl TargetProgramming for TargetDsc {

fn init(&mut self, prog : &mut Programmer) -> Result<(), Error>
{
  prog.set_bdm_options()?;
  prog.refresh_feedback()?;
  prog.set_target_mc56f()?;

  Ok(())  

}

fn connect(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>
{

  prog.target_power_reset()?;
  self.power(power, prog)?;

  let dsc_jtag_id_code = read_master_id_code_DSC_JTAG_ID(true, &prog)?;

  enableCoreTAP(&prog); 

  let target_device_id = read_core_id_code(false, &prog)?; // on second not
  
  //self.print_id_code(&target_device_id, &dsc_jtag_id_code);

  //self.security_status_from_id_code(dsc_jtag_id_code, MC5680XX_SIM_ID);
  self.family.is_unsecure(prog)?;
  dbg!(&self.security);
  self.once_status = OnceStatus::UnknownMode;

  for retry in 0..10 
  {
    dbg!(&self.once_status);
    self.once_status = targetDebugRequest(&prog)?;
    if(self.once_status == OnceStatus::DebugMode)
    {
      break;
    }
    if(self.once_status == OnceStatus::UnknownMode) 
    {
       return Err((Error::TargetNotConnected))
    }
  }

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
  
  //let check_ram = self.test_ram_rw(0x008000)?; // Algo implementation ... now here
   
  let test_addr =  0x0686;//self.memory_map.start_address;
  //let test_mem_access_type =  *self.memory_map.get_memory_space_type(test_addr)?;

  let test_write = vec![0xAA; 0xEC];
  let mem_write = prog.write_memory_block(memory_space_t::MS_XWORD, test_write, test_addr)?;

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

  Ok(())

 }

}

