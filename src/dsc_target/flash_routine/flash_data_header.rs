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
pub const OP_ERASE_BLOCK : u16 = DO_INIT_FLASH | DO_ERASE_BLOCK;
pub const OP_ERASE_RANGE : u16 = DO_INIT_FLASH | DO_ERASE_RANGE;
pub const OP_BLANK_CHECK : u16 = DO_INIT_FLASH | DO_BLANK_CHECK_RANGE; 
pub const OP_PROGRAM     : u16 = DO_INIT_FLASH | DO_BLANK_CHECK_RANGE | DO_PROGRAM_RANGE | DO_VERIFY_RANGE;
pub const OP_VERIFY      : u16 = DO_INIT_FLASH | DO_VERIFY_RANGE; 
pub const OP_TIMING      : u16 = DO_TIMING_LOOP;
pub const OP_NONE        : u16 = 0;

/// `DataHeader` 
/// 
/// this struct represent task with parameters we give to routine
/// 
/// it is stored in the header of the uploaded routine
///
/// orig name `LargeTargetFlashDataHeader` - Header at the start of flash programming buffer (controls program action)
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataHeader {
    /// Controls actions of routine
    pub flash_operation: u16,
    /// Error code from action
    pub error_code: u16,
    /// Ptr to flash controller
    pub controller: u32,
    /// Target frequency (kHz)
    pub frequency: u16,
    /// Size of Flash memory sectors (smallest erasable block)
    pub sector_size: u16,
    /// Memory address being accessed (reserved/page/address)
    pub address: u32,
    /// Size of memory range being accessed
    pub data_size: u16,
    pub pad: u16,
    /// Ptr to data to program
    pub data_address: u32,
 }

impl DataHeader {
    pub fn get(dsc_family : &DscFamily) -> Result<Self, Error> {
        let controller: u32 = DataHeader::get_flash_controller(dsc_family)?;
        let sector_size: u16 = DataHeader::get_sector_size(dsc_family)?;
        Ok(
            Self {
                flash_operation: OP_NONE,
                error_code: 0xFFFF,
                controller,
                frequency: 4000,
                sector_size,
                address: 0,
                data_size: 0,
                pad: 0,
                data_address: 0,
            }
        )
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, Error> {
        match bincode::serialize(&self) {
            Ok(x) => Ok(x),
            Err(_e) => Err(Error::InternalError("Serialization of RoutineDataHeader failed".to_string())),
        }
    }

    pub fn from_vec(vec: Vec<u8>) -> Result<Self, Error> {
        match bincode::deserialize(&vec) {
            Ok(x) => Ok(x),
            Err(_e) => Err(Error::InternalError("Deserialization of RoutineDataHeader failed".to_string())),
        }
    }

    pub fn len(&self) -> Result<u32, Error> {
        Ok(self.to_vec()?.len() as u32)
    }
} 

/// `TimingHeader`
/// 
/// Header at the start of timing data (controls program action & holds result)
/// 
/// orig name `LargeTargetTimingDataHeader`
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimingHeader {
    /// Controls actions of routine
    pub flash_operation: u16,
    /// Error code from action
    pub error_code: u16,
    /// Ptr to flash controller (unused)
    pub controller: u32,
    /// Timing count
    pub timing_count: u32,
}

impl Default for TimingHeader {
    fn default() -> Self {
        Self { 
            flash_operation: DO_TIMING_LOOP | IS_COMPLETE, // IS_COMPLETE as check - should be cleared
            error_code: 0xFFFF,
            controller: 0xFFFFFFFF, // Dummy value - not used 
            timing_count: 0,
        }
    }
}

impl TimingHeader {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routine_timing_header_check() {
        let timing_header: TimingHeader = TimingHeader{
            flash_operation: 0x0011,
            error_code: 0x2233,
            controller: 0x44556677,
            timing_count: 0x8899AABB, 
        };
        let timing_header_vec: Vec<u8> = vec![0x11, 0x00, 0x33, 0x22, 0x77, 0x66, 0x55, 0x44, 0xBB, 0xAA, 0x99, 0x88];

        // Test timing header serialization with to_vec()  
        assert_eq!(timing_header.to_vec().unwrap(), timing_header_vec);
        
        // Test timing header deserialization with from_vec()
        assert_eq!(TimingHeader::from_vec(timing_header_vec).unwrap(), timing_header);

        // Test timing header length with len()
        assert_eq!(timing_header.len().unwrap(), 12); 
    } 

    #[test]
    fn routine_data_header_check() {
        let data_header: DataHeader = DataHeader { 
            flash_operation: 0x0011,
            error_code: 0x2233,
            controller: 0x44556677,
            frequency: 0x8899,
            sector_size: 0xAABB,
            address: 0xCCDDEEFF,
            data_size: 0x0102,
            pad: 0x0304,
            data_address: 0x05060708,
        };
        let data_header_vec: Vec<u8> = vec![0x11, 0x00, 0x33, 0x22, 0x77, 0x66, 0x55, 0x44, 0x99, 0x88,
            0xBB, 0xAA, 0xFF, 0xEE, 0xDD, 0xCC, 0x02, 0x01, 0x04, 0x03, 0x08, 0x07, 0x06, 0x05];

        // Test data header serialization with to_vec()
        assert_eq!(data_header.to_vec().unwrap(), data_header_vec);

        // Test data header deserialization with from_vec()
        assert_eq!(DataHeader::from_vec(data_header_vec).unwrap(), data_header);

        // Test data header length with len()
        assert_eq!(data_header.len().unwrap(), 24); 
    }
}
