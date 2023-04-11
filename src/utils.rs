
///`vec_as_u32_be` decode Vec<u8> to u32
pub fn vec_as_u32_be(vec:  Vec<u8>) -> u32 {
    ((vec[0] as u32) << 24) +
    ((vec[1] as u32) << 16) +
    ((vec[2] as u32) <<  8) +
    ((vec[3] as u32) <<  0)
}

///`msh_lsh_vec_as_u32_be` decode Vec<u8> to u32
pub fn msh_lsh_vec_as_u32_be(lsh_vec:  Vec<u8>, msh_vec:  Vec<u8>) -> u32 {
  ((msh_vec[1] as u32) << 24) +
  ((msh_vec[0] as u32) << 16) +
  ((lsh_vec[1] as u32) <<  8) +
  ((lsh_vec[0] as u32) <<  0)
}

///`print_vec_memory` for debug memory read, sequnces etc., use for print small block in Vec<u8>
pub fn print_vec_memory(mem : &Vec<u8>) {
    
    let mut printed_vec = Vec::new();
    for byte in mem.iter() {
     let in_string = format!("{:02X}", byte);
     printed_vec.push(in_string);
      if(printed_vec.len() == 0x10) {
       for symbol in printed_vec.iter() { print!("{} ", symbol); }
     print!("\n");
     printed_vec.clear();
     }  
   } 
 }

 ///`print_vec_one_line` for debug memory read, sequnces etc., use for print small block in Vec<u8>
pub fn print_vec_one_line(mem : &Vec<u8>) {
     
    for byte in mem.iter() {
    let in_string = format!("{:02X}", byte);
    print!("{}", in_string); }
    print!("\n");
      
} 

pub fn print_id_code(core_id_code : &Vec<u8>, master_id_code : &Vec<u8>) {
    
  println!(" core_id_code :");
   
  for byte in core_id_code.iter()
  {
  let in_string = format!("{:02X}", byte);
  print!("{} ", in_string);
  }
 
  println!(" \n");
 
  println!(" master_id_code (in usbdm jtag-idcode) :");
  for byte in master_id_code.iter()
  {
 
  let in_string = format!("{:02X}", byte);
  print!("{} ", in_string);
 
  }
 
  println!(" \n");
    
 }