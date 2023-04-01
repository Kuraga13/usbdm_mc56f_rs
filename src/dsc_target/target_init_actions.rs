use crate::errors::Error;
use crate::utils::*;
use super::target_factory::{TargetInitActions, SecurityStatus, TargetDsc};
use crate::usbdm::jtag::*;
use crate::usbdm::jtag::{OnceStatus};
use crate::usbdm::constants::{memory_space_t};
use crate::usbdm::programmer::{Programmer};
use crate::usbdm::settings::{TargetVddSelect};

/// `MC56800X_SIM_ID` combine two bytes of JTAG ID (SIM_MSHID+SIM_LSHID), in mc56801x is  $01F2 401D
pub const MC56800X_SIM_ID : u32 =  0x01F2601D;
/// `MC56801X_SIM_ID` combine two bytes of JTAG ID (SIM_MSHID+SIM_LSHID), in mc56801x is  $01F2 401D
pub const MC56801X_SIM_ID : u32 =  0x01F2401D;
/// `MC56802X_SIM_ID` combine two bytes of JTAG ID (SIM_MSHID+SIM_LSHID), in mc568023-35 is  $01F2801D
pub const MC56802X_SIM_ID : u32 =  0x01F2801D;

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
pub struct MC56f80xx;

impl MC56f80xx {

/// Calculate specific on MC56f80xx Family cfmclkd
fn calculate_flash_divider(&self, bus_frequency : u32) -> Result<u32, Error> {

        const DSC_PRDIV8 : u32 = 0x40;
    
        if (bus_frequency < 1000) {
           println! ("Clock too low for flash programming");
           return Err(Error::InternalError(("PROGRAMMING_RC_ERROR_NO_VALID_FCDIV_VALUE".to_string())));
         
        };
     
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
            return Err(Error::InternalError(("PROGRAMMING_RC_ERROR_NO_VALID_FCDIV_VALUE".to_string())));
        }
     
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
fn is_unsecure(&mut self, prog : &mut Programmer, expected_id : u32) -> Result<SecurityStatus, Error> {

    let jtag_id_code =  vec_as_u32_be(read_master_id_code_DSC_JTAG_ID(true, &prog)?);

    match jtag_id_code {
           expected_id           => Ok(SecurityStatus::Unsecured),
           0x0                        => Ok(SecurityStatus::Secured),           
           _                          => Ok(SecurityStatus::Unknown),             
       }    
 } 

fn mass_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer) -> Result<(), Error> {

    unimplemented!();
    
    let ustat_before = prog.dsc_read_memory(memory_space_t::MS_XWORD, 0x02, MC56F80XX_FLASH_MODULE_USTAT)?;
    dbg!(&ustat_before);

    let ustat_after = prog.dsc_read_memory(memory_space_t::MS_XWORD, 0x02, MC56F80XX_FLASH_MODULE_USTAT)?;
    dbg!(&ustat_after);

   

 }

 fn init_for_write_erase(&mut self, power : TargetVddSelect, prog : &mut Programmer, bus_freq : u32) -> Result<(), Error> {

    self.init_flash_divider(power, prog, bus_freq)?;

    let unlock : Vec<u8> = vec![0x0, 0x0];

    let prot_before = prog.dsc_read_memory(memory_space_t::MS_XWORD, 0x02, MC56F80XX_FLASH_MODULE_PROT)?;
    dbg!(&prot_before);

    prog.dsc_write_memory(memory_space_t::MS_XWORD, unlock, MC56F80XX_FLASH_MODULE_PROT)?;

    let prot_after = prog.dsc_read_memory(memory_space_t::MS_XWORD, 0x02, MC56F80XX_FLASH_MODULE_PROT)?;
    dbg!(&prot_after);

    Ok(())

 }

}






