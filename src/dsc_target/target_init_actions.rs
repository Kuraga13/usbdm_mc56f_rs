use crate::errors::Error;
use crate::utils::*;
use super::target_factory::{TargetInitActions, SecurityStatus, FlashModuleStatus, TargetDsc, DscFamily};
use crate::usbdm::jtag::*;
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::constants::{memory_space_t, bdm_commands, jtag_shift};

use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};
use std::{thread, time};
use std::time::Duration;


/// `MC56800X_SIM_ID` combine two bytes of JTAG ID (SIM_MSHID+SIM_LSHID), in mc56801x is  $01F2 401D
pub const MC56800X_SIM_ID : u32 =  0x01F2601D;
pub const MC56800X_SIM_MSH_ID_ADD : u32 =  0xF242;
pub const MC56800X_SIM_LSH_ID_ADD : u32 =  0xF243;

/// `MC56801X_SIM_ID` combine two bytes of JTAG ID (SIM_MSHID+SIM_LSHID), in mc56801x is  $01F2 401D
pub const MC56801X_SIM_ID : u32 =  0x01F2401D;
pub const MC56801X_SIM_MSH_ID_ADD : u32 =  0xF146;
pub const MC56801X_SIM_LSH_ID_ADD : u32 =  0xF147;

/// `MC56802X_SIM_ID` combine two bytes of JTAG ID (SIM_MSHID+SIM_LSHID), in mc568023-35 is  $01F2801D
pub const MC56802X_SIM_ID : u32 =  0x01F2801D;
pub const MC56803X_SIM_ID : u32 =  0x01F2801D;
pub const MC568023X_SIM_MSH_ID_ADD : u32 =  0xF106;
pub const MC568023X_SIM_LSH_ID_ADD : u32 =  0xF107;

///`MC56f80XX_FLASH_MODULE_CLKDIV` Clock Divider (CLKDIV) Register
pub const MC56F80XX_FLASH_MODULE_CLKDIV : u32 =  0xF400;
///`MC56f80XX_FLASH_MODULE_PROT` Protection (PROT) Register 
pub const MC56F80XX_FLASH_MODULE_PROT: u32 =  0xF410;
///`MC56F80XX_FLASH_MODULE_USTAT` User Status (USTAT) Register 
pub const MC56F80XX_FLASH_MODULE_USTAT: u32 =  0xF413;


///`MC56f80xx` describes DSC targets family which include:
/// 
///`MC56F8002/06`,
/// 
///`MC56F8011/13`, 
/// 
///`MC56F8014`
/// 
///`MC56F8023/33`
/// 
///`MC56F8025/35`
/// 
///`MC56F8036`
/// 
///`MC56F8027/37`
#[derive(Debug, Clone)]
pub struct MC56f80xx {

   jtag_id_code : u32,
   msh_id_addr  : u32,
   lsh_id_addr  : u32,
   core_id      : u32,
   family       : DscFamily,

}

impl MC56f80xx {

pub fn new(jtag_id_code : u32, core_id : u32, family : DscFamily) -> Result<Self, Error> {

   let (msh_id, lsh_id) =
    match family {
      DscFamily::Mc56f800X      => (MC56800X_SIM_MSH_ID_ADD, MC56800X_SIM_LSH_ID_ADD),
      DscFamily::Mc56f801X      => (MC56801X_SIM_MSH_ID_ADD, MC56801X_SIM_LSH_ID_ADD),
      DscFamily::Mc56f802X      => (MC568023X_SIM_MSH_ID_ADD, MC568023X_SIM_LSH_ID_ADD),
      DscFamily::Mc56f803X      => (MC568023X_SIM_MSH_ID_ADD, MC568023X_SIM_LSH_ID_ADD),
      _                         => return Err(Error::InternalError("msh_lsh id from family parse Failed".to_string()))};
 
   Ok( Self {
      jtag_id_code,
      msh_id_addr : msh_id,
      lsh_id_addr : lsh_id,
      core_id,
      family, })  
}


/// Calculate specific on MC56f80xx Family cfmclkd
fn calculate_flash_divider(&self, bus_frequency : u32) -> Result<u32, Error> {

        const DSC_PRDIV8 : u32 = 0x40;
    
        if (bus_frequency < 1000) {
           println! ("Clock too low for flash programming");
           return Err(Error::InternalError(("PROGRAMMING_RC_ERROR_NO_VALID_FCDIV_VALUE".to_string()))); };
     
        let osc_frequency = 2 * bus_frequency;
        let mut in_frequency;
        let mut cfmclkd : u32;
     
        if (osc_frequency > 12800) {
           cfmclkd = DSC_PRDIV8;
           in_frequency = osc_frequency / 8;
        } else {
           cfmclkd = 0;
           in_frequency = osc_frequency;
        }
     
        let min_period = 1.0 / 200.0 + 1.0 / (4.0 * bus_frequency as f64);
   
        let mut calculation = in_frequency as f64 * min_period;
        cfmclkd += calculation.round() as u32;
        
        let flash_clk = in_frequency / ((cfmclkd & 0x3F) + 1);
    
        println!("inFrequency {}, kHz cfmclkd = 0x {}, flashClk = {}, kHz, ", in_frequency, cfmclkd, flash_clk);
     
        if (flash_clk < 150) {
            println! ("Not possible to find suitable flash clock divider");
            return Err(Error::InternalError(("PROGRAMMING_RC_ERROR_NO_VALID_FCDIV_VALUE".to_string()))); }
     
         Ok(cfmclkd)
    
     }

/// `init_flash_divider`, CLKDIV = register in FlashModule, need for flash programming, on MC56f80xx Family cfmclkd
fn init_flash_divider(&mut self, power : TargetVddSelect, prog : &mut Programmer, bus_freq : u32) -> Result<(), Error> {
    
    let divider = self.calculate_flash_divider(bus_freq)?;

    dbg!(&divider);

    let b3 : u8 = ((divider >> 8) & 0xff) as u8;
    let b4 : u8 = (divider & 0xff) as u8;

    let div_vec : Vec<u8> = vec![b4, b3];

    dbg!(&div_vec);

    let clk_div_before = prog.dsc_read_memory(memory_space_t::MS_XWORD, 0x02, MC56F80XX_FLASH_MODULE_CLKDIV)?;
    dbg!(&clk_div_before);

    prog.dsc_write_memory(memory_space_t::MS_XWORD, div_vec, 0xf400)?;

    let clk_div_after = prog.dsc_read_memory(memory_space_t::MS_XWORD,0x02, MC56F80XX_FLASH_MODULE_CLKDIV)?;
    dbg!(&clk_div_after);

    Ok(())
 }
}

impl TargetInitActions for MC56f80xx {

/// SIM_LSHID + SIM_MSHID
/// This read-only register, in two parts  displays the least significant half of the JTAG ID for the chip.
/// 
/// For example:
/// 
/// Most Significant Half of JTAG ID (`SIM_MSHID`), in MC56f801x is `$01F2`.
/// 
/// Least Significant Half of JTAG ID (`SIM_LSHID`), in MC56f801x is  `$401D`.
/// 
/// PGO wrote in original usbdm pjt, if you have match id code dsc in
/// we have to match `jtag_id_code` with `SIM_ID`
fn is_unsecure(&mut self, prog : &mut Programmer) -> Result<SecurityStatus, Error> {

    let jtag_id_msh = prog.dsc_read_memory(memory_space_t::MS_XWORD, 0x02, self.msh_id_addr)?;
    let jtag_id_lsh = prog.dsc_read_memory(memory_space_t::MS_XWORD, 0x02, self.lsh_id_addr)?;

    print_vec_one_line(&jtag_id_lsh);
    print_vec_one_line(&jtag_id_msh);

    let jtag_id_code =  msh_lsh_vec_as_u32_be(jtag_id_lsh, jtag_id_msh);
    println!("jtag id calculated: {:02X}", &jtag_id_code);

    
    let expected_id  = self.jtag_id_code;

    if (jtag_id_code == expected_id) {  return Ok(SecurityStatus::Unsecured)};

    println!("Expected id: {:02X}, id from target : {:02X}, ", &expected_id, &jtag_id_code);

    let security_status =
    match jtag_id_code {
           0x0                        => SecurityStatus::Secured,
           0xFFFFFFFF                 => return Err(Error::TargetNotConnected),           
           _                          => SecurityStatus::Unknown,             
       };

   Ok(security_status)    
}
/// SIM_LSHID + SIM_MSHID const (jtag id code from datasheet)  should be match with jtag_id from device
fn target_family_confirmation(&mut self, jtag_id : Vec<u8>, core_id : Vec<u8>)-> Result<(), Error> {


    let jtag_id_code =  vec_as_u32_be(jtag_id); 
    let target_device_id = vec_as_u32_be(core_id);

    println!("Core id: {:02X}, jtag id: {:02X}, ", &target_device_id, &jtag_id_code);
    println!("Expected: {:02X}, Expected: {:02X}, ", &self.core_id, &self.jtag_id_code);

    let expected_id  = self.jtag_id_code;
    if (jtag_id_code == expected_id) { return Ok(())};

    let family_from_id =
    match jtag_id_code {
          MC56800X_SIM_ID      => DscFamily::Mc56f800X,
          MC56801X_SIM_ID      => DscFamily::Mc56f801X,
          MC56802X_SIM_ID      => DscFamily::Mc56f802X,
          MC56803X_SIM_ID      => DscFamily::Mc56f803X, 
          0xFFFFFFFF           => return Err(Error::TargetNotConnected),
          _                    => return Err(Error::InternalError("family_from_id parse Failed".to_string()))};

    dbg!(&family_from_id);

    if (self.family != family_from_id || self.core_id != target_device_id) { return Err(Error::TargetWrongFamilySelected(self.family.to_string(), family_from_id.to_string())); }
  
    Ok(())
} 

fn mass_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error> {

    
    let flash_erase_cmd : Vec<u8> = vec![0x08];     // 8 = Lock Out Recovery (Flash_Erase)
    let clk_div = self.calculate_flash_divider(4000)?;
    let b3 : u8 = ((clk_div >> 8) & 0xff) as u8;
    let b4 : u8 = (clk_div & 0xff) as u8;
    let clk_div_vec : Vec<u8> = vec![b4, b3];

    dbg!(&clk_div_vec);
   
    println!("jtag_reset");
    prog.jtag_reset()?;
    println!("jtag_select_shift JTAG_SHIFT_IR");
    prog.jtag_select_shift(jtag_shift::JTAG_SHIFT_IR)?;

    println!("jtag_write flash_erase_cmd");
    prog.jtag_write(jtag_shift::JTAG_EXIT_SHIFT_DR, 0x8, flash_erase_cmd)?;
    //jtag-shift D(JTAG_EXIT_SHIFT_DR) $::JTAG_IR_LENGTH(8) $::JTAG_UNLOCK_CMD(0x08)
    println!("jtag_write clk_div_vec");
    prog.jtag_write(jtag_shift::JTAG_EXIT_IDLE, 0x16, clk_div_vec)?;
    //jtag-shift R(JTAG_EXIT_IDLE) $::JTAG_DR_LENGTH(16) 0 $cfmclkd

    thread::sleep(time::Duration::from_millis(2000));
    println!("jtag_reset");
    prog.jtag_reset()?;

    Ok(())

 }

 fn init_for_write_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer, bus_freq : u32) -> Result<FlashModuleStatus, Error> {

    self.init_flash_divider(power, prog, bus_freq)?;

    let unlock : Vec<u8> = vec![0x0, 0x0];

    let prot_before = prog.dsc_read_memory(memory_space_t::MS_XWORD, 0x02, MC56F80XX_FLASH_MODULE_PROT)?;
    dbg!(&prot_before);

    prog.dsc_write_memory(memory_space_t::MS_XWORD, unlock, MC56F80XX_FLASH_MODULE_PROT)?;

    let prot_after = prog.dsc_read_memory(memory_space_t::MS_XWORD, 0x02, MC56F80XX_FLASH_MODULE_PROT)?;
    dbg!(&prot_after);

    Ok(FlashModuleStatus::Inited)

 }

}






