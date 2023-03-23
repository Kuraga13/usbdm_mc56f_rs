#![allow(non_camel_case_types)]

use super::*;

// Flash operation masks
const DO_INIT_FLASH        : u16 = 1<<0;  // Do initialisation of flash
const DO_ERASE_BLOCK       : u16 = 1<<1;  // Erase entire flash block e.g. Flash, FlexNVM etc
const DO_ERASE_RANGE       : u16 = 1<<2;  // Erase range (including option region)
const DO_BLANK_CHECK_RANGE : u16 = 1<<3;  // Blank check region
const DO_PROGRAM_RANGE     : u16 = 1<<4;  // Program range (including option region)
const DO_VERIFY_RANGE      : u16 = 1<<5;  // Verify range
const DO_PARTITION_FLEXNVM : u16 = 1<<7;  // Program FlexNVM DFLASH/EEPROM partitioning
const DO_TIMING_LOOP       : u16 = 1<<8;  // Counting loop to determine clock speed
const IS_COMPLETE          : u16 = 1<<15; // Completion flag, routine must clear it

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


/// `RoutineTimimgTask`
/// 
/// Header at the start of timing data (controls program action & holds result)
/// 
/// orig name `LargeTargetTimingDataHeader`
#[derive(Debug, Serialize, Deserialize)]
struct RoutineTimimgTask {
    /// Controls actions of routine
    flash_operation: u16,
    /// Error code from action
    error_code: u16,
    /// Ptr to flash controller (unused)
    controller: u32,
    /// Timing count
    timing_count: u32,
}

impl RoutineTimimgTask {
    pub fn to_vec(&self) -> Result<Vec<u8>, Error> {
        match bincode::serialize(&self) {
            Ok(x) => Ok(x),
            Err(_e) => Err(Error::InternalError("Serialization of RoutineTimimgTask failed".to_string())),
        }
    }
} 

