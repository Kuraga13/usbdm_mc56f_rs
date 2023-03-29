use crate::errors::Error;
use super::flash_routine::FlashRoutine;
use super::target_factory::{TargetInitActions, SecurityStatus};
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};


/// `MC56801X_SIM_ID` combine two bytes of JTAG ID (SIM_MSHID+SIM_LSHID), in mc56801x is  $01F2 401D
pub const MC56801X_SIM_ID : u32 =  0x01F2401D;

/// `MC56f801x` describes DSC targets family which include:
/// 
/// `MC56F8011/13`, 
/// 
///`MC56F8014`
/// 
/// 
#[derive(Debug, Clone)]
pub struct MC56f801x;

impl TargetInitActions for MC56f801x {


fn is_unsecure(&mut self, prog : &mut Programmer, jtag_id_code_vec : Vec<u8>, expected_id : u32) -> Result<SecurityStatus, Error> {

    let jtag_id_code =  vec_as_u32_be(jtag_id_code_vec);
    
    // println!("jtag_id_code : {:02X}", jtag_id_code);
    // println!("expected_id : {:02X}", expected_id);
   
     match jtag_id_code {
           expected_id                => Ok(SecurityStatus::Unsecured),
           0x0                        => Ok(SecurityStatus::Secured),           
           _                          => Ok(SecurityStatus::Unknown),             
       }    


 } 

/// Mass Erase specific on Dsc Target Family mass erase algorith
fn mass_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error> {

    unimplemented!();

 }

/// Calculate specific on Dsc Target Family cfmclkd
fn calculate_flash_divider(&mut self, power : TargetVddSelect, prog : &mut Programmer, bus_frequency : u32 ) -> Result<u32, Error> {

    FlashRoutine::calculate_flash_divider(bus_frequency)

 }

/// Init specific on Dsc Target Family algorith
fn target_init(&mut self, power : TargetVddSelect, prog : &mut Programmer ) {

    unimplemented!();

 }
}


/// `MC56802X_SIM_ID` combine two bytes of JTAG ID (SIM_MSHID+SIM_LSHID), in mc568023-35 is  $01F2801D
pub const MC56802X_SIM_ID : u32 =  0x01F2801D;

/// `MC56f802x` describes DSC targets family which include:
/// 
///`MC56F8023/33`
/// 
///`MC56F8025/35`
/// 
///`MC56F8036`
/// 
///`MC56F8027/37`
#[derive(Debug, Clone)]
pub struct MC56f802x;

impl TargetInitActions for MC56f802x {


fn is_unsecure(&mut self, prog : &mut Programmer, jtag_id_code_vec : Vec<u8>, expected_id : u32) -> Result<SecurityStatus, Error> {

    let jtag_id_code =  vec_as_u32_be(jtag_id_code_vec);
    
    // println!("jtag_id_code : {:02X}", jtag_id_code);
    // println!("expected_id : {:02X}", expected_id);
   

     match jtag_id_code {
           expected_id           => Ok(SecurityStatus::Unsecured),
           0x0                        => Ok(SecurityStatus::Secured),           
           _                          => Ok(SecurityStatus::Unknown),             
       }    


 } 

/// Mass Erase specific on Dsc Target Family mass erase algorith
fn mass_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error> {

    unimplemented!();

 }

/// Calculate specific on Dsc Target Family cfmclkd
fn calculate_flash_divider(&mut self, power : TargetVddSelect, prog : &mut Programmer, bus_frequency : u32) -> Result<u32, Error> {

    unimplemented!();

 }

/// Init specific on Dsc Target Family algorith
fn target_init(&mut self, power : TargetVddSelect, prog : &mut Programmer ) {

    unimplemented!();

 }
}




pub fn vec_as_u32_be(vec:  Vec<u8>) -> u32 {
    ((vec[0] as u32) << 24) +
    ((vec[1] as u32) << 16) +
    ((vec[2] as u32) <<  8) +
    ((vec[3] as u32) <<  0)
}


pub fn print_id_code(core_id_code : &Vec<u8>, master_id_code : &Vec<u8>) {
    
    println!(" core_id_code :");
     
    for byte in core_id_code.iter()
    {
    let in_string = format!("{:02X}", byte);
    print!("{} ", in_string);
    }
   
    println!(" \n");
   
    println!(" master_id_code (in usbdm jtag-idcode) :");
    for byte in master_id_code.iter()
    {
   
    let in_string = format!("{:02X}", byte);
    print!("{} ", in_string);
   
    }
   
    println!(" \n");
      
   }

///'print_vec_memory' for debug memory read, sequnces etc., use for print small block in Vec<u8>
fn print_vec_memory(mem : Vec<u8>) {
   
   let mut printed_vec = Vec::new();
    for byte in mem.iter() {
    let in_string = format!("{:02X}", byte);
    printed_vec.push(in_string);
     if(printed_vec.len() == 0x10) {

      for symbol in printed_vec.iter() {
       print!("{} ", symbol); }
    print!("\n");
    printed_vec.clear();
    }  
  } 
}