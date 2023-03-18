use super::target_factory::{TargetFactory,TargetProgramming, MemoryMap};
use crate::errors::Error;
use crate::usbdm::jtag::*;
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use crate::usbdm::constants::{memory_space_t};


use std::collections::HashMap;
type AddressKey       = u32;
type MemorySpaceType  = u8;
type HexMap = HashMap<AddressKey, MemorySpaceType>; 


impl TargetFactory for MC56f80x {

    type Output = MC56f80x;
    
    fn create_target(mem_size : usize, start_addr : u32, name : String) -> MC56f80x
     {
          
       let v: Vec<u8> = vec![0x00; mem_size]; // default HexBuffer size 0xFFFF, filled 0xFF
       let mut map: HexMap = HashMap::new();
       let mut address_index: AddressKey = start_addr;
  
       for byte_ in v.iter() {
  
           map.insert(address_index, memory_space_t::MS_PWORD);
           address_index += 0x1;
      } 
  
      let m_map = MemoryMap::init_memory_map(mem_size, start_addr, map);
  
  
      return MC56f80x{once_status : OnceStatus::UnknownMode, security : SecurityStatus::Unknown, memory_map : m_map, mcu_name : name};
    }
  }

#[derive(Debug, Clone,PartialEq)]
pub enum SecurityStatus {
        
       Unknown,
       Secured,
       Unsecured,
    
}

  
 pub const  MC5680XX_SIM_ID : u32 =  0x01F2801D;
  // Least Significant Half of JTAG ID (SIM_LSHID), in mc568023-35 is  $801D.
  #[derive(Debug, Clone)]
pub struct MC56f80x {
    
   // pub programmer    : Programmer,
    pub once_status   : OnceStatus,
    pub security      : SecurityStatus,
    pub memory_map    : MemoryMap,
    mcu_name          : String, 

     
 }

 impl MC56f80x
 {
    
    ///`security_status_from_id_code` 
    /// 
    /// This read-only register, in two parts  displays the least significant half of the JTAG ID for the chip.
    /// 
    /// Most Significant Half of JTAG ID (`SIM_MSHID`), in mc568023-35 is `$01F2`.
    /// 
    /// Least Significant Half of JTAG ID (`SIM_LSHID`), in mc568023-35 is  `$801D`.
    /// 
    /// PGO wrote in original usbdm pjt, if you have match id code dsc in
    /// we have to match `jtag_id_code` with `SIM_ID`
    pub fn security_status_from_id_code(&mut self, jtag_id_code_vec : Vec<u8>, expected_id : u32) 
    {   
        let jtag_id_code =  self.vec_as_u32_be(jtag_id_code_vec);
    
       // println!("jtag_id_code : {:02X}", jtag_id_code);
       // println!("expected_id : {:02X}", expected_id);
      

        match jtag_id_code {
              expected_id=> self.security = SecurityStatus::Unsecured,
              0x0                        => self.security =  SecurityStatus::Secured,           
              _                          => self.security =  SecurityStatus::Unknown,             
          }    

        
    }
      
    
    
    pub fn print_id_code(&self, core_id_code : &Vec<u8>, master_id_code : &Vec<u8>) {
    
     println!(" core_id_code :");
      
     for byte in core_id_code.iter()
     {
     let in_string = format!("{:02X}", byte);
     print!("{} ", in_string);
     }
    
     println!(" \n");
    
     println!(" master_id_code (in usbdm jtag-idcode) :");
     for byte in master_id_code.iter()
     {
    
     let in_string = format!("{:02X}", byte);
     print!("{} ", in_string);
    
     }
    
     println!(" \n");
       
    }

    ///'print_vec_memory' for debug memory read, use for print small readed block
    fn print_vec_memory(&self, mem : Vec<u8>) {
    
    let mut printed_vec = Vec::new();
     for byte in mem.iter() {
     let in_string = format!("{:02X}", byte);
     printed_vec.push(in_string);
     if(printed_vec.len() == 0x10)
     {
      for symbol in printed_vec.iter() {
       print!("{} ", symbol); }
     print!("\n");
     printed_vec.clear();

    }  
  }
}

    pub fn vec_as_u32_be(&self, vec:  Vec<u8>) -> u32 {
        ((vec[0] as u32) << 24) +
        ((vec[1] as u32) << 16) +
        ((vec[2] as u32) <<  8) +
        ((vec[3] as u32) <<  0)
    }

    //fn load

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
 
 impl Drop for MC56f80x{
 
         fn drop(&mut self) {
             println!("Target dropped");
         }
 }
 



impl TargetProgramming for MC56f80x
{

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
  
  self.print_id_code(&target_device_id, &dsc_jtag_id_code);

  self.security_status_from_id_code(dsc_jtag_id_code, MC5680XX_SIM_ID);
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
  let test_mem_access_type =  *self.memory_map.get_memory_space_type(test_addr)?;

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

