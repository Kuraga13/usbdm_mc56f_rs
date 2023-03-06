use super::*;

impl ParsedData {
    pub fn to_bin(&self) -> Result<Vec<u8>, Error> {
        if self.data_vec.len() == 0 { return Err(Error::DataParserError("No Input Data".to_string())) } // check data exists
        if self.valid == false { return Err(Error::DataParserError("Data not valid".to_string())) } // check data is valid
        
        let mut output: Vec<u8> = vec![];
        let mut next_address: u32 = 0;
        for i in 0..self.data_vec.len(){
            if self.data_vec[i].address > next_address{
                output.append(&mut vec![0xFF; ((self.data_vec[i].address - next_address) as usize * self.word_length as usize)]);
            } 
            output.append(&mut self.data_vec[i].data_blob.clone());
            next_address = output.len() as u32 / self.word_length as u32;
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        let mut test: ParsedData = ParsedData::default();
        test.valid = true;
        test.word_length = 2;
        let db1: DataBlock = DataBlock { address:  0, data_blob: vec![ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07], };
        let db2: DataBlock = DataBlock { address:  4, data_blob: vec![ 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F], };
        let db3: DataBlock = DataBlock { address: 12, data_blob: vec![ 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17], };
        test.data_vec   = vec![db1.clone(), db2.clone(), db3.clone()];
        let test_output: Vec<u8> = test.to_bin().unwrap();
        let expected_output: Vec<u8> = vec![ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 
                                             0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                                             0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17];
        assert_eq!(test_output, expected_output); // test multiple data blocks

        test.data_vec   = vec![db1.clone()];
        let test_output: Vec<u8> = test.to_bin().unwrap();
        let expected_output: Vec<u8> = vec![ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07 ];
        assert_eq!(test_output, expected_output); // test one data block
    }

    #[test]
    fn no_input_data() {
        let mut test: ParsedData = ParsedData::default();
        let output = test.to_bin();
        assert_eq!(output.is_err() && output.unwrap_err() == Error::DataParserError("No Input Data".to_string()), true);
    }

    #[test]
    fn data_not_valid() {
        let mut test: ParsedData = ParsedData::default();
        test.data_vec.push(DataBlock::default());
        let output = test.to_bin();
        assert_eq!(output.is_err() && output.unwrap_err() == Error::DataParserError("Data not valid".to_string()), true);
    }

}