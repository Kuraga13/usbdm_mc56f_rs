use crate::errors::{Error, USBDM_ErrorCode};
use super::*;
use super::jtag::*;

mod memory_space_t {
    // Memory space indicator - includes element size
    // One of the following
    pub const MS_BYTE    : u8  = 1;        // Byte (8-bit) access
    pub const MS_WORD    : u8  = 2;        // Word (16-bit) access
    pub const MS_LONG    : u8  = 4;        // Long (32-bit) access
    // One of the following
    pub const MS_NONE    : u8  = 0<<4;     // Memory space unused/undifferentiated
    pub const MS_PROGRAM : u8  = 1<<4;     // Program memory space (e.g. P: on DSC)
    pub const MS_DATA    : u8  = 2<<4;     // Data memory space (e.g. X: on DSC)
    pub const MS_GLOBAL  : u8  = 3<<4;     // HCS12 Global addresses (Using BDMPPR register)
    // Fast memory access for HCS08/HCS12 (stopped target, regs. are modified
    pub const MS_FAST    : u8  = 1<<7;
    // Masks for above
    pub const MS_SIZE    : u8  = 0x7<<0;   // Size
    pub const MS_SPACE   : u8  = 0x7<<4;   // Memory space
    // For convenience (DSC)
    pub const MS_PWORD   : u8  = MS_WORD + MS_PROGRAM;
    pub const MS_PLONG   : u8  = MS_LONG + MS_PROGRAM;
    pub const MS_XBYTE   : u8  = MS_LONG + MS_DATA;
    pub const MS_XWORD   : u8  = MS_WORD + MS_DATA;
    pub const MS_XLONG   : u8  = MS_LONG + MS_DATA;
}

impl Programmer
{
    // Read X/P memory via ONCE & target execution
    //
    // @param memorySpace - Memory space & size of memory accesses 1/2/4 bytes
    // @param numBytes    - Number of bytes to read (must be a multiple of elementSize)
    // @param address     - Memory address
    // @param buffer      - Where to obtain the data
    //
    // @note If memory space size is word or long size then address is DSC word address
    // @note If memory space size is byte size then address is DSC byte pointer address
    // @note Size is limited to dscInfo.maxMemoryReadSize
    //
    fn read_memory_block(&self, mut memory_space: u8, num_bytes: u8, address: u32) -> Result<(Vec<u8>), Error> {
        if (memory_space == memory_space_t::MS_PLONG) {
            // Treat as word access
            memory_space = memory_space_t::MS_PWORD;
        };

        let mut num_bytes_adjusted = num_bytes;
        match (memory_space & memory_space_t::MS_SIZE) {
            memory_space_t::MS_LONG => {
                if ((address & 0x01) == 0) {
                    num_bytes_adjusted /= 4;
                } else {
                    return Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_ILLEGAL_PARAMS))
                };},
            memory_space_t::MS_WORD => { num_bytes_adjusted /= 2; },
            memory_space_t::MS_BYTE => { num_bytes_adjusted /= 4; },
            other               => return Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_ILLEGAL_PARAMS)),
        };

        /*
         *    +-----------------------+
         *    |    JTAG_READ_MEM      |
         *    +-----------------------+
         *    |    JTAG_END           |
         *    +-----------------------+
         *    |                       |
         *    +--                   --+
         *    |                       |
         *    +--  Memory Address   --+
         *    |                       |
         *    +--                  ---+
         *    |                       |
         *    +-----------------------+
         *    |  # of memory elements |
         *    +-----------------------+
         *    |   Memory Space        |
         *    +-----------------------+
         */

        let mut sequence: Vec<u8> = Vec::with_capacity(JTAG_READ_MEMORY_HEADER_SIZE.into());
        sequence.push(JTAG_READ_MEM);          // 0
        sequence.push(JTAG_END);               // 1
        sequence.push((address >> 24) as u8);  // 2 Address
        sequence.push((address >> 16) as u8);  // 3
        sequence.push((address >> 8) as u8);   // 4
        sequence.push(address as u8);          // 5
        sequence.push(num_bytes_adjusted);     // 6 Elements
        sequence.push(memory_space);           // 7 Memory space

        self.exec_jtag_seq(sequence, num_bytes)
    }


/*
    //================================================================================
    // Read X/P memory via ONCE & target execution
    //
    // @param memorySpace - Memory space & size of memory accesses 1/2/4 bytes
    // @param numBytes    - Number of bytes to read (must be a multiple of elementSize)
    // @param address     - Memory address (byte = byte address, word/long = word address)
    // @param buffer      - Where to obtain the data
    //
    // @note If memory space size is word or long size then address is DSC word address
    // @note If memory space size is byte size then address is DSC byte pointer address
    //
    pub fn dsc_read_memory (memorySpace: u8, numBytes,
                               unsigned int        address,
                               unsigned char       *buffer) {
   LOGGING_Q;

   MemorySpace_t  memSpace     = (MemorySpace_t) memorySpace;
   int            elementSize  = memorySpace&MS_SIZE;
   unsigned char *bufferPtr = buffer;
   unsigned int  bytesDone = 0;
   unsigned int  currentAddress = address;

   USBDM_ErrorCode rc = BDM_RC_OK;

   log.print("(A=%s, S=%d, #=%d)\n", getKnownAddress(memSpace,currentAddress), elementSize, numBytes);

   saveVolatileTargetRegs();

   while (bytesDone<numBytes) {
      unsigned int blockSize = numBytes-bytesDone;
      if (blockSize>dscInfo.maxMemoryReadSize) {
         blockSize = dscInfo.maxMemoryReadSize;
      }
      rc = readMemoryBlock(memSpace, blockSize, currentAddress, bufferPtr);
      if (rc != BDM_RC_OK) {
         break;
      }
      bytesDone += blockSize;
      bufferPtr += blockSize;
      if ((memorySpace&MS_SIZE) == MS_Byte) {
         // Byte currentAddress advanced by count of bytes written
         currentAddress  += blockSize;
      }
      else {
         // Address advanced by count of words written
         currentAddress  += blockSize/2;
      }
   }
   if (elementSize == MS_Byte) {
      log.printDump((uint8_t*)buffer, numBytes, address, UsbdmSystem::BYTE_ADDRESS|UsbdmSystem::BYTE_DISPLAY);
   }
   else {
      log.printDump((uint8_t*)buffer, numBytes, address, UsbdmSystem::WORD_ADDRESS|UsbdmSystem::WORD_DISPLAY);
   }
   return rc;
}
*/
}
