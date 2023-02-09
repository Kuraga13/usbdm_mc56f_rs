# usbdm_mc56f_rs

## not finished, in progress !

###  Here I am trying to make my own convenient API to work with the USBDM programmer, the original project is written in C++ and seems a little complicated to me.
 [Original USBDM project](https://github.com/podonoghue/usbdm-eclipse-makefiles-build/tree/85cc87da0808b8fe4ba4ec6ac7f2c450a89fc34e).

 Initially, the goal is to work comfortably with the mc56f in Rust. Working via USBDM on mc56f dsp controllers is rather inconvenient, and original the project itself is too large to simply fix something.

###  Done now
* low level USB interface
* few command's to work with USBDM

###  In plan
* Settings module
* Feedback module
* JTAG module & test on mc56f8035
