use crate::errors::Error;
use crate::usbdm::jtag::*;
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use crate::usbdm::constants::{memory_space_t};
use super::flash_routine::FlashRoutine;
use super::target_init_actions::{MC56f801x,MC56f802x};
//use crate::target_dsc::mc56f802x::MC56f802x;
//use crate::target_dsc::mc56f801x::MC56f801x;

use std::borrow::BorrowMut;
use std::io::Read;
use std::path::Path;
use core::ops::Range;
use serde::{Serialize, Deserialize, Deserializer};

type MemorySpace       = u8;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TargetSelector {

    Mc56f8011,
    Mc56f8025,
    Mc56f8035,

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DscFamily {

    Mc56f800X,
    Mc56f801X,
    Mc56f802X,
    Mc56f803X,

}



/// Ram segment
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RamSegement {

    /// A `name` to describe the region
    pub name: Option<String>,

    /// Address `range` of the region
    pub range: Range<u64>,


}

/// Data like Eeprom segment etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataSegment {

    /// A `name` to describe the region
    pub name: Option<String>,

    /// Address `range` of the region
    pub range: Range<u64>,

}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProgrammSegment {

  /// A `name` to describe the region
  pub name: Option<String>,

  /// Address `range` of the region
  pub range: Range<u64>,


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
pub struct TargetBase {
  
    /// The name of the target.
    pub name               : TargetSelector,
    /// `family `specify TargetProgramming trait by enum DscFamily
    pub family             : DscFamily,
    /// `base_routine_path` base pre-compiled routine path
    pub base_routine_path  : String,
    /// `jtag_id_code` id code from dsc, for example  MC56802X_SIM_ID =  0x01F2801D, get by fn read_master_id_code_DSC_JTAG_ID()
    pub jtag_id_code       : u32,
    /// `core id code`, orig pjt `sdid`, for example  mc5680xx(23-35) =  0x02211004, get by fn read_core_id_code().
    pub core_id_code       : u32,
    /// `security_bytes`, security bytes sequense, for unsecuring-securing device ref datasheet
    #[serde(deserialize_with = "deserialize_hex_line")]
    pub security_bytes     : Vec<u8>,
    /// `connection_image_path`, specify path to connection image
    pub connection_image_path     : String,
    /// The memory map of the target.
    pub memory_map         : Vec<MemorySegment>,
}

/// This describes a target import from Yaml, ser-de_ser fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetYaml {
  
    ///`dsc` - targets in Yaml database.
    pub dsc          : Vec<TargetBase>,

}



#[derive(Debug, Clone,PartialEq)]
pub enum SecurityStatus {
        
       Unknown,
       Secured,
       Unsecured,
    
}



#[derive(Debug)]
pub struct FamilyDsc {

  inner: Box<dyn TargetInitActions>,

}


impl FamilyDsc {


 /// Create a new FamilyDsc from preapered FamilyDsc
 pub fn new(dsc_family: impl TargetInitActions + 'static) -> Self {
      Self {
      inner: Box::new(dsc_family),
      }  
  }
}

#[derive(Debug)]
pub struct TargetDsc {

    /// The `name` of the target.
    pub name               : String,
    /// `family` implemented trait with specific for target init actions
    pub family             : Box<dyn TargetInitActions>,
    /// `core_id` we should get it by command read_core_id_code.
    pub core_id            : u32,
    /// `jtag_id_code` we should get it by command jtag_id_code, use for security status check.
    pub jtag_id_code       : u32,
    /// `memory_map` of the target contain ranged Segment with MemorySpaceType.
    pub memory_map         : Vec<MemorySegment>,
    /// `flash_routine` pre-compiled and configured code for concrete target, assume load & execute for some programming task
    pub flash_routine      : FlashRoutine,
    /// `security_bytes`, security bytes sequense, for unsecuring-securing device ref datasheet
    pub security_bytes     : Vec<u8>,
    /// `security` status of target
    pub security           : SecurityStatus,
    /// `once_status` is status of once module 
    pub once_status        : OnceStatus,
    /// `image_path`, path to connection image
    pub image_path         : String,

}

const YAML_STR : &str = include_str!("../dsc_target/targets.yaml");


impl TargetDsc {


  pub fn create_target_from_selector(selector : TargetSelector) -> Result<Self, Error> {

    

    let target_from_yaml: TargetYaml = serde_yaml::from_str(YAML_STR).expect("Failed deser from yaml");

    dbg!(&target_from_yaml.dsc[0]);

    let dsc_list = target_from_yaml.dsc;


    let dsc = dsc_list
    .iter()
    .find(|dsc| dsc.name == selector)
    .expect("Target Not found im Yaml ");
    //.ok_or_else(|| RegistryError::ChipNotFound(chip_name.as_ref().to_string()))?;

    dbg!(&dsc);

    let mut family_actions: Option<Box<dyn TargetInitActions>> = match dsc.family
    {

      DscFamily::Mc56f800X  => 
      {

        let init_type = Box::new(MC56f801x);
        Some(init_type)
      }

      DscFamily::Mc56f801X  => 
      {

        let init_type = Box::new(MC56f801x);
        Some(init_type)
      }

      DscFamily::Mc56f802X => 
      {
        let init_type = Box::new(MC56f802x);
        Some(init_type)
      } 
      
      DscFamily::Mc56f803X => 
      {
        let init_type = Box::new(MC56f802x);
        Some(init_type)
      }   
      _ =>   { None }       

    };

    let family_actions = family_actions.expect("Target family not found in match arm!");

    let dsc_name = stringify!(dsc.name).to_string();

    Ok(TargetDsc {

      name             : dsc_name,
      family           : family_actions,
      core_id          : dsc.core_id_code, 
      jtag_id_code     : dsc.jtag_id_code,
      memory_map       : dsc.memory_map.clone(), 
      flash_routine    : FlashRoutine::init(dsc.family.clone())?, 
      security_bytes   : dsc.security_bytes.clone(),
      security         : SecurityStatus::Unknown,
      once_status      : OnceStatus::UnknownMode,
      image_path       : dsc.connection_image_path.clone() })
  }


     
  pub fn get_ram_range(&self) -> Result<&Range<u64>, Error> {
        
    let ram_seg = self.memory_map.iter()
        .filter_map(|r| match r {
          MemorySegment::Ram(r) => Some(r),
          _ => None,
      })
      .next();

    match ram_seg {

      Some(rs) => {  Ok(&rs.range)  }
      None => {return Err(Error::InternalError("Ram Segment not found for DscTarget!".to_string())) } }
 
     
    }







}

impl Drop for TargetDsc{
 
  fn drop(&mut self) {
      println!("TargetDsc dropped");
  }
}






/////////////////////////////////////

// old variant ->
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

pub trait TargetInitActions:  Send + std::fmt::Debug
{

/// `is_unsecure` - check Target unsecured, get Secure Status
///
/// 
/// This read-only register, in two parts  displays the least significant half of the JTAG ID for the chip.
/// 
/// For example:
/// 
/// Most Significant Half of JTAG ID (`SIM_MSHID`), in MC56f801x is `$01F2`.
/// 
/// Least Significant Half of JTAG ID (`SIM_LSHID`), in MC56f801x is  `$401D`.
/// 
/// PGO wrote in original usbdm pjt, if you have match id code dsc in
/// we have to match `jtag_id_code` with `SIM_ID`
fn is_unsecure(&mut self, prog : &mut Programmer, jtag_id_code_vec : Vec<u8>, expected_id : u32) -> Result<SecurityStatus, Error>;

/// Mass Erase specific on Dsc Target Family mass erase algorith
fn mass_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>;

/// Calculate specific on Dsc Target Family cfmclkd
fn calculate_flash_divider(&mut self, power : TargetVddSelect, prog : &mut Programmer, bus_frequency : u32) -> Result<u32, Error>;

/// Init specific on Dsc Target Family algorith
fn target_init(&mut self, power : TargetVddSelect, prog : &mut Programmer);

}


fn deserialize_hex_line<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    //let hex_bytes2: Vec<u8> = s.as_bytes().to_vec();
    let chars: Vec<char> = s.chars().collect::<Vec<_>>();

    let hex_bytes: Vec<u8> = chars.iter().map(|c| *c as u8).collect::<Vec<_>>();

    Ok(hex_bytes)
}