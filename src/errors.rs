#![allow(unused)]

use std::fmt;


//#[allow(non_camel_case_types)]
//pub type USBDM_Result = Result<USBDM_RC_OK, USBDMerror>;


#[allow(non_camel_case_types)]
#[derive(Debug,Copy, Clone)]
pub enum Error {

   USBDM_Errors(USBDM_ErrorCode),
  // USBDM_ErrorCode,
   Usb(rusb::Error),
   PowerStateError,
   LostConnection,
   TargetNotConnected,
   Unknown,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

impl From<rusb::Error> for Error {
    fn from(error: rusb::Error) -> Error {
        Error::Usb(error)      
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
#[derive(PartialEq, Copy, Clone)]
pub enum USBDM_ErrorCode {

    BDM_RC_OK                                     ,     // OK All right
    BDM_RC_ERROR_HANDLED                          ,     // Indicates error has already been notified to user
    BDM_RC_ILLEGAL_PARAMS                         ,     // Illegal parameters to command
    BDM_RC_FAIL                                   ,     // General Fail
    BDM_RC_BUSY                                   ,     // Busy with last command - try again - don't change
    BDM_RC_ILLEGAL_COMMAND                        ,     // Illegal (unknown) command (may be in wrong target mode)
    BDM_RC_NO_CONNECTION                          ,     // No connection to target
    BDM_RC_OVERRUN                                ,     // New command before previous command completed
    BDM_RC_CF_ILLEGAL_COMMAND                     ,     // Coldfire BDM interface did not recognize the command
    BDM_RC_DEVICE_OPEN_FAILED                     ,     // BDM Open Failed - Other LIBUSB error on open
    BDM_RC_USB_DEVICE_BUSY                        ,     // BDM Open Failed - LIBUSB_ERROR_ACCESS on open - Probably open in another app
    BDM_RC_USB_DEVICE_NOT_INSTALLED               ,    // BDM Open Failed - LIBUSB_ERROR_ACCESS on claim I/F - Probably driver not installed
    BDM_RC_USB_DEVICE_REMOVED                     ,    // BDM Open Failed - LIBUSB_ERROR_NO_DEVICE - enumerated device has been removed
    BDM_RC_USB_RETRY_OK                           ,    // USB Debug use only
    BDM_RC_UNEXPECTED_RESET                       ,    // Target reset was detected
    BDM_RC_CF_NOT_READY                           ,    // Coldfire 2,3,4 Not ready response (running?)
    BDM_RC_UNKNOWN_TARGET                         ,    // Target unknown or not supported by this BDM
    BDM_RC_NO_TX_ROUTINE                          ,    // No Tx routine available at measured BDM communication speed
    BDM_RC_NO_RX_ROUTINE                          ,    // No Rx routine available at measured BDM communication speed
    BDM_RC_BDM_EN_FAILED                          ,    // Failed to enable BDM mode in target (warning)
    BDM_RC_RESET_TIMEOUT_FALL                     ,    // RESET signal failed to fall
    BDM_RC_BKGD_TIMEOUT                           ,    // BKGD signal failed to rise/fall
    BDM_RC_SYNC_TIMEOUT                           ,    // No response to SYNC sequence
    BDM_RC_UNKNOWN_SPEED                          ,    // Communication speed is not known or cannot be determined
    BDM_RC_WRONG_PROGRAMMING_MODE                 ,    // Attempted Flash programming when in wrong mode (e.g. Vpp off)
    BDM_RC_FLASH_PROGRAMING_BUSY                  ,    // Busy with last Flash programming command
    BDM_RC_VDD_NOT_REMOVED                        ,    // Target Vdd failed to fall
    BDM_RC_VDD_NOT_PRESENT                        ,    // Target Vdd not present/failed to rise
    BDM_RC_VDD_WRONG_MODE                         ,    // Attempt to cycle target Vdd when not controlled by BDM interface
    BDM_RC_CF_BUS_ERROR                           ,    // Illegal bus cycle on target (Coldfire)
    BDM_RC_USB_ERROR                              ,    // Indicates USB transfer failed (returned by driver not BDM)
    BDM_RC_ACK_TIMEOUT                            ,    // Indicates an expected ACK was missing
    BDM_RC_FAILED_TRIM                            ,    // Trimming of target clock failed (out of clock range?).
    BDM_RC_FEATURE_NOT_SUPPORTED                  ,    // Feature not supported by this version of hardware/firmware
    BDM_RC_RESET_TIMEOUT_RISE                     ,    // RESET signal failed to rise
   
    // Used by USBDM DLL
    BDM_RC_WRONG_BDM_REVISION                     ,    // BDM Hardware is incompatible with driver/program
    BDM_RC_WRONG_DLL_REVISION                     ,    // Program is incompatible with DLL
    BDM_RC_NO_USBDM_DEVICE                        ,    // No USBDM device was located
   
    BDM_RC_JTAG_UNMATCHED_REPEAT                  ,    // Unmatched REPEAT-END_REPEAT
    BDM_RC_JTAG_UNMATCHED_RETURN                  ,    // Unmatched CALL-RETURN
    BDM_RC_JTAG_UNMATCHED_IF                      ,    // Unmatched IF-END_IF
    BDM_RC_JTAG_STACK_ERROR                       ,    // Underflow in call/return sequence, unmatched REPEAT etc.
    BDM_RC_JTAG_ILLEGAL_SEQUENCE                  ,    // Illegal JTAG sequence
    BDM_RC_TARGET_BUSY                            ,    // Target is busy (executing?)
    BDM_RC_JTAG_TOO_LARGE                         ,    // Subroutine is too large to cache
    BDM_RC_DEVICE_NOT_OPEN                        ,    // USBDM Device has not been opened
    BDM_RC_UNKNOWN_DEVICE                         ,    // Device is not in database
    BDM_RC_DEVICE_DATABASE_ERROR                  ,    // Device database not found or failed to open/parse
   
    BDM_RC_ARM_PWR_UP_FAIL                        ,    // ARM System power failed
    BDM_RC_ARM_ACCESS_ERROR                       ,    // ARM Access error
   
    BDM_JTAG_TOO_MANY_DEVICES                     ,    // JTAG chain is too long (or greater than 1!)
   
    BDM_RC_SECURED                                ,    // ARM Device is secured (& operation failed?)
    BDM_RC_ARM_PARITY_ERROR                       ,    // ARM PARITY error
    BDM_RC_ARM_FAULT_ERROR                        ,    // ARM FAULT response error
    BDM_RC_UNEXPECTED_RESPONSE                    ,    // Unexpected/inconsistent response from BDM
    BDM_RC_HCS_ACCESS_ERROR                       ,    // Memory access failed due to target in stop or wait state
    BDM_RC_SELECTED_BDM_NOT_FOUND                 ,    // Selected BDM not found (removed)
    BDM_RC_NOT_INITIALISED                        ,    // Interface not initialised before use e.g. failed to call USBDM_Init()
    BDM_RC_OPERATION_NOT_SUPPORTED                ,    // Operation not supported for target
    BDM_RC_CF_DATA_INVALID                        ,    // CF target returned data invalid response (whatever that means!)
    BDM_RC_CF_OVERRUN                             ,    // CF target returned overrun response
    BDM_RC_MASS_ERASE_DISABLED                    ,    // ARM Device has mass erase disabled
    BDM_RC_FLASH_NOT_READY                        ,    // ARM - Flash failed to become ready
    BDM_RC_VDD_INCORRECT_LEVEL                    ,    // Target Vdd not at expected level (only applicable when internally controlled)
    BDM_RC_VDD_WRONG_FOR_TARGET                   ,    // Target Vdd not at acceptable level for target device
   
    // Used by programmer
    PROGRAMMING_RC_OK                             ,     //  0 Success
    PROGRAMMING_RC_ERROR_FIRST_MESSAGE            ,
    PROGRAMMING_RC_ERROR_ILLEGAL_PARAMS           ,   //  1 Programming parameters incorrect
    PROGRAMMING_RC_ERROR_WRONG_SDID               ,   //  2 Incorrect target device
    PROGRAMMING_RC_ERROR_FAILED_VERIFY            ,   //  3 Verification of Flash failed
    PROGRAMMING_RC_ERROR_BDM                      ,   //  4 General BDM error
    PROGRAMMING_RC_ERROR_NOT_BLANK                ,   //  5 Device is not blank/failed erase
    PROGRAMMING_RC_ERROR_BDM_NO_DEVICES           ,   //  6 No USBDM devices found
    PROGRAMMING_RC_ERROR_BDM_OPEN                 ,   //  7 Failed to open USBDM device
    PROGRAMMING_RC_ERROR_BDM_CONNECT              ,   //  8 Failed to connect to target
    PROGRAMMING_RC_ERROR_BDM_TARGET               ,   //  9 Failed to set target type
    PROGRAMMING_RC_ERROR_BDM_WRITE                ,   // 10 Failed to write to target
    PROGRAMMING_RC_ERROR_BDM_READ                 ,   // 11 Failed to read from target
    PROGRAMMING_RC_ERROR_BDM_RESET                ,   // 12 Failed to reset target
    PROGRAMMING_RC_ERROR_TRIM                     ,   // 13 Trimming target clock failed
    PROGRAMMING_RC_ERROR_SECURED                  ,   // 14 Target is secured and cannot be programmed
    PROGRAMMING_RC_ERROR_FAILED_FLASH_COMMAND     ,   // 15 Flash command failed
    PROGRAMMING_RC_ERROR_NO_VALID_FCDIV_VALUE     ,   // 16 Failed to find a suitable FCDIV value (clock problem?)
    PROGRAMMING_RC_ERROR_CHECKSUM                 ,   // 17 Checksum of SREC invalid
    PROGRAMMING_RC_ERROR_FAILED_CLOCK             ,   // 18 Failed setup of target clock (connection lost)
    PROGRAMMING_RC_ERROR_INTERNAL_CHECK_FAILED    ,   // 19 Failed an internal software check - should be impossible!
    PROGRAMMING_RC_ERROR_FILE_OPEN_FAIL           ,   // 20 Failed to open S1S9 file
    PROGRAMMING_RC_ERROR_PPAGE_FAIL               ,   // 21 Access to PPAGE register failed
    PROGRAMMING_RC_ERROR_EPAGE_FAIL               ,   // 22 Access to EPAGE register failed
    PROGRAMMING_RC_ERROR_SPEED_APPROX             ,   // 23 Can only approximate the target bus speed
    PROGRAMMING_RC_ERROR_CHIP_UNSUPPORTED         ,   // 24 This chip and/or operation is supported due to target hardware bug
    PROGRAMMING_RC_ERROR_TCL_SCRIPT               ,   // 25 Execution of TCL script returned a error
    PROGRAMMING_RC_ERROR_TCL_UNSECURE_SCRIPT      ,   // 26 Execution of TCL script returned a error
    PROGRAMMING_RC_ERROR_TCL_PREPROGRAM_SCRIPT    ,   // 27 Execution of TCL script returned a error
    PROGRAMMING_RC_ERROR_TCL_POSTPROGRAM_SCRIPT   ,   // 28 Execution of TCL script returned a error
    PROGRAMMING_RC_ERROR_OUTSIDE_TARGET_FLASH     ,   // 29 Image is outside target Flash memory
    PROGRAMMING_RC_ERROR_ILLEGAL_SECURITY         ,   // 30 Illegal Security value (will lock chip forever)
    PROGRAMMING_RC_FLEXNVM_CONFIGURATION_FAILED   ,   // 31 Failed to program FlexNVM Configuration values.
   
    // File Loader errors
    SFILE_RC_OK                                   ,    // No error
    SFILE_RC_FIRST_MESSAGE                        ,
    SFILE_RC_CHECKSUM                             ,  // S-record has incorrect checksum
    SFILE_RC_ILLEGAL_LINE                         , // S-record has invalid/unsupported record
    SFILE_RC_FILE_OPEN_FAILED                     , // Hex file failed to open (fopen() failed)
    SFILE_RC_ELF_FORMAT_ERROR                     , // ELF file does not have the expected format
    SFILE_RC_UNKNOWN_FILE_FORMAT                  , // File is not recognised as ELF or SREC
    SFILE_RC_ELF_WRONG_TARGET                     , // ELF is intended for another target
    SFILE_RC_IMAGE_OVERLAPS                       , // File being loaded overlaps existing contents (will still be loaded)
    Unknown_code                                  , // Unknown code
}


#[allow(non_camel_case_types)]
impl From<u8> for USBDM_ErrorCode {
    fn from(error : u8) -> USBDM_ErrorCode {
        match error {
           0            =>             USBDM_ErrorCode::BDM_RC_OK  ,
           1            =>             USBDM_ErrorCode::BDM_RC_ILLEGAL_PARAMS,                              // Illegal parameters to command
           2            =>             USBDM_ErrorCode::BDM_RC_FAIL ,                                       // General Fail
           3            =>             USBDM_ErrorCode::BDM_RC_BUSY,                                        // Busy with last command - try again - don't change
           4            =>             USBDM_ErrorCode::BDM_RC_ILLEGAL_COMMAND,                             // Illegal (unknown) command (may be in wrong target mode)
           5            =>             USBDM_ErrorCode::BDM_RC_NO_CONNECTION                          ,     // No connection to target
           6            =>             USBDM_ErrorCode::BDM_RC_OVERRUN                                ,     // New command before previous command completed
           7            =>             USBDM_ErrorCode::BDM_RC_CF_ILLEGAL_COMMAND                      ,    // Coldfire USBDM_ErrorCode::BDM interface did not recognize the command
           8            =>             USBDM_ErrorCode::BDM_RC_DEVICE_OPEN_FAILED                      ,    // USBDM_ErrorCode::BDM Open Failed - Other LIBUSB error on open
           9            =>             USBDM_ErrorCode::BDM_RC_USB_DEVICE_BUSY                         ,    // USBDM_ErrorCode::BDM Open Failed - LIBUSB_ERROR_ACCESS on open - Probably open in another app
           10           =>             USBDM_ErrorCode::BDM_RC_USB_DEVICE_NOT_INSTALLED                ,    // USBDM_ErrorCode::BDM Open Failed - LIBUSB_ERROR_ACCESS on claim I/F - Probably driver not installed
           11           =>             USBDM_ErrorCode::BDM_RC_USB_DEVICE_REMOVED                      ,    // USBDM_ErrorCode::BDM Open Failed - LIBUSB_ERROR_NO_DEVICE - enumerated device has been removed
           12           =>             USBDM_ErrorCode::BDM_RC_USB_RETRY_OK                            ,    // USB Debug use only
           13           =>             USBDM_ErrorCode::BDM_RC_UNEXPECTED_RESET                        ,    // Target reset was detected
           14           =>             USBDM_ErrorCode::BDM_RC_CF_NOT_READY                            ,    // Coldfire 2,3,4 Not ready response (running?)
           15           =>             USBDM_ErrorCode::BDM_RC_UNKNOWN_TARGET                          ,    // Target unknown or not supported by this USBDM_ErrorCode::BDM
           16           =>             USBDM_ErrorCode::BDM_RC_NO_TX_ROUTINE                           ,    // No Tx routine available at measured USBDM_ErrorCode::BDM communication speed
           17           =>             USBDM_ErrorCode::BDM_RC_NO_RX_ROUTINE                           ,    // No Rx routine available at measured USBDM_ErrorCode::BDM communication speed
           18           =>             USBDM_ErrorCode::BDM_RC_BDM_EN_FAILED                           ,    // Failed to enable USBDM_ErrorCode::BDM mode in target (warning)
           19           =>             USBDM_ErrorCode::BDM_RC_RESET_TIMEOUT_FALL                      ,    // RESET signal failed to fall
           20           =>             USBDM_ErrorCode::BDM_RC_BKGD_TIMEOUT                            ,    // BKGD signal failed to rise/fall
           21           =>             USBDM_ErrorCode::BDM_RC_SYNC_TIMEOUT                            ,    // No response to SYNC sequence
           22           =>             USBDM_ErrorCode::BDM_RC_UNKNOWN_SPEED                           ,    // Communication speed is not known or cannot be determined
           23           =>             USBDM_ErrorCode::BDM_RC_WRONG_PROGRAMMING_MODE                  ,    // Attempted Flash USBDM_ErrorCode::PROGRAMMING when in wrong mode (e.g. Vpp off)
           24           =>             USBDM_ErrorCode::BDM_RC_FLASH_PROGRAMING_BUSY                   ,    // Busy with last Flash USBDM_ErrorCode::PROGRAMMING command
           25           =>             USBDM_ErrorCode::BDM_RC_VDD_NOT_REMOVED                         ,    // Target Vdd failed to fall
           26           =>             USBDM_ErrorCode::BDM_RC_VDD_NOT_PRESENT                         ,    // Target Vdd not present/failed to rise
           27           =>             USBDM_ErrorCode::BDM_RC_VDD_WRONG_MODE                          ,    // Attempt to cycle target Vdd when not controlled by USBDM_ErrorCode::BDM interface
           28           =>             USBDM_ErrorCode::BDM_RC_CF_BUS_ERROR                            ,    // Illegal bus cycle on target (Coldfire)
           29           =>             USBDM_ErrorCode::BDM_RC_USB_ERROR                               ,    // Indicates USB transfer failed (returned by driver not USBDM_ErrorCode::BDM)
           30           =>             USBDM_ErrorCode::BDM_RC_ACK_TIMEOUT                             ,    // Indicates an expected ACK was missing
           31           =>             USBDM_ErrorCode::BDM_RC_FAILED_TRIM                             ,    // Trimming of target clock failed (out of clock range?).
           32           =>             USBDM_ErrorCode::BDM_RC_FEATURE_NOT_SUPPORTED                   ,    // Feature not supported by this version of hardware/firmware
           33           =>             USBDM_ErrorCode::BDM_RC_RESET_TIMEOUT_RISE                      ,    // RESET signal failed to rise  
           34           =>             USBDM_ErrorCode::BDM_RC_WRONG_BDM_REVISION                      ,    // USBDM_ErrorCode::BDM Hardware is incompatible with driver/program
           35           =>             USBDM_ErrorCode::BDM_RC_WRONG_DLL_REVISION                      ,    // Program is incompatible with DLL
           36           =>             USBDM_ErrorCode::BDM_RC_NO_USBDM_DEVICE                         ,    // No USUSBDM_ErrorCode::BDM device was located      
           37           =>             USBDM_ErrorCode::BDM_RC_JTAG_UNMATCHED_REPEAT                   ,    // Unmatched REPEAT-END_REPEAT
           38           =>             USBDM_ErrorCode::BDM_RC_JTAG_UNMATCHED_RETURN                   ,    // Unmatched CALL-RETURN
           39           =>             USBDM_ErrorCode::BDM_RC_JTAG_UNMATCHED_IF                       ,    // Unmatched IF-END_IF
           40           =>             USBDM_ErrorCode::BDM_RC_JTAG_STACK_ERROR                        ,    // Underflow in call/return sequence, unmatched REPEAT etc.
           41           =>             USBDM_ErrorCode::BDM_RC_JTAG_ILLEGAL_SEQUENCE                   ,    // Illegal JTAG sequence
           42           =>             USBDM_ErrorCode::BDM_RC_TARGET_BUSY                             ,    // Target is busy (executing?)
           43           =>             USBDM_ErrorCode::BDM_RC_JTAG_TOO_LARGE                          ,    // Subroutine is too large to cache
           44           =>             USBDM_ErrorCode::BDM_RC_DEVICE_NOT_OPEN                         ,    // USUSBDM_ErrorCode::BDM Device has not been opened
           45           =>             USBDM_ErrorCode::BDM_RC_UNKNOWN_DEVICE                          ,    // Device is not in database
           46           =>             USBDM_ErrorCode::BDM_RC_DEVICE_DATABASE_ERROR                   ,    // Device database not found or failed to open/parse
           47           =>             USBDM_ErrorCode::BDM_RC_ARM_PWR_UP_FAIL                         ,    // ARM System power failed
           48           =>             USBDM_ErrorCode::BDM_RC_ARM_ACCESS_ERROR                        ,    // ARM Access error 
           49           =>             USBDM_ErrorCode::BDM_JTAG_TOO_MANY_DEVICES                      ,    // JTAG chain is too long (or greater than 1!)    
           50           =>             USBDM_ErrorCode::BDM_RC_SECURED                                 ,    // ARM Device is secured (& operation failed?)
           51           =>             USBDM_ErrorCode::BDM_RC_ARM_PARITY_ERROR                        ,    // ARM PARITY error
           52           =>             USBDM_ErrorCode::BDM_RC_ARM_FAULT_ERROR                         ,    // ARM FAULT response error
           53           =>             USBDM_ErrorCode::BDM_RC_UNEXPECTED_RESPONSE                     ,    // Unexpected/inconsistent response from USBDM_ErrorCode::BDM
           54           =>             USBDM_ErrorCode::BDM_RC_HCS_ACCESS_ERROR                        ,    // Memory access failed due to target in stop or wait state
           55           =>             USBDM_ErrorCode::BDM_RC_SELECTED_BDM_NOT_FOUND                  ,    // Selected USBDM_ErrorCode::BDM not found (removed)
           56           =>             USBDM_ErrorCode::BDM_RC_NOT_INITIALISED                         ,    // Interface not initialised before use e.g. failed to call USUSBDM_ErrorCode::BDM_Init()
           57           =>             USBDM_ErrorCode::BDM_RC_OPERATION_NOT_SUPPORTED                 ,    // Operation not supported for target
           58           =>             USBDM_ErrorCode::BDM_RC_CF_DATA_INVALID                         ,    // CF target returned data invalid response (whatever that means!)
           59           =>             USBDM_ErrorCode::BDM_RC_CF_OVERRUN                              ,    // CF target returned overrun response
           60           =>             USBDM_ErrorCode::BDM_RC_MASS_ERASE_DISABLED                     ,    // ARM Device has mass erase disabled
           61           =>             USBDM_ErrorCode::BDM_RC_FLASH_NOT_READY                         ,    // ARM - Flash failed to become ready
           62           =>             USBDM_ErrorCode::BDM_RC_VDD_INCORRECT_LEVEL                     ,    // Target Vdd not at expected level (only applicable when internally controlled)
           63           =>             USBDM_ErrorCode::BDM_RC_VDD_WRONG_FOR_TARGET                    ,    // Target Vdd not at acceptable level for target device                         ,     //  0 Success
           101          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_FIRST_MESSAGE             ,
           101          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_ILLEGAL_PARAMS            ,   //  1 USBDM_ErrorCode::PROGRAMMING parameters incorrect
           102          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_WRONG_SDID                ,   //  2 Incorrect target device
           103          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_FAILED_VERIFY             ,   //  3 Verification of Flash failed
           104          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_BDM                       ,   //  4 General USBDM_ErrorCode::BDM error
           105          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_NOT_BLANK                 ,   //  5 Device is not blank/failed erase
           106          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_BDM_NO_DEVICES            ,   //  6 No USUSBDM_ErrorCode::BDM devices found
           107          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_BDM_OPEN                  ,   //  7 Failed to open USUSBDM_ErrorCode::BDM device
           108          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_BDM_CONNECT               ,   //  8 Failed to connect to target
           109          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_BDM_TARGET                ,   //  9 Failed to set target type
           110          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_BDM_WRITE                 ,   // 10 Failed to write to target
           111          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_BDM_READ                  ,   // 11 Failed to read from target
           112          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_BDM_RESET                 ,   // 12 Failed to reset target
           113          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_TRIM                      ,   // 13 Trimming target clock failed
           114          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_SECURED                   ,   // 14 Target is secured and cannot be programmed
           115          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_FAILED_FLASH_COMMAND      ,   // 15 Flash command failed
           116          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_NO_VALID_FCDIV_VALUE      ,   // 16 Failed to find a suitable FCDIV value (clock problem?)
           117          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_CHECKSUM                  ,   // 17 Checksum of SREC invalid
           118          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_FAILED_CLOCK              ,   // 18 Failed setup of target clock (connection lost)
           119          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_INTERNAL_CHECK_FAILED     ,   // 19 Failed an internal software check - should be impossible!
           120          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_FILE_OPEN_FAIL            ,   // 20 Failed to open S1S9 file
           121          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_PPAGE_FAIL                ,   // 21 Access to PPAGE register failed
           122          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_EPAGE_FAIL                ,   // 22 Access to EPAGE register failed
           123          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_SPEED_APPROX              ,   // 23 Can only approximate the target bus speed
           124          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_CHIP_UNSUPPORTED          ,   // 24 This chip and/or operation is supported due to target hardware bug
           125          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_TCL_SCRIPT                ,   // 25 Execution of TCL script returned a error
           126          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_TCL_UNSECURE_SCRIPT       ,   // 26 Execution of TCL script returned a error
           127          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_TCL_PREPROGRAM_SCRIPT     ,   // 27 Execution of TCL script returned a error
           128          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_TCL_POSTPROGRAM_SCRIPT    ,   // 28 Execution of TCL script returned a error
           129          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_OUTSIDE_TARGET_FLASH      ,   // 29 Image is outside target Flash memory
           130          =>             USBDM_ErrorCode::PROGRAMMING_RC_ERROR_ILLEGAL_SECURITY          ,   // 30 Illegal Security value (will lock chip forever)
           131          =>             USBDM_ErrorCode::PROGRAMMING_RC_FLEXNVM_CONFIGURATION_FAILED    ,   // 31 Failed to program FlexNVM Configuration values. 
           201          =>             USBDM_ErrorCode::SFILE_RC_FIRST_MESSAGE                         ,
           201          =>             USBDM_ErrorCode::SFILE_RC_CHECKSUM                              ,  // S-record has incorrect checksum
           202          =>             USBDM_ErrorCode::SFILE_RC_ILLEGAL_LINE                           , // S-record has invalid/unsupported record
           203          =>             USBDM_ErrorCode::SFILE_RC_FILE_OPEN_FAILED                       , // Hex file failed to open (fopen() failed)
           204          =>             USBDM_ErrorCode::SFILE_RC_ELF_FORMAT_ERROR                       , // ELF file does not have the expected format
           205          =>             USBDM_ErrorCode::SFILE_RC_UNKNOWN_FILE_FORMAT                    , // File is not recognised as ELF or SREC
           206          =>             USBDM_ErrorCode::SFILE_RC_ELF_WRONG_TARGET                       , // ELF is intended for another target
           207          =>             USBDM_ErrorCode::SFILE_RC_IMAGE_OVERLAPS                         ,
           _            =>             USBDM_ErrorCode::Unknown_code    
       }
    }
}





impl fmt::Display for USBDM_ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}



fn test_error(return_code : u8) -> Result< String, USBDM_ErrorCode>
{
    //let status = u32::from(USBDM_ErrorCode::BDM_RC_OK);
    //let status = USBDM_RC_OK::BDM_RC_OK;
   // let status2 = USBDMerror::VoltageDivisionByZero;
    let status = USBDM_ErrorCode::from(return_code);
    match return_code
    {
    0 => Ok(status.to_string()),
    x => Err(status),
    //x => panic!("Unexpected invalid token {:?}", x),
    }
    
}


