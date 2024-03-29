#![allow(non_camel_case_types)]

use super::*;
use crate::file_buffer::data_parser::ParsedData;

const DSC_56F800X_FLASH_PROG: &[u8] = include_bytes!(r"base_routine\DSC-56F800x-base_routine.S");
const DSC_56F801X_FLASH_PROG: &[u8] = include_bytes!(r"base_routine\DSC-56F801x-base_routine.S");
const DSC_56F802X_FLASH_PROG: &[u8] = include_bytes!(r"base_routine\DSC-56F802x-base_routine.S");


// Capability masks
const CAP_ERASE_BLOCK       : u16 = 1<<1;
const CAP_ERASE_RANGE       : u16 = 1<<2;
const CAP_BLANK_CHECK_RANGE : u16 = 1<<3;
const CAP_PROGRAM_RANGE     : u16 = 1<<4;
const CAP_VERIFY_RANGE      : u16 = 1<<5;
const CAP_PARTITION_FLEXNVM : u16 = 1<<7;
const CAP_DSC_OVERLAY       : u16 = 1<<11; // Indicates DSC code in pMEM overlays xRAM
const CAP_DATA_FIXED        : u16 = 1<<12; // Indicates TargetFlashDataHeader is at fixed address
const CAP_RELOCATABLE       : u16 = 1<<15; // Code may be relocated

/// Contains all information about base flash routine
#[derive(Debug)]
pub struct BaseRoutine {
    /// Describes family of processors this flash routine is for
    pub dsc_family: DscFamily,
    /// Vec<u8> containing flash routine
    pub routine: Vec<u8>,
    /// Address where this routine is loaded to (in 2 byte words)
    pub address: u32,
    /// Memory Space where this routine is loaded to
    pub address_memspace: u8,
    /// Address for execution start (in 2 byte words)
    pub code_entry: u32,
    /// Capabilities of routine
    pub capabilities: u16,
    /// Frequency (kHz) used for calibFactor
    pub calib_frequency: u16,
    /// Calibration factor for speed determination
    pub calib_factor: u32,
    /// Address where to load flash data for execution (in 2 byte words)
    pub data_header_address: u32,
    /// Memory Space where to load data_header
    pub data_header_address_memspace: u8,
}

impl BaseRoutine
{
    pub fn get(dsc_family : DscFamily) -> Result<Self, Error> {
        let base_routine_s19: Vec<u8> = match dsc_family {
            DscFamily::Mc56f800X => DSC_56F800X_FLASH_PROG.to_vec(),
            DscFamily::Mc56f801X => DSC_56F801X_FLASH_PROG.to_vec(),
            DscFamily::Mc56f802X => DSC_56F802X_FLASH_PROG.to_vec(),
            DscFamily::Mc56f803X => DSC_56F802X_FLASH_PROG.to_vec(), // DSC_56F803X use the same routine as DSC_56F802X
            _                    => return Err(Error::InternalError("Unknown DscFamily".to_string())),
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

        let address: u32 = 
            (((base_routine[0 + header_offset] as u32) <<  0) |  //LITTLE ENDIAN
            (( base_routine[1 + header_offset] as u32) <<  8) | 
            (( base_routine[2 + header_offset] as u32) << 16) | 
            (( base_routine[3 + header_offset] as u32) << 24));

        if (address as usize != image_address) { return Err(Error::InternalError("Inconsistent actual and code load addresses".to_string())) }

        let address_memspace: u8 = memory_space_t::MS_PWORD;

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

        let data_header_address: u32 =
            (((base_routine[16 + header_offset] as u32) <<  0) | //LITTLE ENDIAN
            (( base_routine[17 + header_offset] as u32) <<  8) | 
            (( base_routine[18 + header_offset] as u32) << 16) | 
            (( base_routine[19 + header_offset] as u32) << 24));

        let data_header_address_memspace: u8 = memory_space_t::MS_XWORD;

        Ok(
            BaseRoutine {
            dsc_family,
            routine: base_routine,
            address,
            address_memspace,
            code_entry,
            capabilities,
            calib_frequency,
            calib_factor,
            data_header_address,
            data_header_address_memspace,
            }
        )
    }

}

#[cfg(test)]
#[allow(arithmetic_overflow)]
mod tests {
    use super::*;

    fn checksum(family: DscFamily) -> u32 {
        let base_routine = BaseRoutine::get(family).unwrap();

        let mut sum: u32 = 0;
        for &x in base_routine.routine.iter() {
            sum += x as u32;
        }
        sum
    } 

    #[test]
    fn base_routine_checksum() {
        // checksum test
        assert_eq!(checksum(DscFamily::Mc56f800X), 124047);
        assert_eq!(checksum(DscFamily::Mc56f801X), 124494);
        assert_eq!(checksum(DscFamily::Mc56f802X), 124051);
        assert_eq!(checksum(DscFamily::Mc56f803X), 124051);

        // code_load_address_memspace check
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f800X).unwrap().address_memspace, memory_space_t::MS_PWORD);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f801X).unwrap().address_memspace, memory_space_t::MS_PWORD);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f802X).unwrap().address_memspace, memory_space_t::MS_PWORD);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f803X).unwrap().address_memspace, memory_space_t::MS_PWORD);

        // code_load_address check
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f800X).unwrap().address, 0x008000);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f801X).unwrap().address, 0x008000);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f802X).unwrap().address, 0x008000);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f803X).unwrap().address, 0x008000);

        // code_entry check
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f800X).unwrap().code_entry, 0x00800C);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f801X).unwrap().code_entry, 0x00800C);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f802X).unwrap().code_entry, 0x00800C);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f803X).unwrap().code_entry, 0x00800C);

        // data_header_address check
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f800X).unwrap().data_header_address, 0x000260);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f801X).unwrap().data_header_address, 0x000260);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f802X).unwrap().data_header_address, 0x000260);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f803X).unwrap().data_header_address, 0x000260);

        // data_header_address_memspace check
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f800X).unwrap().data_header_address_memspace, memory_space_t::MS_XWORD);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f801X).unwrap().data_header_address_memspace, memory_space_t::MS_XWORD);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f802X).unwrap().data_header_address_memspace, memory_space_t::MS_XWORD);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f803X).unwrap().data_header_address_memspace, memory_space_t::MS_XWORD);

        // calib_frequency check
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f800X).unwrap().calib_frequency, 4000);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f801X).unwrap().calib_frequency, 4000);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f802X).unwrap().calib_frequency, 4000);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f803X).unwrap().calib_frequency, 4000);

        // calib_factor
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f800X).unwrap().calib_factor, 444039);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f801X).unwrap().calib_factor, 444039);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f802X).unwrap().calib_factor, 444039);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f803X).unwrap().calib_factor, 444039);

        // capabilities check
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f800X).unwrap().capabilities, 0b0000100000111110);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f801X).unwrap().capabilities, 0b0000100000111110);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f802X).unwrap().capabilities, 0b0000100000111110);
        assert_eq!(BaseRoutine::get(DscFamily::Mc56f803X).unwrap().capabilities, 0b0000100000111110);
    }

}


