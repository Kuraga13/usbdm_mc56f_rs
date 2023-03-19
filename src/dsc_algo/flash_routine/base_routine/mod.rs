#![allow(non_camel_case_types)]

use super::*;
use crate::file_buffer::data_parser::ParsedData;

const DSC_56F8006_FLASH_PROG: &[u8] = include_bytes!(r"DSC-56F8006-flash-prog.elf.p.S");
const DSC_56F8014_FLASH_PROG: &[u8] = include_bytes!(r"DSC-56F8014-flash-prog.elf.p.S");
const DSC_56F8023_FLASH_PROG: &[u8] = include_bytes!(r"DSC-56F8023-flash-prog.elf.p.S");
//const DSC_56F825X_FLASH_PROG: &[u8] = include_bytes!(r"DSC-56F825X-flash-prog.elf.p.S");
//const DSC_56F8323_FLASH_PROG: &[u8] = include_bytes!(r"DSC-56F8323-flash-prog.elf.p.S");
//const DSC_FTFA_FLASH_PROG: &[u8] = include_bytes!(r"DSC-FTFA-flash-prog.elf.p.S");
//const DSC_FTFL_FLASH_PROG: &[u8] = include_bytes!(r"DSC-FTFL-flash-prog.elf.p.S");

// Capability masks
const CAP_ERASE_BLOCK       : u16 = 1<<1;
const CAP_ERASE_RANGE       : u16 = 1<<2;
const CAP_BLANK_CHECK_RANGE : u16 = 1<<3;
const CAP_PROGRAM_RANGE     : u16 = 1<<4;
const CAP_VERIFY_RANGE      : u16 = 1<<5;
const CAP_PARTITION_FLEXNVM : u16 = 1<<7;
const CAP_TIMING            : u16 = 1<<8;
const CAP_DSC_OVERLAY       : u16 = 1<<11; // Indicates DSC code in pMEM overlays xRAM
const CAP_DATA_FIXED        : u16 = 1<<12; // Indicates TargetFlashDataHeader is at fixed address
const CAP_RELOCATABLE       : u16 = 1<<15; // Code may be relocated

/// Contains all information about base flash routine
pub struct BaseRoutine {
    /// Describes family of processors this flash routine is for
    family: BaseRoutineFamily,
    /// Vec<u8> containing flash routine
    routine: Vec<u8>,
    /// Address where this routine is loaded to (in 2 byte words)
    code_load_address: u32,
    /// Address for execution start (in 2 byte words)
    code_entry: u32,
    /// Capabilities of routine
    capabilities: u16,
    /// Frequency (kHz) used for calibFactor
    calib_frequency: u16,
    /// Calibration factor for speed determination
    calib_factor: u32,
    /// Address where to load flash data for execution (in 2 byte words)
    flash_data: u32,
}

pub enum BaseRoutineFamily {
    DSC_56F8006,
    DSC_56F8014,
    DSC_56F8023,
    //DSC_56F825X, 
    //DSC_56F8323,
    //DSC_FTFA,
    //DSC_FTFL,
    NONE,
}

impl Default for BaseRoutine {
    fn default() -> Self { 
        BaseRoutine {
            family: BaseRoutineFamily::NONE,
            routine: vec![],
            code_load_address: 0,
            code_entry: 0,
            capabilities: 0,
            calib_frequency: 0,
            calib_factor: 0,
            flash_data: 0,
        }
    } 
}

impl BaseRoutine
{
    pub fn new(base_routine_family : BaseRoutineFamily) -> Result<Self, Error> {
        let base_routine_s19: Vec<u8> = match base_routine_family {
            BaseRoutineFamily::DSC_56F8006 => DSC_56F8006_FLASH_PROG.to_vec(),
            BaseRoutineFamily::DSC_56F8014 => DSC_56F8014_FLASH_PROG.to_vec(),
            BaseRoutineFamily::DSC_56F8023 => DSC_56F8023_FLASH_PROG.to_vec(),
            //BaseRoutineFamily::DSC_56F825X => DSC_56F825X_FLASH_PROG.to_vec(),
            //BaseRoutineFamily::DSC_56F8323 => DSC_56F8323_FLASH_PROG.to_vec(),
            //BaseRoutineFamily::DSC_FTFA    => DSC_FTFA_FLASH_PROG.to_vec(),
            //BaseRoutineFamily::DSC_FTFL    => DSC_FTFL_FLASH_PROG.to_vec(),
            BaseRoutineFamily::NONE        => return Ok(BaseRoutine::default()),
        };
        
        let parsed_data = ParsedData::parse_s19(base_routine_s19)?;
        if parsed_data.data_vec.len() != 1 { return Err(Error::InternalError("Base routine is fragmented or do not exist".to_string())) }
        let base_routine: Vec<u8> = parsed_data.data_vec[0].data_blob.clone();
        let image_address: usize = parsed_data.data_vec[0].address as usize;

        if base_routine.len() < 4 { return Err(Error::InternalError("Base routine length < 4".to_string())) }

        let header_offset: usize = 
            (((((base_routine[0] as u32) <<  0) |  //LITTLE ENDIAN
            ((   base_routine[1] as u32) <<  8) | 
            ((   base_routine[2] as u32) << 16) | 
            (    base_routine[3] as u32) << 24) as usize) - image_address) * 2;

        if base_routine.len() < (header_offset + 20) { return Err(Error::InternalError("Base routine length < (header_offset + 20)".to_string())) }

        let code_load_address: u32 = 
            (((base_routine[0 + header_offset] as u32) <<  0) |  //LITTLE ENDIAN
            (( base_routine[1 + header_offset] as u32) <<  8) | 
            (( base_routine[2 + header_offset] as u32) << 16) | 
            (( base_routine[3 + header_offset] as u32) << 24));

        if (code_load_address as usize != image_address) { return Err(Error::InternalError("Inconsistent actual and code load addresses".to_string())) }

        let code_entry: u32 =
            (((base_routine[4 + header_offset] as u32) <<  0) |  //LITTLE ENDIAN
            (( base_routine[5 + header_offset] as u32) <<  8) | 
            (( base_routine[6 + header_offset] as u32) << 16) | 
            (( base_routine[7 + header_offset] as u32) << 24));

        let capabilities: u16 =
            ((base_routine[8 + header_offset] as u16) <<  0) |   //LITTLE ENDIAN 
            ((base_routine[9 + header_offset] as u16) <<  8);

        let calib_frequency: u16 =
            ((base_routine[10 + header_offset] as u16) <<  0) |  //LITTLE ENDIAN 
            ((base_routine[11 + header_offset] as u16) <<  8);

        let calib_factor: u32 =
            ((base_routine[12 + header_offset] as u32) <<  0) |  //LITTLE ENDIAN
            ((base_routine[13 + header_offset] as u32) <<  8) | 
            ((base_routine[14 + header_offset] as u32) << 16) | 
            ((base_routine[15 + header_offset] as u32) << 24);

        let flash_data: u32 =
            (((base_routine[16 + header_offset] as u32) <<  0) | //LITTLE ENDIAN
            (( base_routine[17 + header_offset] as u32) <<  8) | 
            (( base_routine[18 + header_offset] as u32) << 16) | 
            (( base_routine[19 + header_offset] as u32) << 24));

        Ok(
            BaseRoutine {
            family: base_routine_family,
            routine: base_routine,
            code_load_address,
            code_entry,
            capabilities,
            calib_frequency,
            calib_factor,
            flash_data,
            }
        )
    }

}

#[cfg(test)]
#[allow(arithmetic_overflow)]
mod tests {
    use super::*;

    fn checksum(family: BaseRoutineFamily) -> u32 {
        let base_routine = BaseRoutine::new(family).unwrap();

        let mut sum: u32 = 0;
        for &x in base_routine.routine.iter() {
            sum += x as u32;
        }
        sum
    } 

    #[test]
    fn base_routine_checksum() {
        // checksum test
        assert_eq!(checksum(BaseRoutineFamily::DSC_56F8006), 124047);
        assert_eq!(checksum(BaseRoutineFamily::DSC_56F8014), 124494);
        assert_eq!(checksum(BaseRoutineFamily::DSC_56F8023), 124051);
        //assert_eq!(checksum(BaseRoutineFamily::DSC_56F825X), 120433);
        //assert_eq!(checksum(BaseRoutineFamily::DSC_56F8323), 131804);
        //assert_eq!(checksum(BaseRoutineFamily::DSC_FTFA),    157857);
        //assert_eq!(checksum(BaseRoutineFamily::DSC_FTFL),    161699);
        assert_eq!(checksum(BaseRoutineFamily::NONE),        0);

        // code_load_address check
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8006).unwrap().code_load_address, 0x008000);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8014).unwrap().code_load_address, 0x008000);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8023).unwrap().code_load_address, 0x008000);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F825X).unwrap().code_load_address, 0x008000);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8323).unwrap().code_load_address, 0x02F800);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFA   ).unwrap().code_load_address, 0x00F000);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFL   ).unwrap().code_load_address, 0x060000);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::NONE       ).unwrap().code_load_address,        0);

        // code_entry check
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8006).unwrap().code_entry, 0x00800C);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8014).unwrap().code_entry, 0x00800C);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8023).unwrap().code_entry, 0x00800C);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F825X).unwrap().code_entry, 0x00800C);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8323).unwrap().code_entry, 0x02F80C);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFA   ).unwrap().code_entry, 0x00F00C);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFL   ).unwrap().code_entry, 0x06000C);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::NONE       ).unwrap().code_entry,        0);

        // flash_data check
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8006).unwrap().flash_data, 0x000260);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8014).unwrap().flash_data, 0x000260);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8023).unwrap().flash_data, 0x000260);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F825X).unwrap().flash_data, 0x000252);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8323).unwrap().flash_data, 0x000028);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFA   ).unwrap().flash_data, 0x000298);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFL   ).unwrap().flash_data, 0x0002E0);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::NONE       ).unwrap().flash_data,        0);

        // calib_frequency check
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8006).unwrap().calib_frequency, 4000);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8014).unwrap().calib_frequency, 4000);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8023).unwrap().calib_frequency, 4000);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F825X).unwrap().calib_frequency, 4000);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8323).unwrap().calib_frequency, 4000);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFA   ).unwrap().calib_frequency, 4000);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFL   ).unwrap().calib_frequency, 4000);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::NONE       ).unwrap().calib_frequency,    0);

        // calib_factor
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8006).unwrap().calib_factor, 444039);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8014).unwrap().calib_factor, 444039);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8023).unwrap().calib_factor, 444039);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F825X).unwrap().calib_factor, 444039);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8323).unwrap().calib_factor, 508214);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFA   ).unwrap().calib_factor, 500553);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFL   ).unwrap().calib_factor, 500553);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::NONE       ).unwrap().calib_factor,      0);

        // capabilities check
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8006).unwrap().capabilities, 0b0000100000111110);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8014).unwrap().capabilities, 0b0000100000111110);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8023).unwrap().capabilities, 0b0000100000111110);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F825X).unwrap().capabilities, 0b0000100000111110);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_56F8323).unwrap().capabilities, 0b0000000000111110);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFA   ).unwrap().capabilities, 0b0000100000111110);
        //assert_eq!(BaseRoutine::new(BaseRoutineFamily::DSC_FTFL   ).unwrap().capabilities, 0b0000100000111110);
        assert_eq!(BaseRoutine::new(BaseRoutineFamily::NONE       ).unwrap().capabilities,                  0);
    }

}


