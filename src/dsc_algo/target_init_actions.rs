use crate::errors::Error;
use super::target_factory::{TargetInitActions};
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

/// is Unsecure - check Target unsecured, get Secure Status
fn is_unsecure(&mut self, prog : &mut Programmer) -> Result<(), Error> {

    unimplemented!();

 } 

/// Mass Erase specific on Dsc Target Family mass erase algorith
fn mass_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error> {

    unimplemented!();

 }

/// Calculate specific on Dsc Target Family cfmclkd
fn calculate_flash_divider(&mut self, power : TargetVddSelect, prog : &mut Programmer ) -> Result<(), Error> {

    unimplemented!();

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

/// is Unsecure - check Target unsecured, get Secure Status
fn is_unsecure(&mut self, prog : &mut Programmer) -> Result<(), Error> {

    unimplemented!();

 } 

/// Mass Erase specific on Dsc Target Family mass erase algorith
fn mass_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error> {

    unimplemented!();

 }

/// Calculate specific on Dsc Target Family cfmclkd
fn calculate_flash_divider(&mut self, power : TargetVddSelect, prog : &mut Programmer ) -> Result<(), Error> {

    unimplemented!();

 }

/// Init specific on Dsc Target Family algorith
fn target_init(&mut self, power : TargetVddSelect, prog : &mut Programmer ) {

    unimplemented!();

 }
}

