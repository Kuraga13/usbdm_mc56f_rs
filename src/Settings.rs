///`Settings` 
///The idea is to group a huge number of USBDM structures, enumerations and settings into three abstractions.
/// 
/// One is Settings - Here are all the settings that determine the operation of USBDM such as speed, power, etc.

pub struct Settings{


    // Options passed to the BDM

TargetType_t      :  targetType;                 // Target type - have to be init twice ))) On SetExtOptions + set target command
Voltage           :  targetVdd;                  // Target Vdd (off, 3.3V or 5V) - here you only select target vdd, to power ON need send set vdd command
ResetMode         :
Speed                                        //!< CFVx/JTAG etc - Interface speed (kHz) interfaceFrequency

bool               cycleVddOnReset;            //!< Cycle target Power  when resetting
bool               cycleVddOnConnect;          //!< Cycle target Power if connection problems)
bool               leaveTargetPowered;         //!< Leave target power on exit
AutoConnect_t      autoReconnect;              //!< Automatically re-connect to target (for speed change)
bool               guessSpeed;                 //!< Guess speed for target w/o ACKN
ClkSwValues_t      bdmClockSource;             //!< BDM clock source in target
bool               useResetSignal;             //!< Whether to use RESET signal on BDM interface
bool               maskInterrupts;             //!< Whether to mask interrupts when  stepping
unsigned           interfaceFrequency;         
bool               usePSTSignals;              //!< CFVx, PST Signal monitors
unsigned           powerOffDuration;           //!< How long to remove power (ms)
unsigned           powerOnRecoveryInterval;    //!< How long to wait after power enabled (ms)
unsigned           resetDuration;              //!< How long to assert reset (ms)
unsigned           resetReleaseInterval;       //!< How long to wait after reset release to release other signals (ms)
unsigned           resetRecoveryInterval;      //!< How long to wait after reset sequence completes (ms)
unsigned           hcs08sbdfrAddress;          //!< Address to use to access SBDFR register

}