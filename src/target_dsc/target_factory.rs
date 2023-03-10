use crate::errors::Error;
use crate::usbdm::jtag::*;
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use crate::usbdm::constants::{memory_space_t};
use super::flash_routine::{FlashRoutine};

use std::borrow::BorrowMut;
use core::ops::Range;

// new variant -> in Work, is draft! 
use serde::{Serialize, Deserialize};
// map need to find memory type. On mc56f you have different access on memory space
use std::collections::HashMap;
type MemorySpace       = u8;


//#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Ram segment
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RamSegement {

    /// A name to describe the region
    pub name: Option<String>,

    /// Address range of the region
    pub range: Range<u64>,

}

/// Data like Eeprom segment etc.
//#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataSegment {

    /// A name to describe the region
    pub name: Option<String>,

    /// Address range of the region
    pub range: Range<u64>,

}

//#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProgrammSegment {

  /// A name to describe the region
  pub name: Option<String>,

  /// Address range of the region
  pub range: Range<u64>,

}


/// Declares the type of a memory region.
//#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemorySegment {

    Ram(RamSegement),

    DataEeprom(DataSegment),

    FlashProgramm(ProgrammSegment),
}






/// This describes a complete target with a fixed chip model and variant.
#[derive(Clone)]
pub struct TargetDsc {
    /// The name of the target.
    pub name: String,
    /// The memory map of the target.
    pub memory_map: Vec<MemorySegment>,
    // Type of access (MemorySpaceType) of each segment in HexMap
    pub mem_space_type : HashMap<MemorySegment, MemorySpace>,
    /// The name of the flash algorithm.
    pub flash_routine: Vec<FlashRoutine>,

}


// current variant ->
//

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