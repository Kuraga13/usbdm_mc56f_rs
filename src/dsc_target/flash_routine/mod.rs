pub mod base_routine;
mod flash_data_header;
mod flash_routine;

use crate::usbdm::Programmer;
use crate::usbdm::constants::{memory_space_t};
use crate::errors::Error;
use crate::dsc_target::target_factory::DscFamily;
use serde::{Deserialize, Serialize};

use base_routine::BaseRoutine;
use flash_data_header::*;
use std::{thread, time};
pub use flash_routine::FlashRoutine;

/*
use packed_struct::prelude::*;
use crate::errors::Error;

use serde::{Serialize, Deserialize};
use std::io::Read;
use std::path::Path;
use std::fs;
use crate::file_buffer::data_parser::ParsedData;


 impl RoutineTaskByte {



    fn check_task_ability(ability : RoutineCapabilites)
    {
        unimplemented!();
    }

    fn programm_task_byte() -> Self
    {
        unimplemented!();
        RoutineTaskByte {
            init_flash            : true,
            erase_block           : true,
            erase_range           : true,
            blank_check_range     : true,
            program_range         : true,
            verify_range          : true,

            partion_data_programm : false,
            timing_loop           : false,
            is_complete           : false,
      }
    }
 }



/// `RoutineResult` from orig `ResultStruct` - routine save in target memory result of self work -  error codes, flag, etc.
#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, Serialize, Deserialize)]
pub struct RoutineResult
{
    pub flags             : u16,   // Controls actions of routine
    pub error_code        : u16,   // Error code from action

}

impl RoutineResult
{
    fn parse_routine_result(read_from_dsc : Vec<u8>) -> Self { 

        unimplemented!();

        RoutineResult {
            flags            : 0, //something like u16::from(read_from_dsc[0] + read_from_dsc[1]),
            error_code       : 0, // something like u16::from(read_from_dsc[2] + read_from_dsc[3],)
      }
    }

    fn check_result(&self) -> Result<(), Error> {

        unimplemented!();

    }
}





impl RoutineTask {


fn build_routine_task(raw_elf : Vec<u8>) -> Self {


        RoutineTask {

            controller         : u16::from(raw_elf[1]),
            frequency          : u16::from(raw_elf[2]), 
            minimal_sector     : u16::from(raw_elf[3]), 
            start_address      : u32::from(raw_elf[4]), 
            range_size         : u16::from(raw_elf[5]), 
            buffer_address     : u32::from(raw_elf[6]), 
            timing_count       : u32::from(raw_elf[7]), 
            task_byte          : RoutineTaskByte::new_task(raw_elf),
      }
   }


fn build_speed_measurment_task(raw_elf : Vec<u8>) -> Self {

    unimplemented!(); // !! For example

    let mut task_byte= RoutineTaskByte::default();

    task_byte.timing_loop = true;
    task_byte.is_complete = true;

    RoutineTask {

        task_byte          : task_byte,
        controller         : u16::from(raw_elf[1]),
        frequency          : u16::from(raw_elf[2]), 
        minimal_sector     : u16::from(raw_elf[3]), 
        start_address      : u32::from(raw_elf[4]), 
        range_size         : u16::from(raw_elf[5]), 
        buffer_address     : u32::from(raw_elf[6]), 
        timing_count       : u32::from(raw_elf[7]), 
    }
 }



}
    


#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, Serialize, Deserialize)]
pub struct FlashRoutine {

   /// A name to describe FlashRoutine
   pub name                 : Option<String>,

   /// Memory addres where to load all this shit.  orig `loadAddress`
   pub load_address         : u32,

   ///Execution start address, entry of routine (for currently loaded routine) orig `entry`
   pub entry_address        : u32,
   
   ///Stored in elf file, we need parse this shit, and check we have the necessary capabilities, orig `capabilities`
   pub capabilities         : RoutineCapabilites, 

   /// `TODO` 
   ///combine `calibFrequency` and `calib` into `speed calibration` structure, make normal methods
   /// 
   /// Frequency (kHz) used for calibFactor 
   pub calib_frequency      : u16, 

   ///`calib_factor` from orig `Calibration factor` for speed determination
   pub calib_factor         : u32,  

   ///`base_routine` - base code compiled for concrete dsc target, from elf file
   pub base_routine         : Vec<u8>,

   /// `routine_task`  - struct `RoutineTask`, describe task and all data needed for task
   pub routine_task         : RoutineTask,

   // orig `flashData` or `dataHeaderAddress` Ptr to routine_task struct, so routine can use data
   pub address_routine_task : u32, 

   /// from orig ResultStruct `execution_result`  - routine save in target memory result of self work -  error codes, flag, etc.
   pub execution_result     : Option<RoutineResult>,
   
}


impl FlashRoutine {


pub fn build_speed_meter_routine(&mut self,  target_yaml : &str) -> Self {

    unimplemented!(); // !! For example !!!
    //this is a draft, it's in progress

    let serialazed          = serde_yaml::to_string(&target_yaml).unwrap();
    let deserialized :  Vec<u8>     = serde_yaml::from_str(&target_yaml).unwrap();
    //TODO - bind serde_yaml with FlashRoutine, serialize/deserialize 

  

    let elf_bin = std::fs::File::open("bin_file_path_from_yaml.yaml");

    let routine_from_elf = vec![0;0xff];

    FlashRoutine {

        name                 : Some("Some Mcu name from yaml".to_string()),
        load_address         : 0, // set in YAML directly OR parse base load address from elf_bin 
        entry_address        : 0, //set in YAML directly OR  parse from bin 
        capabilities         : RoutineCapabilites::parse_capabilities_from_elf(), // in param need binary elf OR directly from YAML
        calib_frequency      : 0, //set in YAML directly OR  parse from bin 
        calib_factor         : 0, //set in YAML directly OR  parse from bin 
        base_routine         : routine_from_elf, // here binary from elf_bin
        routine_task         : RoutineTask::build_speed_measurment_task(routine_from_elf), // 
        address_routine_task : 0, //set in YAML directly OR  parse from bin 
        execution_result     : None, // 
     }
   }


pub fn build_write_block_routine(&mut self,  target_yaml : &str) -> Self {

    unimplemented!(); // !! For example !!!
    //this is a draft, it's in progress

    let serialazed          = serde_yaml::to_string(&target_yaml).unwrap();
    let deserialized :  Vec<u8>     = serde_yaml::from_str(&target_yaml).unwrap();
    //TODO - bind serde_yaml with FlashRoutine, serialize/deserialize 

    let elf_bin = std::fs::File::open("bin_file_path_from_yaml.yaml");

    let routine_from_elf = vec![0;0xff];

    FlashRoutine {

        name                 : Some("Some Mcu name from yaml".to_string()),
        load_address         : 0, // set in YAML directly OR parse base load address from elf_bin 
        entry_address        : 0, //set in YAML directly OR  parse from bin 
        capabilities         : RoutineCapabilites::parse_capabilities_from_elf(), // in param need binary elf OR directly from YAML
        calib_frequency      : 0, //set in YAML directly OR  parse from bin 
        calib_factor         : 0, //set in YAML directly OR  parse from bin 
        base_routine         : routine_from_elf, // here binary from elf_bin
        routine_task         : RoutineTask::build_speed_measurment_task(routine_from_elf), // 
        address_routine_task : 0, //set in YAML directly OR  parse from bin 
        execution_result     : None, // 
     }
   }


pub fn build_blanck_check_routine(&mut self,  target_yaml : &str) -> Self {

    unimplemented!(); // !! For example !!!
    //this is a draft, it's in progress

    let serialazed          = serde_yaml::to_string(&target_yaml).unwrap();
    let deserialized :  Vec<u8>     = serde_yaml::from_str(&target_yaml).unwrap();
    //TODO - bind serde_yaml with FlashRoutine, serialize/deserialize 

    let elf_bin = std::fs::File::open("bin_file_path_from_yaml.yaml");

    let routine_from_elf = vec![0;0xff];

    FlashRoutine {

        name                 : Some("Some Mcu name from yaml".to_string()),
        load_address         : 0, // set in YAML directly OR parse base load address from elf_bin 
        entry_address        : 0, //set in YAML directly OR  parse from bin 
        capabilities         : RoutineCapabilites::parse_capabilities_from_elf(), // in param need binary elf OR directly from YAML
        calib_frequency      : 0, //set in YAML directly OR  parse from bin 
        calib_factor         : 0, //set in YAML directly OR  parse from bin 
        base_routine         : routine_from_elf, // here binary from elf_bin
        routine_task         : RoutineTask::build_speed_measurment_task(routine_from_elf), // 
        address_routine_task : 0, //set in YAML directly OR  parse from bin 
        execution_result     : None, // 
     }
   }

pub fn build_erase_routine(&mut self,  target_yaml : &str) -> Self {

    unimplemented!(); // !! For example !!!
    //this is a draft, it's in progress

    let serialazed          = serde_yaml::to_string(&target_yaml).unwrap();
    let deserialized :  Vec<u8>     = serde_yaml::from_str(&target_yaml).unwrap();
    //TODO - bind serde_yaml with FlashRoutine, serialize/deserialize 

    let elf_bin = std::fs::File::open("bin_file_path_from_yaml.yaml");

    let routine_from_elf = vec![0;0xff];

    FlashRoutine {

        name                 : Some("Some Mcu name from yaml".to_string()),
        load_address         : 0, // set in YAML directly OR parse base load address from elf_bin 
        entry_address        : 0, //set in YAML directly OR  parse from bin 
        capabilities         : RoutineCapabilites::parse_capabilities_from_elf(), // in param need binary elf OR directly from YAML
        calib_frequency      : 0, //set in YAML directly OR  parse from bin 
        calib_factor         : 0, //set in YAML directly OR  parse from bin 
        base_routine         : routine_from_elf, // here binary from elf_bin
        routine_task         : RoutineTask::build_speed_measurment_task(routine_from_elf), // 
        address_routine_task : 0, //set in YAML directly OR  parse from bin 
        execution_result     : None, // 
     }
   }

pub fn build_verify_routine(&mut self,  target_yaml : &str) -> Self {

    unimplemented!(); // !! For example !!!
    //this is a draft, it's in progress

    let serialazed          = serde_yaml::to_string(&target_yaml).unwrap();
    let deserialized :  Vec<u8>     = serde_yaml::from_str(&target_yaml).unwrap();
    //TODO - bind serde_yaml with FlashRoutine, serialize/deserialize 

    let elf_bin = std::fs::File::open("bin_file_path_from_yaml.yaml");

    let routine_from_elf = vec![0;0xff];

    FlashRoutine {

        name                 : Some("Some Mcu name from yaml".to_string()),
        load_address         : 0, // set in YAML directly OR parse base load address from elf_bin 
        entry_address        : 0, //set in YAML directly OR  parse from bin 
        capabilities         : RoutineCapabilites::parse_capabilities_from_elf(), // in param need binary elf OR directly from YAML
        calib_frequency      : 0, //set in YAML directly OR  parse from bin 
        calib_factor         : 0, //set in YAML directly OR  parse from bin 
        base_routine         : routine_from_elf, // here binary from elf_bin
        routine_task         : RoutineTask::build_speed_measurment_task(routine_from_elf), // 
        address_routine_task : 0, //set in YAML directly OR  parse from bin 
        execution_result     : None, // 
     }
   }

   pub fn adapt_routine_verify(&mut self,  target_yaml : &str) -> Vec<u8> {

    unimplemented!(); // !! For example !!!
    //this is a draft, it's in progress
    let deserialized :  String     = serde_yaml::from_str(&target_yaml).unwrap();

    self.name = Some("Verify routine".to_string() + &deserialized); 
    self.routine_task = RoutineTask::default();
    self.routine_task.task_byte.verify_range = true;

   }
}

*/







