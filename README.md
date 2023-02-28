# usbdm_mc56f_rs

### Сonvenient programmer for work with mc56f80xx DSC, at the same time I am and my friend  practicing writing in Rust
 [Original USBDM project](https://github.com/podonoghue/usbdm-eclipse-makefiles-build/tree/85cc87da0808b8fe4ba4ec6ac7f2c450a89fc34e).

![screen](https://user-images.githubusercontent.com/81683023/221999257-ebcfb0e4-84c3-4446-b51d-bdb6df5c6dc3.png)


###  Motivation
The original project does not support reading the controller. This is the underlying reason. Little things associated with the inconvenience of management.
Initially, the goal is to work comfortably with the mc56f in Rust. Working via USBDM on mc56f dsp controllers is rather inconvenient, and original the project itself is too large to simply fix something.
 
Target - USBDM/CF version, supporting DSC.
 
 ###  On test now

* Concrete mc56f (target) commands - read

###  In work
* Concrete mc56f (target) commands - write, erase 
* S19 loader, parser, we do specifically for format s19 from usbdm, s325

### In plan
* Made HexBuffer interactive, copying data, addressing etc.
* Make target_factory from yaml file with parameters


###  Done now
* low level USB interface
* few command's to work with USBDM
* Settings module (Set Usbdm Options)
* Feedback module (Get status and parse bits)
* Capabilites (Get BDM Information, like JTAG buffer size etc, parse bytes & save)
* Jtag Command Builder
* Concrete mc56f (target) command: - Connect, tested
* GUI - hex-buffer, download-upload binary

 ###  Acknowledgements

Thanks to the authors of the original Usbdm project, they did a great job. [Original USBDM project](https://github.com/podonoghue/usbdm-eclipse-makefiles-build/tree/85cc87da0808b8fe4ba4ec6ac7f2c450a89fc34e).
Thanks to the authors of Iced, Iced_aw especially for the menu widget.
And thanks to everyone who helps and help early.  
