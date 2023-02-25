use crate::errors::Error;
use crate::programmer::jtag::*;
use crate::programmer::jtag::{OnceStatus};
use crate::programmer::{Programmer};
use crate::settings::{TargetVddSelect};
use crate::feedback::{PowerStatus};
use std::borrow::BorrowMut;

    // Memory space indicator - includes element size
    // One of the following
    pub const MS_BYTE    : u8  = 1;        // Byte (8-bit) access
    pub const MS_WORD    : u8  = 2;        // Word (16-bit) access
    pub const MS_LONG    : u8  = 4;        // Long (32-bit) access
    // One of the following
    pub const MS_NONE    : u8  = 0<<4;     // Memory space unused/undifferentiated
    pub const MS_PROGRAM : u8  = 1<<4;     // Program memory space (e.g. P: on DSC)
    pub const MS_DATA    : u8  = 2<<4;     // Data memory space (e.g. X: on DSC)
  //  pub const MS_GLOBAL  : u8  = 3<<4;     // HCS12 Global addresses (Using BDMPPR register)
    // Fast memory access for HCS08/HCS12 (stopped target, regs. are modified
   // pub const MS_FAST    : u8  = 1<<7;
    // Masks for above
   pub const MS_SIZE    : u8  = 0x7<<0;   // Size
    //pub const MS_SPACE   : u8  = 0x7<<4;   // Memory space
    // For convenience (DSC)
    pub const MS_PWORD   : u8  = MS_WORD + MS_PROGRAM;
    pub const MS_PLONG   : u8  = MS_LONG + MS_PROGRAM;
    pub const MS_XBYTE   : u8  = MS_LONG + MS_DATA;
    pub const MS_XWORD   : u8  = MS_WORD + MS_DATA;
    pub const MS_XLONG   : u8  = MS_LONG + MS_DATA;

use std::collections::HashMap;
type AddressKey       = u32;
type MemorySpaceType  = u8;
type HexMap = HashMap<AddressKey, MemorySpaceType>;

pub struct MemoryMap
{

  pub memory_size    : usize,  // MC56f80x memory map is linear and have one segment, so now we can do it simple way
  pub start_address  : u32,
  pub mem_space_type : HexMap, // use for conversion address->MemorySpaceType (MS_PWORD,MS_PLONG etc.). 

}

impl MemoryMap
{

 pub fn init_memory_map(mem_size : usize, start_addr : u32, hex_map : HexMap) -> Self {
  
      Self{
          memory_size     : mem_size,
          start_address   : start_addr,
          mem_space_type  : hex_map,
    }
}

  pub fn get_memory_space_type(&self, addr: AddressKey) -> Result<&MemorySpaceType, Error>
  {
    match self.mem_space_type.get(&addr)
    {
        Some(m_type) => 
        {
        println!("space type on this addr is: {:#02X}", m_type);
        Ok(m_type)
        }
        _ => 
        {
        println!("AddressKey not found!");
        Err(Error::PowerStateError)
        }
     }
  }

  fn memory_size(&self)           -> usize
  {

    self.memory_size

  }
  fn memory_start_address(&self)  -> u32
  {
   
    self.start_address

  }
}


 pub trait TargetFactory{

  type Output: TargetProgramming;

  fn create_target( prg : Programmer,  mem_size : usize, start_addr : u32, name : String) -> Self::Output;
  
}

 impl TargetFactory for MC56f80x {

  type Output = MC56f80x;
  
  fn create_target(prg : Programmer,  mem_size : usize, start_addr : u32, name : String) -> MC56f80x
   {
        
     let v: Vec<u8> = vec![0x00; mem_size]; // default HexBuffer size 0xFFFF, filled 0xFF
     let mut map: HexMap = HashMap::new();
     let mut address_index: AddressKey = start_addr;

     for byte_ in v.iter() {

         map.insert(address_index, MS_PWORD);
         address_index += 0x1;
    } 

    let m_map = MemoryMap::init_memory_map(mem_size, start_addr, map);


    return MC56f80x{programmer : prg, once_status : OnceStatus::UnknownMode, memory_map : m_map, mcu_name : name};
  }
}


pub struct MC56f80x {
    
   pub programmer    : Programmer,
   pub once_status   : OnceStatus,
   memory_map        : MemoryMap,
   mcu_name          : String, 
    
}


impl Drop for MC56f80x{

        fn drop(&mut self) {
            drop(&mut self.programmer);
            println!("Target dropped");
        }
}

pub trait TargetProgramming
{

/// Init
fn init(&mut self) -> Result<(), Error>;

/// Connect
fn connect(&mut self, power : TargetVddSelect ) -> Result<(), Error>;

/// Power. Toggled
fn power(&mut self, power : TargetVddSelect ) -> Result<(), Error>;

/// Disconnect
fn disconnect(&mut self);

/// Read target
fn read_target(&mut self, power : TargetVddSelect) -> Result<(), Error>;

/// Write target
fn write_target(&mut self, power : TargetVddSelect) -> Result<(), Error>;

/// Write target
fn erase_target(&mut self, power : TargetVddSelect) -> Result<(), Error>;

}

impl TargetProgramming for MC56f80x
{

fn init(&mut self) -> Result<(), Error>
{

  self.programmer.refresh_feedback()?;
  self.programmer.set_bdm_options()?;
  self.programmer.set_target_mc56f()?;

  Ok(())  

}

fn connect(&mut self, power : TargetVddSelect) -> Result<(), Error>
{

  self.programmer.target_power_reset()?;
  self.power(power);

  let master_id_code = read_master_id_code(true, &self.programmer)?;
  dbg!(master_id_code);
  enableCoreTAP(&self.programmer); // on second not
  let core_id_code = read_core_id_code(false, &self.programmer)?;
  dbg!(core_id_code);

  self.once_status = OnceStatus::UnknownMode;

  //while(self.once_status  != OnceStatus::DebugMode)
  //{
   dbg!(&self.once_status);
   self.once_status = targetDebugRequest(&self.programmer)?;
  //}

  self.once_status = enableONCE(&self.programmer)?;
  dbg!("Final status is: ", &self.once_status);
  
  if(self.once_status == OnceStatus::UnknownMode) 
  {

     return Err((Error::TargetNotConnected))

  }

  Ok(())
    
}

fn power(&mut self, user_power_query : TargetVddSelect) -> Result<(), Error>
{
                                                                        
  self.programmer.set_vdd(user_power_query);                      // If we try double-set power, filter in set_vdd just return ok
  self.programmer.check_expected_power(user_power_query)?;              // Check power is setted
  Ok(())

}


fn disconnect(&mut self) 
{
    
 drop(self);
  
}

fn read_target(&mut self, power : TargetVddSelect) -> Result<(), Error>
{
 
 
 self.connect(power)?;
 dbg!(&self.once_status);
 
 //let memory_read = self.programmer.read_memory_block(MS_PWORD, 0x20,  0x7000)?;
  let test_addr = 0x7000;
  let test_mem_access_type =  *self.memory_map.get_memory_space_type(test_addr)?;
  //let memory_read = self.programmer.read_memory_block(test_mem_access_type, 0x20,  test_addr)?;
  let memory_read = self.programmer.dsc_read_memory(test_mem_access_type, 0x60,  test_addr)?;
  let mut printed_vec = Vec::new();

 for byte in memory_read.iter()
 {
   let in_string = format!("{:02X}", byte);

   printed_vec.push(in_string);
   if(printed_vec.len() == 0x10)
   {

    for symbol in printed_vec.iter()
    {
      print!("{} ", symbol);
    }

    print!("\n");
    printed_vec.clear();

   }  
 }
 self.programmer.target_power_reset()?;
 self.programmer.refresh_feedback()?;
 self.power(TargetVddSelect::VddOff)?;

 Ok(())


}

fn write_target(&mut self, power : TargetVddSelect) -> Result<(), Error>
{
    
  self.connect(power)?; 
  dbg!(&self.once_status);

  let test_addr = 0x7000;
  let test_mem_access_type =  *self.memory_map.get_memory_space_type(test_addr)?;
  
  //self.write_memory_block(test_mem_access_type, )
  //let memory_read = self.programmer.read_memory_block(MS_PWORD, 0x20,  0x7000)?;
   
   //let memory_read = self.programmer.read_memory_block(test_mem_access_type, 0x20,  test_addr)?;
   let memory_read = self.programmer.dsc_read_memory(test_mem_access_type, 0x20,  test_addr)?;
   let mut printed_vec = Vec::new();
 
  for byte in memory_read.iter()
  {
    let in_string = format!("{:02X}", byte);
 
    printed_vec.push(in_string);
    if(printed_vec.len() == 0x0F)
    {
 
     for symbol in printed_vec.iter()
     {
       print!("{}", symbol);
     }
 
     print!("\n");
     printed_vec.clear();
 
    }  
  }
  self.programmer.target_power_reset()?;
  self.programmer.refresh_feedback()?;
  self.power(TargetVddSelect::VddOff)?;
 
  Ok(())
    
}

fn erase_target(&mut self, power : TargetVddSelect) -> Result<(), Error>
{
    
    unimplemented!()
    
}




}