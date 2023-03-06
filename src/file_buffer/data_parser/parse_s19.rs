use super::*;

impl ParsedData {

    pub fn parse_s19(input: Vec<u8>) -> Result<ParsedData, Error> {
        let mut parsed_data: ParsedData = ParsedData::default();
        if input.len() == 0 { return Err(Error::DataParserError("No Input Data".to_string())) }
        let mut data_strings: Vec<Vec<u8>> = split_strings(input);
        check_remove_s(&mut data_strings);
        if data_strings.len() == 0 { return Err(Error::DataParserError("No Valid Data".to_string())) }
        hex_string_to_byte(&mut data_strings);
        verify_remove_checksum(&mut data_strings)?;
        parse_strings(&mut data_strings, &mut parsed_data)?;
        parsed_data.sort_and_check()?;
        parsed_data.valid = true;
        Ok(parsed_data)  
    }
}

fn parse_strings(data: &mut Vec<Vec<u8>>, output: &mut ParsedData) -> Result<(), Error> {
    let mut data_block = DataBlock::default();
    let mut string_number: u32 = 1;
    let mut next_address: u32 = 0;
    
    for n in 0..data.len() {
        let mut address_length: u8 = 0;
        let mut address: u32 = 0;
        let mut string_data: Vec<u8> = vec![];
        parse_one_string(&mut data[n], &mut address_length, &mut address, &mut string_data)?;
        if address_length != 0 {
            if string_number == 1 {
                data_block.address = address;
                data_block.data_blob.append(&mut string_data); }
            else if string_number == 2 {
                let expected_bytes_in_previous_string: u32 = address - data_block.address;
                let actual_bytes_in_previous_string: u32 = data_block.data_blob.len() as u32;
                output.word_length = (actual_bytes_in_previous_string / expected_bytes_in_previous_string) as u8;
                next_address = address + (string_data.len() as u32) / (output.word_length as u32);
                data_block.data_blob.append(&mut string_data); } 
            else if string_number > 2 {
                if address == next_address {
                    next_address = address + (string_data.len() as u32) / (output.word_length as u32);
                    data_block.data_blob.append(&mut string_data);
                } else {
                    output.data_vec.push(data_block);
                    data_block = DataBlock::default();
                    data_block.address = address;
                    next_address = address + (string_data.len() as u32) / (output.word_length as u32);
                    data_block.data_blob.append(&mut string_data);
                }
            }
            string_number += 1;
        }

    }
    output.data_vec.push(data_block);
    Ok(())
}

fn parse_one_string(data_string: &mut Vec<u8>, address_length: &mut u8, address: &mut u32, data: &mut Vec<u8>) -> Result<(), Error> {
    if data_string.len() < 1 { return Err(Error::DataParserError("Parse Address Error".to_string())) }
    *address_length = match data_string[0] {
        1 => 2,
        2 => 3,
        3 => 4,
        _ => 0,
    };

    if *address_length == 0 { return Ok(()) }
    if data_string.len() < (3 + *address_length).into() { return Err(Error::DataParserError("Parse Address Error".to_string())) }
    if *address_length == 2 {
        *address = ((data_string[2] as u32) <<  8) +   data_string[3] as u32; 
    } else if *address_length == 3 {
        *address = ((data_string[2] as u32) << 16) + ((data_string[3] as u32) <<  8) +  data_string[4] as u32;
    } else if *address_length == 4 {
        *address = ((data_string[2] as u32) << 24) + ((data_string[3] as u32) << 16) + ((data_string[4] as u32) << 8) + data_string[5] as u32;
    }
    data.append(&mut data_string.drain(((2 + *address_length) as usize)..data_string.len()).collect());

    Ok(())
}

fn verify_remove_checksum(data: &mut Vec<Vec<u8>>) -> Result<(), Error> {
    for n in 0..data.len() {
        let data_length = data[n][1];
        if data[n].len() < (data_length + 2) as usize {
            return Err(Error::DataParserError("Checksum Error".to_string()));
        }
        let mut checksum: u32 = 0;
        for i in 1..(1 + data_length) {
            checksum += (data[n][i as usize]) as u32; // & 0xFF;
        }
        let checksum_u8: u8 = checksum as u8;
        if data[n][(data_length + 1) as usize] != !checksum_u8 {
            return Err(Error::DataParserError("Checksum Error".to_string()));
        }
        data[n].drain(data_length as usize + 1..);
    }
    Ok(())
}

fn hex_string_to_byte(data: &mut Vec<Vec<u8>>){
    for string in data.iter_mut() {
        let mut local: Vec<u8> = vec![];
        local.append(string);
        string.push(hex_to_byte(0, local[0]));

        let mut i = 1;
        while local.len() > i + 1 {
            string.push(hex_to_byte(local[i], local[i+1]));
            i += 2;
        }     
    }
}

fn hex_to_byte(a: u8, b: u8) -> u8 {
    let mut byte = vec![a, b];
    for x in byte.iter_mut() {
        if      *x >= b'0' && *x <= b'9' { *x -= b'0'; }
        else if *x >= b'a' && *x <= b'f' { *x -= b'a' - 10; }
        else if *x >= b'A' && *x <= b'F' { *x -= b'A' - 10;}
    }
    (byte[0] << 4) + byte[1]
}

fn check_remove_s(data: &mut Vec<Vec<u8>>) {
    let mut i = 0;
    while i < data.len() {
        if data[i].len() == 0 {
            data.remove(i);
        } else if data[i][0] == b'S' || data[i][0] == b's' {
            data[i].remove(0);
            i += 1;
        } else {
            data[i].remove(0);
        }
    }
}

fn split_strings(data: Vec<u8>) -> Vec<Vec<u8>> {
    let mut output: Vec<Vec<u8>> = vec![];
    let mut temp: Vec<u8> = vec![];
    for &byte in data.iter(){
        if byte_is_valid(byte) { 
            temp.push(byte);   
        } else if (byte == 13 || byte == 10) && temp.len() != 0 { //13 and 10 are /cr and /lf
            output.push(temp.clone());
            temp.clear();
        }
    }
    output
}

fn byte_is_valid(x: u8) -> bool {
    if x >= b'A' && x <= b'F' { return true }
    if x >= b'a' && x <= b'f' { return true }
    if x >= b'0' && x <= b'9' { return true }
    if x == b'S' || x == b's' { return true }
    return false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_data_error() {
        let vec = ParsedData::parse_s19(vec![]);
        assert_eq!((vec.is_err() && vec.unwrap_err() == Error::DataParserError("No Input Data".to_string())), true);
    }

    #[test]
    fn s_remove_test() {
        let mut x: Vec<Vec<u8>> = vec![vec![83, 48, 48, 51, 48, 48, 48, 48, 70, 67], 
            vec![b's', 51, 50, 53, 48, 48, 48, 48, 48, 48, 48, 48, 53, 52], 
            vec![15, 15, 15],
            vec![b'S', 51, 50, 53, 48, 48, 48, 48, 48, 48, 49, 48, 53, 52]];
        check_remove_s(&mut x);
        let expected_result: Vec<Vec<u8>> = vec![vec![48, 48, 51, 48, 48, 48, 48, 70, 67], 
            vec![51, 50, 53, 48, 48, 48, 48, 48, 48, 48, 48, 53, 52], 
            vec![51, 50, 53, 48, 48, 48, 48, 48, 48, 49, 48, 53, 52]];
        
        assert_eq!(x, expected_result);
    }

    #[test]
    fn s_remove_no_data_test() {
        let mut x: Vec<Vec<u8>> = vec![vec![]];
        let mut y: Vec<Vec<u8>> = vec![];
        check_remove_s(&mut x);
        check_remove_s(&mut y);
        let expected_result: Vec<Vec<u8>> = vec![];
        assert_eq!(x, expected_result);
        assert_eq!(y, expected_result);
    }

    
}