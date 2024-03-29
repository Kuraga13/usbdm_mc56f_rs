use crate::errors::Error;
use crate::utils::*;
use crate::usbdm::jtag::*;
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use crate::usbdm::constants::{memory_space_t};
use super::flash_routine::FlashRoutine;
use super::target_init_actions::{MC56f80xx};
use super::memory_buffer::{MemoryBuffer};


use std::borrow::BorrowMut;
use std::io::Read;
use std::fmt;
use std::path::Path;
use core::ops::Range;
use serde::{Serialize, Deserialize, Deserializer};
use std::path::PathBuf;
use std::env;

type MemorySpace       = u8;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TargetSelector {

    Tester56f8035,
    Mc56f8002,
    Mc56f8006,
    Mc56f8011,
    Mc56f8013,
    Mc56f8025,
    Mc56f8035,

}

impl fmt::Display for TargetSelector {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      ///write!(f, "{:?}", self)
      // or test ::
       fmt::Debug::fmt(self, f)
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum DscFamily {

    Mc56f800X,
    Mc56f801X,
    Mc56f802X,
    Mc56f803X,

}

impl fmt::Display for DscFamily {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      ///write!(f, "{:?}", self)
      // or test ::
       fmt::Debug::fmt(self, f)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessType {

    MemoryX,
    MemoryP,

}

impl From<AccessType> for u8 {
  fn from(access : AccessType) -> u8 {
      match access { 
        AccessType::MemoryX => memory_space_t::MS_XWORD,
        AccessType::MemoryP => memory_space_t::MS_PWORD,
      }
   }
}



/// Ram segment
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RamSegement {

    /// A `name` to describe the region
    pub name: Option<String>,

    /// Address `range` of the region
    pub range: Range<u64>,

    /// byte, word X/P  `AccessType` of the region
    pub access_type: AccessType,


}

/// Data like Eeprom segment etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataSegment {

    /// A `name` to describe the region
    pub name: Option<String>,

    /// Address `range` of the region
    pub range: Range<u64>,

    pub access_type: AccessType,

}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProgrammSegment {

  /// A `name` to describe the region
  pub name: Option<String>,

  /// Address `range` of the region
  pub range: Range<u64>,

  pub access_type: AccessType,


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

const YAML_STR : &str = include_str!("../dsc_target/targets.yaml");

/// This describes a target import from Yaml, ser-de_ser fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetYaml {
  
    ///`dsc` - targets in Yaml database.
    pub dsc          : Vec<TargetBase>,

}

impl TargetYaml {

  pub fn init_target_db() -> Result<Self, Error> {

    let target_db: TargetYaml = serde_yaml::from_str(YAML_STR).expect("Failed deser from yaml");

    Ok(target_db)
  }
}



#[derive(Debug, Clone,PartialEq)]
pub enum SecurityStatus {
        
       Unknown,
       Secured,
       Unsecured,
    
}

#[derive(Debug, Clone,PartialEq)]
pub enum FlashModuleStatus {
        
       NotInited,
       Inited,
    
}


#[derive(Debug)]
pub struct TargetDsc {

    /// The `name` of the target.
    pub name               : String,
    /// `family` implemented trait with specific for target init actions
    pub family             : Box<dyn TargetInitActions>,
    /// `memory_map` of the target contain ranged Segment with MemorySpaceType.
    pub memory_map         : Vec<MemorySegment>,
    /// `memory_buffer` represent a FlashProgramm memory of Target
    pub memory_buffer      : MemoryBuffer,
    /// `flash_routine` pre-compiled and configured code for concrete target, assume load & execute for some programming task
    pub flash_routine      : FlashRoutine,
    /// `security_bytes`, security bytes sequense, for unsecuring-securing device ref datasheet
    pub security_bytes     : Vec<u8>,
    /// `security` status of target
    pub security           : SecurityStatus,
    /// `once_status` is status of once module 
    pub once_status        : OnceStatus,
    /// `flash_module` is status of flash module registers, needed for flash programming
    pub flash_module       : FlashModuleStatus,
    /// `image_path`, path to connection image
    pub image_path         : PathBuf,

}


impl TargetDsc {


  pub fn target_from_selector(selector : TargetSelector, target_db : TargetYaml) -> Result<Self, Error> {

    let dsc_list = target_db.dsc;

    let dsc = dsc_list
    .iter()
    .find(|dsc| dsc.name == selector)
    .ok_or_else(|| Error::InternalError("dsc from selector not found on target_db!".to_string()))
    .unwrap();

    dbg!(&dsc);

    let mut family_actions: Option<Box<dyn TargetInitActions>> = match dsc.family {

      DscFamily::Mc56f800X |  DscFamily::Mc56f801X  | DscFamily::Mc56f802X | DscFamily::Mc56f803X   => 
      {

        let init_type = Box::new(MC56f80xx::new(dsc.jtag_id_code, dsc.core_id_code, dsc.family)?);       
        Some(init_type)
      }
      _ =>   { None }      };


    let family_actions = family_actions.expect("Target family not found in match arm!");

    let ram_seg = dsc.memory_map.iter()
    .filter_map(|r| match r {
      MemorySegment::Ram(r) => Some(r),
      _ => None,
    })
    .next();
    let range = match ram_seg {
    Some(rs) => {  &rs.range  }
    None => {return Err(Error::InternalError("ram_range not found for DscTarget!".to_string())) } };

    let ram_size = (range.end - range.start  + 1) as u32;

    let prog_seg = dsc.memory_map.iter()
    .filter_map(|r| match r {
      MemorySegment::FlashProgramm(r) => Some(r),
      _ => None,
    })
    .next();
    let program_range = match prog_seg {
    Some(progs) => {  &progs.range  }
    None => {return Err(Error::InternalError("ram_range not found for DscTarget!".to_string())) } };

    
    let mut img_path = env::current_dir().expect("Current directory env err.");
    let image_folder_path = PathBuf::from(dsc.connection_image_path.clone());
    img_path.push(image_folder_path);
    img_path.push(dsc.name.to_string().to_uppercase());
    img_path.set_extension("jpeg");

    println!("The img_path is : {}", img_path.display());

    let flash_range : Range<usize> = Range{start: program_range.start as usize, end: program_range.end as usize};

    Ok(TargetDsc {

      name             : dsc.name.to_string(),
      family           : family_actions,
      memory_map       : dsc.memory_map.clone(), 
      memory_buffer    : MemoryBuffer::init_empty(0xFF, flash_range, 0x02),
      flash_routine    : FlashRoutine::init(dsc.family.clone(), ram_size)?, 
      security_bytes   : dsc.security_bytes.clone(),
      security         : SecurityStatus::Unknown,
      once_status      : OnceStatus::UnknownMode,
      flash_module     : FlashModuleStatus::NotInited,
      image_path       : img_path })
  }


    pub fn ram_range(&self) -> Result<&Range<u64>, Error> {
      let ram_seg = self.memory_map.iter()
      .filter_map(|r| match r {
        MemorySegment::Ram(r) => Some(r),
        _ => None,
      })
     .next();
     let range = match ram_seg {
      Some(rs) => {  &rs.range  }
      None => {return Err(Error::InternalError("ram_range not found for DscTarget!".to_string())) } };
     Ok(range)
    }

    pub fn programm_range(&self) -> Result<&Range<u64>, Error> {
      let prog_flash_seg = self.memory_map.iter()
      .filter_map(|r| match r {
        MemorySegment::FlashProgramm(r) => Some(r),
        _ => None,
      })
     .next();
     let range = match prog_flash_seg {
      Some(rs) => {  &rs.range  }
      None => {return Err(Error::InternalError("programm_range not found for DscTarget!".to_string())) } };
     Ok(range)
    }

    pub fn data_seg_range(&self) -> Result<&Range<u64>, Error> {
      let data_seg = self.memory_map.iter()
      .filter_map(|r| match r {
        MemorySegment::FlashProgramm(r) => Some(r),
        _ => None,})
     .next();
     let range = match data_seg {
      Some(rs) => {  &rs.range  }
      None => {return Err(Error::InternalError("data_seg_range not found for DscTarget!".to_string())) } };
     Ok(range)
    }


    fn check_range_is_programm_flash_memory(&mut self, range: Range<u64>) -> Result<(), Error> {
      let mut address = range.start;
      while address < range.end {
          match Self::address_in_segment(&self.memory_map, address, AccessType::MemoryP) {
              Some(MemorySegment::FlashProgramm(segment)) => address = segment.range.end,
              _ => {
                  return Err(Error::InternalError("Range not found in check_range_is_programm_flash_memory!".to_string())) }
              }
          }
      Ok(()) 
    }

    pub fn check_range_is_ram_memory(&mut self, range: Range<u64>, access : AccessType) -> Result<(), Error> {
      let mut address = range.start;
      while address < range.end {
          match Self::address_in_segment(&self.memory_map, address, access) {
              Some(MemorySegment::Ram(seg)) => address = seg.range.end,
              _ => {
                  return Err(Error::InternalError("Range not found in check_range_is_ram_memory!".to_string())) }
              }
          }
      
      Ok(()) 
    }

    fn address_in_segment( memory_map: &[MemorySegment], address: u64, access : AccessType) -> Option<&MemorySegment> {

      for segment in memory_map {
          let (r, access_seg) = match segment {
              MemorySegment::Ram(r) => (r.range.clone(), r.access_type.clone()),
              MemorySegment::FlashProgramm(r) => (r.range.clone(), r.access_type.clone()),
              MemorySegment::DataEeprom(r) => (r.range.clone(), r.access_type.clone()),
          };
          if access == access_seg && r.contains(&address) { 
            return Some(segment);}
      }
      None
   }



}

impl Drop for TargetDsc{
 
  fn drop(&mut self) {
      println!("TargetDsc dropped");
  }
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
fn write_target(&mut self, power : TargetVddSelect, address : u32, data_to_write : Vec<u8>,  prog : &mut Programmer) -> Result<(), Error>;

/// Verify target
fn verify_target(&mut self, power : TargetVddSelect, address : u32,  prog : &mut Programmer) -> Result<usize, Error>;

/// Write target
fn erase_target(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>;

}

pub trait TargetInitActions:  Send + std::fmt::Debug
{

/// `is_unsecure` - check Target unsecured, get Secure Status
fn is_unsecure(&mut self, prog : &mut Programmer) -> Result<SecurityStatus, Error>;

/// `target_family_confirmation` - check  is right Target selected
fn target_family_confirmation(&mut self, jtag_id : Vec<u8>, core_id : Vec<u8>)  -> Result<(), Error>;

/// Mass Erase specific on Dsc Target Family mass erase algorith
fn mass_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error>;

/// `init_for_write_erase` specific on Dsc Target Family algorith preapare & unlock flash for write & erase
fn init_for_write_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer, bus_freq : u32) -> Result<FlashModuleStatus, Error>;

}

///`deserialize_hex_line` casting byte array in str to Vec<u8>
/// 
/// `length` must be even!
/// 
/// for some weird reason `Yaml` doesn't want to do direct deserialization of the byte array
fn deserialize_hex_line<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    
    match s.len() % 2 {
      0 => println!("Length is even"),
      _ => println!("Length is odd! Error, need be handled!"),
   }

   let mut deser_vec = vec![];

   let mut local : Vec<u8> = s.as_bytes().to_vec();

   let mut iter = local.iter().enumerate();

   while let Some((pos, one_byte)) = iter.next() {
    let byte_1 = *one_byte;
    let byte_2 = *iter.next().unwrap().1;
    
    deser_vec.push(hex_to_byte(byte_1, byte_2)); }

    print_vec_one_line(&deser_vec);

    Ok(deser_vec)
}


///`hex_to_byte` - decode ASCII hex digit to byte 
/// 
/// `a`, `b` is digit coded in 2 ASCII symbol, need to be converted in 2 bytes
/// 
/// `return` decoded digit in one byte
pub fn hex_to_byte(a: u8, b: u8) -> u8 {

  let mut byte = vec![a, b];
  for x in byte.iter_mut() {
      if      *x >= b'0' && *x <= b'9' { *x -= b'0'; }
      else if *x >= b'a' && *x <= b'f' { *x -= b'a' - 10; }
      else if *x >= b'A' && *x <= b'F' { *x -= b'A' - 10;}
  }
  (byte[0] << 4) + byte[1]
}