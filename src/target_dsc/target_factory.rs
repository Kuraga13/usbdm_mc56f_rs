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


type MemorySpace       = u8;

/// Ram segment
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RamSegement {

    /// A `name` to describe the region
    pub name: Option<String>,

    /// Address `range` of the region
    pub range: Range<u64>,

    /// Type of `access` (MemorySpaceType). map need to find memory type. On mc56f you have different access on memory space
    pub access : MemorySpace, 

}

/// Data like Eeprom segment etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataSegment {

    /// A `name` to describe the region
    pub name: Option<String>,

    /// Address `range` of the region
    pub range: Range<u64>,

    /// Type of `access` (MemorySpaceType). map need to find memory type. On mc56f you have different access on memory space
    pub access : MemorySpace, 

}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProgrammSegment {

  /// A `name` to describe the region
  pub name: Option<String>,

  /// Address `range` of the region
  pub range: Range<u64>,

  /// Type of `access` (MemorySpaceType). map need to find memory type. On mc56f you have different access on memory space
  pub access : MemorySpace, 

}


/// Declares the type of a memory region.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemorySegment {

    Ram(RamSegement),

    DataEeprom(DataSegment),

    FlashProgramm(ProgrammSegment),

    
}




/// This describes a target import from Yaml, ser-de_ser fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetYaml {
    /// The name of the target.
    #[serde(default)]
    pub name               : String,
    /// The memory map of the target.
    pub memory_map         : Vec<MemorySegment>,
    /// `base_routine_path` base pre-compiled routine path
    pub base_routine_path  : String,
    /// `flash_routine` configured and builded for some task
    pub flash_routine      : FlashRoutine,
    /// `jtag_id_code` id code from dsc, for example  MC5680XX_SIM_ID =  0x01F2801D, get by fn read_master_id_code_DSC_JTAG_ID()
    pub jtag_id_code       : u32,
    /// `core id code`, orig pjt `sdid`, for example  mc5680xx(23-35) =  0x02211004, get by fn read_core_id_code().
    pub core_id_code       : u32,
    /// `security_bytes`, security bytes sequense, for unsecuring-securing device ref datasheet
    pub security_bytes     : Vec<u8>,
 

}

#[derive(Debug, Clone,PartialEq)]
pub enum SecurityStatus {
        
       Unknown,
       Secured,
       Unsecured,
    
}
#[derive(Debug, Clone)]
pub struct TargetState
{
  pub once_status   : OnceStatus,
  pub security      : SecurityStatus,
}

impl Default for TargetState
{
    fn default() -> Self { 

       TargetState {

        once_status : OnceStatus::UnknownMode,
        security    : SecurityStatus::Unknown,
    }
  } 
}

/// This describes a complete target with a fixed chip model and variant.
#[derive(Debug, Clone)]
pub struct TargetDsc {

    /// The name of the target.
    pub name               : String,
    /// The memory map of the target.
    pub memory_map         : Vec<MemorySegment>,
    /// `flash_routine` configured and builded for some task
    pub flash_routine      : FlashRoutine,
    /// `jtag_id_code` id code from dsc, for example  MC5680XX_SIM_ID =  0x01F2801D, get by fn read_master_id_code_DSC_JTAG_ID()
    pub jtag_id_code       : u32,
    /// `core id code`, orig pjt `sdid`, for example  mc5680xx(23-35) =  0x02211004, get by fn read_core_id_code().
    pub core_id_code       : u32,
    /// `security_bytes`, security bytes sequense, for unsecuring-securing device ref datasheet
    pub security_bytes     : Vec<u8>,
    /// `dsc_state`, all state data needed for programming
    pub dsc_state          : TargetState,

}

// TODO 
// 1. deal with data in old TCL's
//2. Implement them either
//-trait TP-gimng
//- data from yaml?
//3. Put it all together
//- build specific target from yaml
//- building her flash routine
//- applying trait methods

/////////////////////////////////////

// current variant ->
//
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

  fn create_target(mem_size : usize, start_addr : u32, name : String) -> Self::Output;
  
}


pub trait TargetProgramming
{

/// Init
fn init(&mut self, prog : &mut Programmer) -> Result<(), Error>;

/// Connect
fn connect(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>;

/// Power. Toggled
fn power(&mut self, power : TargetVddSelect, prog : &mut Programmer ) -> Result<(), Error>;

/// Disconnect
fn disconnect(&mut self);

/// Read target
fn read_target(&mut self, power : TargetVddSelect, address : u32, prog : &mut Programmer) -> Result<Vec<u8>, Error>;

/// Write target
fn write_target(&mut self, power : TargetVddSelect, data_to_write : Vec<u8>,  prog : &mut Programmer) -> Result<(), Error>;

/// Write target
fn erase_target(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>;

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