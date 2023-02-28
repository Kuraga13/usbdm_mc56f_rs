pub fn to_bdm_s19_325(data: Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    output.append(&mut first_string());
    output.append(&mut body_compose(data));
    output  
} 

//pub fn from_bdm_s19_325() -> Vec<u8> {
//}

fn byte_to_hex(byte: u8) -> Vec<u8> {
    let mut output: Vec<u8> = vec![(byte & 0xF0) >> 4, byte & 0x0F];
    
    for x in output.iter_mut(){
        if *x >= 10 {*x += 65 - 10} else {*x += 48}}
    output
}

fn data_to_hex(data: Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];

    for x in data.iter(){
        output.append(&mut byte_to_hex(*x));}
    output 
}

fn end_of_string() -> Vec<u8> {
    vec![13, 10] // CR LF
}

fn first_string() ->  Vec<u8> {
    let mut vec: Vec<u8> = vec![b'S', b'0', b'0', b'3'];
    vec.append(&mut data_to_hex(vec![0x00, 0x00, 0xFC]));
    vec.append(&mut end_of_string());
    vec
}
fn address_to_hex(address: u32) -> Vec<u8> {
    let byte_address: Vec<u8> = vec![(address>>24) as u8, (address>>16) as u8, (address>>8) as u8, address as u8];
    data_to_hex(byte_address)
}

fn checksum(address: u32, data: Vec<u8>) -> u8 {
    let mut checksum: u32 = 0x25;
    checksum += ((address>>24) & 0xff) + ((address>>16) & 0xff) + ((address>>8) & 0xff) + (address & 0xff);
    for &byte in data.iter(){
        checksum = (checksum + byte as u32) & 0xFF;}
    !checksum as u8
}

fn string_compose(address: u32, data: Vec<u8>) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![b'S', b'3', b'2', b'5'];
    vec.append(&mut address_to_hex(address));
    vec.append(&mut data_to_hex(data.clone()));
    vec.append(&mut byte_to_hex(checksum(address, data)));
    vec.append(&mut end_of_string());
    vec
}

fn body_compose(global_data: Vec<u8>) -> Vec<u8> {
    let mut data = global_data.clone();
    let mut output: Vec<u8> = vec![];

    let mut address: u32 = 0;
    while data.len() > 0 {
        let mut block_size: u32 = data.len() as u32;
    
        if block_size > 0x20 {
        block_size = 0x20; };

        output.append(&mut string_compose(address, data.drain(..(block_size as usize)).collect()));
        address += block_size / 2;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bdm_s19_325_test() {
        let mut data1: Vec<u8> = vec![0x54, 0xE1, 0x5D, 0x32, 0x54, 0xE1, 0x5D, 0x32, 0x54, 0xE2, 0x6C, 0x24, 0x54, 0xE2, 0x6C, 0x24, 0x54, 0xE2, 0x6C, 0x24, 0x54, 0xE2, 0x6C, 0x24, 0x54, 0xE2, 0x4A, 0x30, 0x54, 0xE2, 0x1B, 0x45];
        let mut data2: Vec<u8> = vec![0x54, 0xE2, 0x4E, 0x30, 0x54, 0xE2, 0x52, 0x30, 0x54, 0xE2, 0x56, 0x30, 0x54, 0xE2, 0x6C, 0x24, 0x54, 0xE2, 0x6C, 0x24, 0x54, 0xE2, 0x6C, 0x24, 0x54, 0xE2, 0x6C, 0x24, 0x54, 0xE2, 0x93, 0x51];
        data1.append(&mut data2);
        let output_vec = to_bdm_s19_325(data1);
        let mut output_string: String = "".to_string();
        for &x in output_vec.iter(){output_string += &{if x >= 33 && x <= 126 {x as char} else {'.'}}.to_string();}
        let mut test_output = "S0030000FC..".to_string();
        test_output += &"S3250000000054E15D3254E15D3254E26C2454E26C2454E26C2454E26C2454E24A3054E21B45F4..".to_string();
        test_output += &"S3250000001054E24E3054E2523054E2563054E26C2454E26C2454E26C2454E26C2454E2935170..".to_string();
        assert_eq!(output_string, test_output);
    }
}

