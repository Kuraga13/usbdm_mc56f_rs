use packed_struct::prelude::*;
use crate::errors::{Error};

///`Feedback` 
///The idea is to group a huge number of USBDM structures, enumerations and settings into three abstractions.
/// 
/// One is Feedback - it includes everything we can learn, like a sensor, from USBDM about its target state and USBDM itself, like reset state
/// or power sensor, connection state
/// 
/// `Target Status bit masks`
/// 
///     9       8       7       6       5        4       3       2       1       0
/// +-------+-------+-------+-------+--------+-------+-------+-------+-------+-------+
/// |      VPP      |     Power     |  Halt  | Communication | Reset | ResDet| Ackn  |
/// +-------+-------+-------+-------+--------+-------+-------+-------+-------+-------+
///
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq)]
#[packed_struct(bit_numbering="lsb0",size_bytes="2",endian="lsb",)]
pub struct FeedBack {
   
    #[packed_field(bits="0")]
    ackn_mode: bool,                                // Supports ACKN ?
    #[packed_field(bits="1")]
    reset_recent: bool,                             // Target reset recently?
    #[packed_field(bits="2")]
    reset_state: bool,                              // Current target RST0 state
    #[packed_field(bits="3:4", ty="enum")]
    speed_mode: ConnectionState,                     // Connection status & speed determination method
    #[packed_field(bits="5")]
    halt_state: bool,                              // CFVx halted (from ALLPST)?
    #[packed_field(bits="6:7", ty="enum")]
pub power_state: PowerState,                       // Target has power?
    #[packed_field(bits="8:9", ty="enum")]
    vpp_state: VppState,                           // State of Target Vpp* used only for Rs08
     
}

impl Default for FeedBack {
   fn default() -> Self { 
      FeedBack {
           ackn_mode:              false,
           reset_recent:           false,
           reset_state:            false,
           speed_mode:             ConnectionState::NoInfo,
           halt_state:             false,
           power_state:            PowerState::None,
           vpp_state:              VppState::Off,
       }
   } 
}


impl FeedBack{


   pub fn print_feedback(&self){

   println!("ackn_mode is {:?}", &self.ackn_mode); 
   println!("reset_recent is {:?}", &self.reset_recent); 
   println!("reset_state is {:?}", &self.reset_state); 
   println!("speed_mode is {:?}", &self.speed_mode);  
   println!("halt_state is {:?}", &self.halt_state); 
   println!("power_state is {:?}", &self.power_state); 
   println!("vpp_state is {:?}", &self.vpp_state); 

   }

}


/// Connection status & speed determination method - as field of `FeedBack`
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
 pub enum ConnectionState {
    NoInfo          = 0,   // Not connected
    Sync            = 1,   // Speed determined by SYNC
    Gueesed         = 2,   // Speed determined by trial & error
    UserSupplied    = 3,    // User has specified the speed to use
 } 


/// Connection state of power from USBDM power sensor circuit - as field of `FeedBack`
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
 pub enum PowerState {
    None            = 0,   //Target Vdd not detected
    External        = 1,   //Target Vdd external
    Internal        = 2,   // Target Vdd internal
    Error           = 3,   // Target Vdd error
 } 

 // Use for App
 #[derive(Debug, Clone, PartialEq)]
 pub enum PowerStatus {
     
     PowerOn,
     PowerOff,
 
 }


 impl From<PowerState> for Result<PowerStatus, Error> {
   fn from(power_from_bdm : PowerState) -> Result<PowerStatus, Error> {
     match power_from_bdm {
            PowerState::None         => Ok(PowerStatus::PowerOff),    // Target Vdd not detected
            PowerState::External     => Ok(PowerStatus::PowerOn),    // Target Vdd external (in real life - when you on and off power, power state is External)
            PowerState::Internal     => Ok(PowerStatus::PowerOn),     // Target Vdd internal - On State checked
            PowerState::Error        => Err(Error::PowerErrorInFeedback),  // Target Vdd error Possible overload !
     }    
   }
 }


/// Connection status & speed determination method - as field of `FeedBack`
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
 pub enum VppState  {
    Off       = 0,   //Target Vpp Off
    Stanby    = 1,   // Target Vpp Standby (Inverter on, Vpp off)
    On        = 2,   // Target Vpp On
    Error     = 3,   // Target Vpp ??
 } 
 
