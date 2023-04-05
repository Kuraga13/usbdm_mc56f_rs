use core::ops::Range;
use crate::errors::Error;

pub const HEX_LINE_LENGHT : usize =  0x10;


pub struct MemoryBuffer {

    buffer     : Vec<Vec<u8>>,
}


impl MemoryBuffer
{
    pub fn init_empty_buffer(programm_segment_range : Range<usize>) -> Self {
  
     let number_of_hex_lines = ((programm_segment_range.end - programm_segment_range.start) + 1) / HEX_LINE_LENGHT;
        
       Self {
        
         buffer : vec![vec![0xFF; HEX_LINE_LENGHT]; number_of_hex_lines],
  } 
}

 pub fn upload_packed(&mut self, new_buff : Vec<Vec<u8>>) -> Result<(), Error> {


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

 pub fn upload(&mut self, new_buff : Vec<u8>) -> Result<(), Error> {


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

 pub fn download_in_one(&mut self) -> Vec<u8> {


    
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

pub fn download_string(&self) -> Vec<Vec<String>> {



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


}