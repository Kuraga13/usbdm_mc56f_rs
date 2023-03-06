use crate::errors::Error;
use crate::usbdm::jtag::*;
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use std::borrow::BorrowMut;

pub mod memory_space_type   
{ 
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
}

use std::collections::HashMap;
type AddressKey       = u32;
type MemorySpaceType  = u8;
type HexMap = HashMap<AddressKey, MemorySpaceType>; // map need to find memory type. On mc56f you have different access on memory space

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
        Err(Error::MemorySpaceTypeAddress_Out)
        }
     }
  }

pub fn memory_size(&self)           -> usize
{

    self.memory_size

}
pub fn memory_start_address(&self)  -> u32
{
   
    self.start_address

}

}


 pub trait TargetFactory{

  type Output: TargetProgramming;

  fn create_target( prg : Programmer,  mem_size : usize, start_addr : u32, name : String) -> Self::Output;
  
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
fn read_target(&mut self, power : TargetVddSelect, address : u32) -> Result<Vec<u8>, Error>;

/// Write target
fn write_target(&mut self, power : TargetVddSelect, data_to_write : Vec<u8>) -> Result<(), Error>;

/// Write target
fn erase_target(&mut self, power : TargetVddSelect) -> Result<(), Error>;

}

// TODO - Depreceated! A lot of device in one family have same SDID...
// we need this function to create target after connection with abstract DSC
//  This will require refactoring.
// It should work like this:
// 1. we have created an abstract DSC,
// 2. we connect with it
// 3. get SDID - NEED TO CHECK IS 
// (It is necessary to check that SDID detection by XML of the original USBDM really works!)
// 4. Target Factory return yous created concrete DSC with all parameters, specificallyЖ
// - MSHID (SIM_MSHID, SIM_LSHID) + function security_status_from_id_code (need customization from target type)
// - Memory MAP
// - trait TargetProgramming included all pre-implemented programming actions
//
//this will give us:
//you can not specify the target name manually, there will be logic like in cool programmer soft, the target name by id
pub fn dsc_target_from_yaml()
{

  unimplemented!()

}