# usbdm_mc56f_rs

## not finished, in progress !

###  Here I am trying to make my own convenient API to work with the USBDM programmer, the original project is written in C++ and seems a little complicated to me.
 [Original USBDM project](https://github.com/podonoghue/usbdm-eclipse-makefiles-build/tree/85cc87da0808b8fe4ba4ec6ac7f2c450a89fc34e).

 Initially, the goal is to work comfortably with the mc56f in Rust. Working via USBDM on mc56f dsp controllers is rather inconvenient, and original the project itself is too large to simply fix something.
 
 Target - USBDM/CF version, supporting DSC.
 
 ###  In test
* Concrete mc56f (target) commands - read

###  In work
* Concrete mc56f (target) commands - read, write, erase 
* GUI - ideally hex buffer, In work now

###  Done now
* low level USB interface
* few command's to work with USBDM
* Settings module (Set Usbdm Options)
* Feedback module (Get status and parse bits)
* Capabilites (Get BDM Information, like JTAG buffer size etc, parse bytes & save)
* Jtag Command Builder
* Concrete mc56f (target) command: - Connect, tested


