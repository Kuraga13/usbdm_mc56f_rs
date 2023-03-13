use packed_struct::prelude::*;
use crate::errors::Error;

use serde::{Serialize, Deserialize};
use std::io::Read;
use std::path::Path;
use crate::file_buffer::data_parser::ParsedData;

/*

  this is a draft, it's in progress


 */


/// `RoutineCapabilites` u16 byte, stored in compiled elf file for concrete target! 
/// 
/// We need parse and match to check what we are asking from the routine that it has the ability
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[packed_struct(bit_numbering="lsb0",size_bytes="2",endian="lsb",)]
pub struct RoutineCapabilites {

    #[packed_field(bits="1")]
    cap_erase_block           : bool,     // Erase entire flash block e.g. Flash, FlexNVM etc                    
    #[packed_field(bits="2")]
    cap_erase_range           : bool,     // Erase range (including option region) 
    #[packed_field(bits="3")]
    cap_blanck_check_range    : bool,     // Blank check region                             
    #[packed_field(bits="4")]
    cap_programm_range        : bool,     // Program range (including option region)  
    #[packed_field(bits="5")]
    cap_verify_range          : bool,     // Verify range                
    #[packed_field(bits="7")]
    cap_partion_data_programm : bool,     // Program FlexNVM DFLASH/EEPROM partitioning    
    #[packed_field(bits="8")]
    cap_timing                : bool,     // Counting loop to determine clock speed
    #[packed_field(bits="11")]
    cap_dsc_overlay           : bool,     // Indicates DSC code in pMEM overlays xRAM
    #[packed_field(bits="12")]
    cap_data_fixed            : bool,     // Indicates TargetFlashDataHeader is at fixed address
    #[packed_field(bits="15")]
    cap_relocatable           : bool,     // Routine may be relocated  
}

impl Default for RoutineCapabilites {
    fn default() -> Self { 

        RoutineCapabilites {
            cap_erase_block           : false,
            cap_erase_range           : false,
            cap_blanck_check_range    : false,
            cap_programm_range        : false,
            cap_verify_range          : false,
            cap_partion_data_programm : false,
            cap_timing                : false,
            cap_dsc_overlay           : false,
            cap_data_fixed            : false,
            cap_relocatable           : false,
      }
    } 
 }

impl RoutineCapabilites  {

    fn parse_capabilities_from_elf() -> Self { 

        unimplemented!();

        RoutineCapabilites {
            cap_erase_block           : false,
            cap_erase_range           : false,
            cap_blanck_check_range    : false,
            cap_programm_range        : false,
            cap_verify_range          : false,
            cap_partion_data_programm : false,
            cap_timing                : false,
            cap_dsc_overlay           : false,
            cap_data_fixed            : false,
            cap_relocatable           : false,
      }
    } 
 }


 


/// `RoutineTaskByte` u16 byte, descripe command operation to Routine, packed in header, orig `flags` in `LargeTargetFlashDataHeader`
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[packed_struct(bit_numbering="lsb0",size_bytes="2",endian="lsb",)]
pub struct RoutineTaskByte {
   
    #[packed_field(bits="0")]
    init_flash             : bool,          // Do initialisation of flash                   
    #[packed_field(bits="1")]
    erase_block            : bool,          // Erase entire flash block e.g. Flash, FlexNVM etc         
    #[packed_field(bits="2")]
    erase_range            : bool,          // Erase range (including option region)                          
    #[packed_field(bits="3")]
    blank_check_range      : bool,          // Blank check region    
    #[packed_field(bits="4")]
    program_range          : bool,          // Program range (including option region)          
    #[packed_field(bits="5")]
    verify_range           : bool,          // Verify range     
    #[packed_field(bits="6")]
    partion_data_programm  : bool,          // Program FlexNVM DFLASH/EEPROM partitioning        
    #[packed_field(bits="7")]
    timing_loop            : bool,          // Counting loop to determine clock speed
    #[packed_field(bits="15")]
    is_complete            : bool,          // set completion flag, routine must clear it
}

impl Default for RoutineTaskByte {
    fn default() -> Self { 

        unimplemented!();

        RoutineTaskByte {
            init_flash            : false,
            erase_block           : false,
            erase_range           : false,
            blank_check_range     : false,
            program_range         : false,
            verify_range          : false,
            partion_data_programm : false,
            timing_loop           : false,
            is_complete           : false,
      }
    }



 }

 impl RoutineTaskByte {

    fn new_task(flash_code : Vec<u8>) -> Self { 

        RoutineTaskByte {
            init_flash            : false,
            erase_block           : false,
            erase_range           : false,
            blank_check_range     : false,
            program_range         : false,
            verify_range          : false,
            partion_data_programm : false,
            timing_loop           : false,
            is_complete           : false,
      }
    }

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


/// `RoutineTask` 
/// 
/// this struct represent task with parameters we give to routine
/// 
/// it is stored in the header of the uploaded routine
/// 
/// orig name `LargeTargetFlashDataHeader` - Header at the start of flash programming buffer (controls program action)
#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, Serialize, Deserialize)]
pub struct RoutineTask
{
   /// `task_byte`  - coded in byte task for Routine, struct `RoutineTaskByte`
   /// 
   /// in orig = defines DO_PROGRAM_RANGE or DO_BLANK_CHECK_RANGE|DO_PROGRAM_RANGE|DO_VERIFY_RANGE etc.
   /// 
   /// `task_byte`  - struct `RoutineTaskByte` in u16 orig flags
   pub task_byte               : RoutineTaskByte,

   // from orig `error_code`  -  not sure we need this, looks like another regular boilerplate from origin
   //pub error_code            : u16,
   
   /// from orig `controller`  -  Ptr to flash controller. Controller is struct hold dsc core registers for routine
   pub controller             : u16,
   
   /// `calibFrequency` in u16, Target frequency (kHz)
   pub frequency              : u16,

   ///`minimal_sector` from orig `sector_size` - Size of Flash memory sectors (smallest erasable block)
   pub minimal_sector         : u16,

   /// `start_address` start address for routine programming. orig `address`:  Memory address being accessed (reserved/page/address)
   pub start_address          : u32,

   /// `range_size` range for routine programming orig `dataSize`
   pub range_size             : u16, 
   
   // from orig `pad`  -  not sure we need this, it's either a stub or a reserve
   //pub pad                    : u16,

   /// `buffer_address` Ptr to data to program orig `dataAddress`
   pub buffer_address         : u32, 

   /// ` timing_count `for speed determine function
   pub timing_count           : u32,   // from LargeTargetTimingDataHeader(I think the rest is redundant), 
   
}


impl RoutineTask {


fn build_routine_task(raw_elf : Vec<u8>) -> Self {

        unimplemented!(); // !! For example

        RoutineTask {

            task_byte          : RoutineTaskByte::new_task(raw_elf),
            controller         : u16::from(raw_elf[1]),
            frequency          : u16::from(raw_elf[2]), 
            minimal_sector     : u16::from(raw_elf[3]), 
            start_address      : u32::from(raw_elf[4]), 
            range_size         : u16::from(raw_elf[5]), 
            buffer_address     : u32::from(raw_elf[6]), 
            timing_count       : u32::from(raw_elf[7]), 
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


pub fn build_base_routine(base_routine_path : String) -> Result<Self, Error> {
   
           unimplemented!(); // !! For example !!!
           //this is a draft, it's in progress

           let p =Path::new(&base_routine_path);
           let s19_file = std::fs::File::open(p)?;

           let mut buffer_vec = Vec::new();
           s19_file.read_to_end(&mut buffer_vec)?;
           let parsed_data = ParsedData::parse_s19(buffer_vec)?;
           let bin_routine = parsed_data.to_bin()?;

     
      
   
           Ok(FlashRoutine {
   
               name                 : Some("Some Mcu name from yaml".to_string()),
               load_address         : 0, // set in YAML directly OR parse base load address from elf_bin 
               entry_address        : 0, //set in YAML directly OR  parse from bin 
               capabilities         : RoutineCapabilites::parse_capabilities_from_elf(), // in param need binary elf OR directly from YAML
               calib_frequency      : 0, //set in YAML directly OR  parse from bin 
               calib_factor         : 0, //set in YAML directly OR  parse from bin 
               base_routine         : bin_routine, // here binary from elf_bin
               routine_task         : RoutineTask::build_routine_task(bin_routine), // 
               address_routine_task : 0, //set in YAML directly OR  parse from bin 
               execution_result     : None, // 
    })
}




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









