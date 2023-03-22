#![allow(non_camel_case_types)]

use super::*;

// Flash operation masks
const DO_INIT_FLASH        : u16 = 1<<0; // Do initialisation of flash
const DO_ERASE_BLOCK       : u16 = 1<<1; // Erase entire flash block e.g. Flash, FlexNVM etc
const DO_ERASE_RANGE       : u16 = 1<<2; // Erase range (including option region)
const DO_BLANK_CHECK_RANGE : u16 = 1<<3; // Blank check region
const DO_PROGRAM_RANGE     : u16 = 1<<4; // Program range (including option region)
const DO_VERIFY_RANGE      : u16 = 1<<5; // Verify range
const DO_PARTITION_FLEXNVM : u16 = 1<<7; // Program FlexNVM DFLASH/EEPROM partitioning
const DO_TIMING_LOOP       : u16 = 1<<8; // Counting loop to determine clock speed
const IS_COMPLETE          : u16 = 1<<15;

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
