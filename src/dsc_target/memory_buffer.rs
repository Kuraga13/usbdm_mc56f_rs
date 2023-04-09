use core::ops::Range;
use crate::errors::Error;


pub const HEX_LINE_LENGHT  : usize =  0x10;


#[derive(Debug)]
pub struct MemoryBuffer {

    /// A `buffer` builded for view, 
    buffer     : Vec<Vec<u8>>,

    /// native address `range` from target memory map eq flash_programm area 
    range      : Range<usize>,

    ///  `cell_size` in bytes, how much bytes on one address
    cell_size  : usize,
}


impl MemoryBuffer {

pub fn init_empty(fill_byte : u8, flash_seg : Range<usize>, cell_size : usize) -> Self {
  
  let number_of_hex_lines = ((((flash_seg.end - flash_seg.start) + 1) * cell_size) / HEX_LINE_LENGHT);
  dbg!(&number_of_hex_lines);

  let number_of_hex_lines2 = dsc_buffer_size_from_range(flash_seg.clone(), cell_size) / HEX_LINE_LENGHT;
  dbg!(&number_of_hex_lines2);


       Self {
        
         buffer : vec![vec![fill_byte; HEX_LINE_LENGHT]; number_of_hex_lines],
         range  : flash_seg,
         cell_size,
  } 
}
////

pub fn download_target_block(&self, block_start_addr : usize, block_size : usize) -> Result<Vec<u8>, Error> {

  if(block_start_addr < self.range.start || block_start_addr > self.range.end) {

    return Err(Error::InternalError("Try do download block from buff out of programm range!".to_string()));

  }
                                                                                       
  let start_drain = (block_start_addr - self.range.start) * self.cell_size;   
  let mut end_drain = start_drain + (block_size * self.cell_size);                    

  let mut buffer= self.download_in_one();
  let buffer_last_addr = buffer.len();                              
                                                                  

  if end_drain > buffer_last_addr {                                        
    end_drain = buffer_last_addr;
  };

 // println!("start_drain {}, end_drain {}, buffer_last_addr = {}", start_drain, end_drain, buffer_last_addr);


  let target_block: Vec<u8> = buffer.drain(start_drain..end_drain).collect(); 

  Ok(target_block)
}



pub fn download_all_target(&self) -> Vec<u8> {
                                                                                       
  let mut buffer= self.download_in_one();

  let flash_memory_size = dsc_buffer_size_from_range(self.range.clone(), self.cell_size);

  //println!("buffer.len() {}, flash_memory_size = {}", buffer.len(), flash_memory_size);
                                                                               
  buffer
}

pub fn download_export_fs(&self, empty_fill : u8) -> Result<Vec<u8>, Error> {
                                                                                       
  let mut buffer= self.download_all_target();

  let mut export : Vec<u8> = Vec::new();
  if(self.range.start != 0) {

    let new_size = self.range.start * self.cell_size;
    export.resize(new_size, empty_fill);

  }

  export.append(&mut buffer);

  let flash_memory_size = dsc_buffer_size_from_range(self.range.clone(), self.cell_size);
  let full_size = (self.range.end + 1) * self.cell_size;


  println!("export.len() {}, full_size {}, flash_memory_size = {}", export.len(), full_size, flash_memory_size);
                                                                        

  Ok(export)
}


  /// `resize` unimplemented! is draft!
  /// 
  /// idea is copy data from buffer & resize buffer, than user change target, so you don't need reload 
  /// files
  fn resize(&mut self, fill_byte : u8, resized_range : Range<usize>, cell : usize) -> Self {

    unimplemented!();
    // if out of range - clear buffer and reload empty value
    if (resized_range.start >= self.range.end || resized_range.end <= self.range.start) {

        dbg!();
        return Self::init_empty(fill_byte, resized_range, cell);
    }

    let start : usize = 
    if(resized_range.start > self.range.start) {

      resized_range.start / HEX_LINE_LENGHT

     } 
     else if(resized_range.start <= self.range.start) {
     
      0
      
     } else {

      0

     };

     let end = 
     if(resized_range.end < self.range.end) {

        resized_range.end / HEX_LINE_LENGHT

     } else {

      ((self.range.end - self.range.start) + 1) / HEX_LINE_LENGHT

     };

     dbg!(&start);
     dbg!(&end);

     let mut drained: Vec<Vec<u8>> = self.buffer.drain(start..end).collect(); 

     let new_len = ((resized_range.end - resized_range.start) + 1) / HEX_LINE_LENGHT;
     dbg!(&new_len);
     dbg!(&drained.len());

     if(new_len != drained.len()) { 

        drained.resize(new_len, vec![fill_byte; HEX_LINE_LENGHT]);
   

    } else {};

      Self {
       
        buffer    : drained,
        range     : resized_range,
        cell_size : cell,
  } 
}

pub fn upload_from_bin(&mut self, mut bin : Vec<u8>) -> Result<(), Error> {


    let flash_memory_size = dsc_buffer_size_from_range(self.range.clone(), self.cell_size);

    //println!("bin.len() without_resize {}, flash_memory_size = {}", bin.len(), flash_memory_size);

    if(self.range.start != 0x0 && bin.len() > flash_memory_size){

      let start = self.range.start * self.cell_size;
      bin = bin.drain(start..).collect(); 
      //println!(" bin.drain()");
    }

    if(bin.len() != flash_memory_size) {

      bin.resize(flash_memory_size, 0xFF);
      //println!(" bin.resize()");
    }

   //println!("bin.len() {}, flash_memory_size = {}", bin.len(), flash_memory_size);

    self.upload(bin)?;

    Ok(())
}

pub fn upload_from_target(&mut self, mut read : Vec<Vec<u8>>) -> Result<(), Error> {


  let hex_buffer_size = dsc_buffer_size_from_range(self.range.clone(), self.cell_size);

  let mut unpack_read  = Vec::new();
  for one_line_vec in read.iter() {
      for mut one_byte in one_line_vec.iter()
      {
        unpack_read.push(* one_byte) 

      }  
  }
  println!("unpack_read.len() {}, flash_memory_size = {}", unpack_read.len(), hex_buffer_size);

  if(unpack_read.len() != hex_buffer_size) {

    return Err(Error::InternalError("Readed from Target not match with buffer size!".to_string()));

  }

  self.upload_packed(read)?;

  Ok(())
}

fn upload(&mut self, new_buff : Vec<u8>) -> Result<(), Error> {


    self.buffer.clear();
  
    let mut one_line_vec = Vec::new();
  
       for byte in new_buff.iter() {
  
         one_line_vec.push(*byte);
  
         if(one_line_vec.len() == 16) {
  
         self.buffer.push(one_line_vec.clone());
         one_line_vec.clear(); 
  
        }  
      } 
  
      Ok(())
  }


 fn upload_packed(&mut self, new_buff : Vec<Vec<u8>>) -> Result<(), Error> {


    self.buffer.clear();

    let mut one_line_vec = Vec::new();

    for line_input in new_buff.iter() {

        for byte in line_input.iter()
        {
            one_line_vec.push(*byte);

            if(one_line_vec.len() == 16) {

                self.buffer.push(one_line_vec.clone());
                one_line_vec.clear(); 
          
            }  
        }
   } 

      Ok(())
}


fn download_in_one(&self) -> Vec<u8> {


    
    let mut byte_vec = Vec::new();
    for one_line_vec in self.buffer.clone().iter()
    {
        for mut one_byte in one_line_vec.iter()
        {
            byte_vec.push(* one_byte) 

        }  
    }
    
   byte_vec

}

fn download_string(&self) -> Vec<Vec<String>> {



    let mut all_string_buffer = Vec::new();
    let mut one_line_string = Vec::new();
  
    for one_line_vec in self.buffer.iter() {

        for byte in one_line_vec.iter()
        {
           let in_string =   format!("{:02X?}", byte);
           one_line_string.push(in_string);  
              
           if(one_line_string.len() == 16)
           {
            all_string_buffer.push(one_line_string.clone());
            one_line_string.clear(); 
           }  
        }
    } 
  
    all_string_buffer.clone()

}

pub fn download_all_u8(&self) -> Vec<Vec<u8>> {


    
  self.buffer.clone()
  

}

pub fn flash_memory_size(&self) -> usize {
                                                                                       
  let flash_memory_size = dsc_buffer_size_from_range(self.range.clone(), self.cell_size);

  //println!(" flash_memory_size = {}", flash_memory_size);
                                                             
  flash_memory_size

}
}


pub fn dsc_buffer_size_from_range(range : Range<usize>, cell : usize) -> usize {
                                                                                       
  let buffer_size = (((range.end - range.start) + 1) * cell);

 // println!(" computed size = {}", buffer_size);
                                                             
  buffer_size

}


pub const BYTES_IN_ADDRESS : usize =  0x02; // cell_size

#[cfg(test)]
#[allow(arithmetic_overflow)]
mod tests {
    use super::*;


    fn resize_new_start(fill_byte : u8, mut buffer : MemoryBuffer, resize_range : Range<usize>) -> u8 {

        let mut buffer_new = buffer.resize(fill_byte, resize_range.clone(), 0x02);
        let mut buff: Vec<u8> = buffer_new.download_in_one();
        buff[(buffer_new.range.start / 16)]
  }

    fn resize_old_start(fill_byte : u8, mut buffer : MemoryBuffer, resize_range : Range<usize>) -> u8 {

      let mut buffer_new = buffer.resize(fill_byte, resize_range.clone(),0x02);
      let mut buff: Vec<u8> = buffer_new.download_in_one();
      buff[(buffer.range.start / 16)]
  }

    fn resize(fill_byte : u8, mut buffer : MemoryBuffer, resize_range : Range<usize>) -> MemoryBuffer {

      let buffer_new = buffer.resize(fill_byte, resize_range.clone(), 0x02);
      buffer_new
  }

    fn block_size(mut buffer : MemoryBuffer, start : usize, size : usize) -> usize {

      let block = buffer.download_target_block(start, size).unwrap();
      block.len()
  }

    fn block_range(mut buffer : MemoryBuffer, start : usize, size : usize) -> Range<usize> {

      let block = buffer.download_target_block(start, size).unwrap();
      let end_r = block.len();
      let start_r = (size * buffer.cell_size) - block.len();

      Range{ start: start_r, end: end_r }
  }

    fn all_flash(mut buffer : MemoryBuffer) -> usize {

    let flash = buffer.download_all_target();
    flash.len()

  }

    fn export_buffer(mut buffer : MemoryBuffer) -> usize {

    let export = buffer.download_export_fs(0xFF).unwrap();
    export.len()

  }

    fn build_empty_dsc(range : Range<usize>) -> MemoryBuffer {

    let empty_dsc_buffer = MemoryBuffer::init_empty(0xFF, range, 0x02);
    empty_dsc_buffer

  }


  
    #[test]
    fn empty_buffer() {
        
    assert_eq!(MemoryBuffer::init_empty(0xFF, Range { start: 0x0, end: 0x7FF }, 2).range.start, 0x0);
    assert_eq!(MemoryBuffer::init_empty(0xFF, Range { start: 0x0, end: 0x7FF }, 2).range.end, 0x7FF);
    assert_eq!(MemoryBuffer::init_empty(0xFF, Range { start: 0x0, end: 0x10  },  2).buffer.len(), 2);
    assert_eq!(MemoryBuffer::init_empty(0xFF, Range { start: 0x0, end: 0x30  },  2).buffer.len(), 6);
    assert_eq!(MemoryBuffer::init_empty(0xFF, Range { start: 0x0, end: 0x7FF }, 2).buffer.len(), ((((0x7FF - 0x0) + 1)  * BYTES_IN_ADDRESS) / HEX_LINE_LENGHT));
    
    }

    #[test]
    fn download_block() {
    
     assert_eq!(block_size(build_empty_dsc(Range { start: 0x0, end: 0x7FFF }), 0x4000, 0x100), 0x200);
     assert_eq!(block_size(build_empty_dsc(Range { start: 0x0, end: 0x7FFF }), 0x7FF0, 0x100), 32);
     assert_eq!(block_size(build_empty_dsc(Range { start: 0x4000, end: 0x7FFF }), 0x4000, 0x100), 0x200);
     assert_eq!(block_size(build_empty_dsc(Range { start: 0x4000, end: 0x7FFF }), 0x7FF0, 0x100), 32);
     assert_eq!(block_size(build_empty_dsc(Range { start: 0x0800, end: 0x1FFF }), 0x1FF0, 0x50), 32);
     assert_eq!(block_size(build_empty_dsc(Range { start: 0x0800, end: 0x1FFF }), 0x1000, 0x50), 0xA0);
     assert_eq!(block_range(build_empty_dsc( Range { start: 0x0800, end: 0x1FFF }), 0x1000, 0x50), Range { start: 0x0, end: 0xA0 });
      
    }

    #[test]
    #[should_panic]
    fn download_out_of_range() {

     block_size(build_empty_dsc(Range { start: 0x0800, end: 0x1FFF }), 0x000, 0x50);
     block_size(build_empty_dsc(Range { start: 0x0800, end: 0x1FFF }), 0x2000, 0x50);

    }

    #[test]
    fn download_and_verify() {

      let mut buff = build_empty_dsc(Range { start: 0x4000, end: 0x7FFF });

      let mut bin_start= vec![0xFF; 0x4000 * 2];
      let mut bin_4500 = vec![0xAA; 0x500 * 2];
      let mut bin_end  = vec![0xBB; 0x3B00 * 2];
      
      let mut bin = vec![];
      bin.append(&mut bin_start);
      bin.append(&mut bin_4500);
      bin.append(&mut bin_end);

      buff.upload_from_bin(bin).unwrap();

      let mut read_block_size = 0x100;
      let dump_from_target = vec![0xAA; read_block_size];
      let block = buff.download_target_block(0x4000, (read_block_size / 2)).unwrap();
      assert_eq!(block, dump_from_target);
   
      read_block_size = 0x21;
      let block = buff.download_target_block(0x4500, (read_block_size)).unwrap();
      let dump_from_target = vec![0xBB; read_block_size * 2];
      assert_eq!(block.len(), dump_from_target.len());
      assert_eq!(block, dump_from_target);
            
      read_block_size = 0x63;
      let block = buff.download_target_block(0x4500, (read_block_size)).unwrap();
      let dump_from_target = vec![0xBB; block.len()];
      assert_eq!(block.len(), dump_from_target.len());
      assert_eq!(block, dump_from_target);


    }

    #[test]
    fn all_flash_size() {

      assert_eq!(all_flash(build_empty_dsc(Range { start: 0x00, end: 0x7FFF })), 64 * 1024);
      assert_eq!(all_flash(build_empty_dsc(Range { start: 0x4000, end: 0x7FFF })), 32 * 1024);
      assert_eq!(all_flash(build_empty_dsc(Range { start: 0x0, end: 0x1FFF })), 16 * 1024);
      assert_eq!(all_flash(build_empty_dsc(Range { start: 0x800, end: 0x1FFF })), 12 * 1024);

    }

    #[test]
    fn export_full_size() {

      assert_eq!(export_buffer(build_empty_dsc(Range { start: 0x00, end: 0x7FFF })), 64 * 1024);
      assert_eq!(export_buffer(build_empty_dsc(Range { start: 0x4000, end: 0x7FFF })), 64 * 1024);
      assert_eq!(export_buffer(build_empty_dsc(Range { start: 0x4000, end: 0x7FFF })), 64 * 1024);
      assert_eq!(export_buffer(build_empty_dsc(Range { start: 0x0, end: 0x1FFF })), 16 * 1024);
      assert_eq!(export_buffer(build_empty_dsc(Range { start: 0x800, end: 0x1FFF })), 16 * 1024);


    }

    #[test]
    fn upload_bin() {

      let mut buff = build_empty_dsc(Range { start: 0x00, end: 0x7FFF });
      let mut bin : Vec<u8> = vec![0xFF; 0xFFFF];
      buff.upload_from_bin(bin).unwrap();
      assert_eq!(buff.buffer.len(), 4096);               // HexLines 4096 = real_len() / 16
      assert_eq!(buff.buffer.len() * 16, 64 * 1024);   

      buff = build_empty_dsc(Range { start: 0x4000, end: 0x7FFF });
      let mut bin2 = vec![0xFF; 0x1000];
      buff.upload_from_bin(bin2).unwrap();
      assert_eq!(buff.buffer.len(), 2048);               
      assert_eq!(buff.buffer.len() * 16, 32 * 1024);       

      buff = build_empty_dsc(Range { start: 0x4000, end: 0x7FFF });
      let mut bin3 = vec![0xFF; 0x1FFFF];
      buff.upload_from_bin(bin3).unwrap();
      assert_eq!(buff.buffer.len(), 2048);               
      assert_eq!(buff.buffer.len() * 16, 32 * 1024);   

      
      buff = build_empty_dsc(Range { start: 0x800, end: 0x1FFF });
      let mut bin4 = vec![0xFF; 0x32];
      buff.upload_from_bin(bin4).unwrap();
      assert_eq!(buff.buffer.len(), 768);               
      assert_eq!(buff.buffer.len() * 16, 12 * 1024);   
    }


    #[test]
    fn upload_from_target_dump() {

      let mut buff = build_empty_dsc(Range { start: 0x00, end: 0x7FFF });
      let test_block_size = 0x100;
      let mut number_of_read_pack = 0x10000 / test_block_size; // We work with pre-packed (Vec<Vec<u8>>)
      let mut dump = vec![vec![0xFF; test_block_size]; number_of_read_pack];
      buff.upload_from_target(dump).unwrap();
      assert_eq!(buff.buffer.len(), 4096);               // HexLines 4096 = real_len() / 16
      assert_eq!(buff.buffer.len() * 16, 64 * 1024);     

      buff = build_empty_dsc(Range { start: 0x800, end: 0x1FFF });
      number_of_read_pack = 0x20000 / test_block_size;
      let mut dump2 = vec![vec![0xFF; test_block_size]; number_of_read_pack];
      buff.upload_from_target(dump2).unwrap();
      assert_eq!(buff.buffer.len(), 768);               
      assert_eq!(buff.buffer.len() * 16, 12 * 1024);

      buff = build_empty_dsc(Range { start: 0x00, end: 0x7FF });
      number_of_read_pack = 0x800 / test_block_size;
      let mut dump2 = vec![vec![0xFF; test_block_size]; number_of_read_pack];
      buff.upload_from_target(dump2).unwrap();
      assert_eq!(buff.buffer.len(), 256);               
      assert_eq!(buff.buffer.len() * 16, 4 * 1024);        
    }

    #[test]
    #[should_panic]
    fn out_of_range_upload_from_target() {

      let mut buff = build_empty_dsc(Range { start: 0x00, end: 0x7FFF });
      let test_block_size = 0x100;
      let mut number_of_read_pack = 0x20000 / test_block_size; // We work with pre-packed (Vec<Vec<u8>>)
      let mut dump = vec![vec![0xFF; test_block_size]; number_of_read_pack];
      buff.upload_from_target(dump).unwrap();
    }

    #[test]
    #[should_panic]
    fn resize_start() {
      
      assert_eq!(resize(0xAA, build_empty_dsc(Range { start: 0x0, end: 0x7FFF }), Range { start: 0x4000, end: 0x7FFF }).buffer.len(), (((0x7FFF - 0x4000) + 1) / 16));
      assert_eq!(resize(0xAA, build_empty_dsc(Range { start: 0x4000, end: 0x7FFF }), Range { start: 0x0, end: 0x7FFF }).buffer.len(), (((0x7FFF - 0x00) + 1) / 16));

    }

}


