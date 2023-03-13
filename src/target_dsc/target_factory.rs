use crate::errors::Error;
use crate::usbdm::jtag::*;
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use crate::usbdm::constants::{memory_space_t};
use super::flash_routine::{FlashRoutine};
use crate::target_dsc::mc56f802x::MC56f802x;
use crate::target_dsc::mc56f801x::MC56f801x;

use std::borrow::BorrowMut;
use std::io::Read;
use std::path::Path;
use core::ops::Range;
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
    /// `family `specify TargetProgramming trait
    #[serde(default)]
    pub family             : String,
    /// The memory map of the target.
    pub memory_map         : Vec<MemorySegment>,
    /// `base_routine_path` base pre-compiled routine path
    pub base_routine_path  : String,
    /// `jtag_id_code` id code from dsc, for example  MC56802X_SIM_ID =  0x01F2801D, get by fn read_master_id_code_DSC_JTAG_ID()
    pub jtag_id_code       : u32,
    /// `core id code`, orig pjt `sdid`, for example  mc5680xx(23-35) =  0x02211004, get by fn read_core_id_code().
    pub core_id_code       : u32,
    /// `security_bytes`, security bytes sequense, for unsecuring-securing device ref datasheet
    pub security_bytes     : Vec<u8>,
    /// `connection_image_path`, specify path to connection image
    pub connection_image_path     : String,
}




#[derive(Debug, Clone,PartialEq)]
pub enum SecurityStatus {
        
       Unknown,
       Secured,
       Unsecured,
    
}


pub enum TargetSelector
{

  Mc56f8011,
  Mc56f8023,
  Mc56f8035,

}


/// This describes a complete target with a fixed chip model and variant.
#[derive(Debug)]
pub struct TargetDsc {

  inner: Box<dyn TargetProgramming>,
  
}

impl TargetDsc 
{

 /// Create a new dsc from preapered dsc
 pub fn new(dsc: impl TargetProgramming + 'static) -> Self {
        Self {
            inner: Box::new(dsc),
        }
   }

 pub fn create_target_from_selector(selector : TargetSelector) -> Self {

      let test_yaml =  Path::new("target_yaml_path");
      let f = std::fs::File::open(test_yaml).unwrap();
      let target_from_yaml: TargetYaml = serde_yaml::from_reader(f).unwrap();


      let dsc_family = target_from_yaml.family.clone();


      let programming_type: Option<Box<dyn TargetProgramming>> = match dsc_family.as_str()
      {
        "801x"  => 
        {

          let prg_type =  Box::new( 
            MC56f801x {
            name             : target_from_yaml.name.clone(), 
            core_id          : target_from_yaml.core_id_code, 
            memory_map       : target_from_yaml.memory_map.clone(), 
            flash_routine    : FlashRoutine::build_base_routine(target_from_yaml.base_routine_path.clone()).unwrap(), 
            security_bytes   : target_from_yaml.security_bytes.clone(),
            security         : SecurityStatus::Unknown,
            once_status      : OnceStatus::UnknownMode,
            image_path       : target_from_yaml.connection_image_path.clone()});

          Some(prg_type)

        }

        "802x" => 
        {
          let prg_type =  Box::new( 
            MC56f802x {
            name             : target_from_yaml.name.clone(), 
            core_id          : target_from_yaml.core_id_code, 
            memory_map       : target_from_yaml.memory_map.clone(), 
            flash_routine    : FlashRoutine::build_base_routine(target_from_yaml.base_routine_path.clone()).unwrap(), 
            security_bytes   : target_from_yaml.security_bytes.clone(),
            security         : SecurityStatus::Unknown,
            once_status      : OnceStatus::UnknownMode, 
            image_path       : target_from_yaml.connection_image_path.clone()});
            Some(prg_type)
        }
        _ => 
        {
          None
        }
        
      };

      let configured_dsc = programming_type.expect("Target not found in Yaml!");
      
      TargetDsc {

        inner : configured_dsc }
  }

 
 pub fn read(&mut self, power : TargetVddSelect, address : u32, prog : &mut Programmer) -> Result<Vec<u8>, Error> {

    let dump = self.inner.read_target(power, address, prog)?;
    Ok(dump)

  }
}







/////////////////////////////////////

// current variant ->
//
use std::collections::HashMap;
type AddressKey       = u32;
type MemorySpaceType  = u8;
type HexMap = HashMap<AddressKey, MemorySpaceType>; // map need to find memory type. On mc56f you have different access on memory space

#[derive(Debug, Clone)]
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

pub trait TargetProgramming:  Send + std::fmt::Debug
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

