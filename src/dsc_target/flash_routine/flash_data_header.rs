#![allow(non_camel_case_types)]

use super::*;

// Flash operation masks
pub const DO_INIT_FLASH        : u16 = 1<<0;  // Do initialisation of flash
pub const DO_ERASE_BLOCK       : u16 = 1<<1;  // Erase entire flash block e.g. Flash, FlexNVM etc
pub const DO_ERASE_RANGE       : u16 = 1<<2;  // Erase range (including option region)
pub const DO_BLANK_CHECK_RANGE : u16 = 1<<3;  // Blank check region
pub const DO_PROGRAM_RANGE     : u16 = 1<<4;  // Program range (including option region)
pub const DO_VERIFY_RANGE      : u16 = 1<<5;  // Verify range
pub const DO_PARTITION_FLEXNVM : u16 = 1<<7;  // Program FlexNVM DFLASH/EEPROM partitioning
pub const DO_TIMING_LOOP       : u16 = 1<<8;  // Counting loop to determine clock speed
pub const IS_COMPLETE          : u16 = 1<<15; // Completion flag, routine must clear it

/// Different flash operations
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlashOperation {
    OpEraseBlock = DO_INIT_FLASH | DO_ERASE_BLOCK,
    OpEraseRange = DO_INIT_FLASH | DO_ERASE_RANGE, 
    OpBlankCheck = DO_INIT_FLASH | DO_BLANK_CHECK_RANGE, 
    OpProgram    = DO_INIT_FLASH | DO_BLANK_CHECK_RANGE | DO_PROGRAM_RANGE | DO_VERIFY_RANGE, 
    OpVerify     = DO_INIT_FLASH | DO_VERIFY_RANGE, 
    OpTiming     = DO_TIMING_LOOP,
    OpNone       = 0, 
}


/// `RoutineFlashTask` 
/// 
/// this struct represent task with parameters we give to routine
/// 
/// it is stored in the header of the uploaded routine
///
/// orig name `LargeTargetFlashDataHeader` - Header at the start of flash programming buffer (controls program action)
#[derive(Debug, Serialize, Deserialize)]
struct RoutineFlashTask {
    /// Controls actions of routine
    flash_operation: u16,
    /// Error code from action
    error_code: u16,
    /// Ptr to flash controller
    controller: u32,
    /// Target frequency (kHz)
    frequency: u16,
    /// Size of Flash memory sectors (smallest erasable block)
    sector_size: u16,
    /// Memory address being accessed (reserved/page/address)
    address: u32,
    /// Size of memory range being accessed
    data_size: u16,
    pad: u16,
    /// Ptr to data to program
    data_address: u32,
 }

impl RoutineFlashTask {
    pub fn to_vec(&self) -> Result<Vec<u8>, Error> {
        match bincode::serialize(&self) {
            Ok(x) => Ok(x),
            Err(_e) => Err(Error::InternalError("Serialization of RoutineFlashTask failed".to_string())),
        }
    }
} 

/// `TimingRoutineHeader`
/// 
/// Header at the start of timing data (controls program action & holds result)
/// 
/// orig name `LargeTargetTimingDataHeader`
#[derive(Debug, Serialize, Deserialize)]
pub struct TimingRoutineHeader {
    /// Controls actions of routine
    pub flash_operation: u16,
    /// Error code from action
    pub error_code: u16,
    /// Ptr to flash controller (unused)
    pub controller: u32,
    /// Timing count
    pub timing_count: u32,
}

impl Default for TimingRoutineHeader {
    fn default() -> Self {
        Self { 
            flash_operation: DO_TIMING_LOOP | IS_COMPLETE, // IS_COMPLETE as check - should be cleared
            error_code: 0xFFFF,
            controller: 0xFFFF, // Dummy value - not used 
            timing_count: 0,
        }
    }
}

impl TimingRoutineHeader {
    pub fn get() -> Self {
        Self::default()
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, Error> {
        match bincode::serialize(&self) {
            Ok(x) => Ok(x),
            Err(_e) => Err(Error::InternalError("Serialization of RoutineTimimgTask failed".to_string())),
        }
    }

    pub fn from_vec(vec: Vec<u8>) -> Result<Self, Error> {
        match bincode::deserialize(&vec) {
            Ok(x) => Ok(x),
            Err(_e) => Err(Error::InternalError("Deserialization of RoutineTimimgTask failed".to_string())),
        }
    }

    pub fn len(&self) -> Result<u32, Error> {
        Ok(self.to_vec()?.len() as u32)
    }
} 

// Error codes return from the flash driver
pub const FLASH_ERR_OK               : u16 = 0;  // No error
pub const FLASH_ERR_LOCKED           : u16 = 1;  // Flash is still locked
pub const FLASH_ERR_ILLEGAL_PARAMS   : u16 = 2;  // Parameters illegal
pub const FLASH_ERR_PROG_FAILED      : u16 = 3;  // STM - Programming operation failed - general
pub const FLASH_ERR_PROG_WPROT       : u16 = 4;  // STM - Programming operation failed - write protected
pub const FLASH_ERR_VERIFY_FAILED    : u16 = 5;  // Verify failed
pub const FLASH_ERR_ERASE_FAILED     : u16 = 6;  // Erase or Blank Check failed
pub const FLASH_ERR_TRAP             : u16 = 7;  // Program trapped (illegal instruction/location etc.)
pub const FLASH_ERR_PROG_ACCERR      : u16 = 8;  // Kinetis/CFVx - Programming operation failed - ACCERR
pub const FLASH_ERR_PROG_FPVIOL      : u16 = 9;  // Kinetis/CFVx - Programming operation failed - FPVIOL
pub const FLASH_ERR_PROG_MGSTAT0     : u16 = 10; // Kinetis - Programming operation failed - MGSTAT0
pub const FLASH_ERR_CLKDIV           : u16 = 11; // CFVx - Clock divider not set
pub const FLASH_ERR_ILLEGAL_SECURITY : u16 = 12; // Kinetis - Illegal value for security location
pub const FLASH_ERR_UNKNOWN          : u16 = 13; // Unspecified error
pub const FLASH_ERR_TIMEOUT          : u16 = 14; // Timeout waiting for completion

pub fn parse_flash_err(error: u16) -> String {
    match error {
        FLASH_ERR_OK                => String::from("No error"),
        FLASH_ERR_LOCKED            => String::from("Flash is still locked"),
        FLASH_ERR_ILLEGAL_PARAMS    => String::from("Parameters illegal"),
        FLASH_ERR_PROG_FAILED       => String::from("STM - Programming operation failed - general"),
        FLASH_ERR_PROG_WPROT        => String::from("STM - Programming operation failed - write protected"),
        FLASH_ERR_VERIFY_FAILED     => String::from("Verify failed"),
        FLASH_ERR_ERASE_FAILED      => String::from("Erase or Blank Check failed"),
        FLASH_ERR_TRAP              => String::from("Program trapped (illegal instruction/location etc.)"),
        FLASH_ERR_PROG_ACCERR       => String::from("Kinetis/CFVx - Programming operation failed - ACCERR"),
        FLASH_ERR_PROG_FPVIOL       => String::from("Kinetis/CFVx - Programming operation failed - FPVIOL"),
        FLASH_ERR_PROG_MGSTAT0      => String::from("Kinetis - Programming operation failed - MGSTAT0"),
        FLASH_ERR_CLKDIV            => String::from("CFVx - Clock divider not set"),
        FLASH_ERR_ILLEGAL_SECURITY  => String::from("Kinetis - Illegal value for security location"),
        FLASH_ERR_UNKNOWN           => String::from("Unspecified error"),
        FLASH_ERR_TIMEOUT           => String::from("Timeout waiting for completion"),
        _                           => String::from("Unexpected Error Value"),
    }

}
