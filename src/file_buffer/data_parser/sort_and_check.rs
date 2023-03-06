use super::*;

impl ParsedData {
    pub fn sort_and_check(&mut self) -> Result<(), Error> {
        
        // check data exists
        if self.data_vec.len() == 0 { 
            return Err(Error::DataParserError("No Input Data".to_string())) }
        
        // sort data
        let mut sorted = 1;
        while self.data_vec.len() > sorted {
            for i in 1..self.data_vec.len(){
                if self.data_vec[i].address < self.data_vec[i - 1].address {
                    let temp_data_block = self.data_vec[i].clone();
                    self.data_vec[i] = self.data_vec[i - 1].clone();
                    self.data_vec[i - 1] = temp_data_block;
                    break
                } else {
                    sorted += 1;
                }
            }
        }

        // check data is not overlapping
        for i in 1..self.data_vec.len(){
            let address = self.data_vec[i].address;
            let end_of_previous_block =
               self.data_vec[i - 1].address + ((self.data_vec[i - 1].data_blob.len() as u32) / self.word_length as u32);
            if address < end_of_previous_block {
                return Err(Error::DataParserError("Data is Overlapping".to_string())) }
        }
        
        Ok(())  
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        let mut unsorted: ParsedData = ParsedData::default();
        let mut sorted: ParsedData = ParsedData::default();
        unsorted.word_length = 2;
        sorted.word_length = 2;
        let db1: DataBlock = DataBlock { address: 0, data_blob: vec![ 0,  1,  2,  3,  4,  5,  6,  7], };
        let db2: DataBlock = DataBlock { address: 4, data_blob: vec![ 8,  9, 10, 11, 12, 13, 14, 15], };
        let db3: DataBlock = DataBlock { address: 8, data_blob: vec![16, 17, 18, 19, 20, 21, 22, 23], };
        unsorted.data_vec = vec![db3.clone(), db2.clone(), db1.clone()];
        sorted.data_vec   = vec![db1.clone(), db2.clone(), db3.clone()];

        let mut sort_test = unsorted.clone();
        sort_test.sort_and_check().unwrap();  
        assert_eq!(sorted, sort_test);

        let mut sort_test_2 = sorted.clone();
        sort_test_2.sort_and_check().unwrap();  
        assert_eq!(sorted, sort_test_2);
    }    
    
    #[test]
    fn check_overlapping() {
        let mut sorted: ParsedData = ParsedData::default();
        sorted.word_length = 1;
        let db1: DataBlock = DataBlock { address: 0, data_blob: vec![ 0,  1,  2,  3,  4,  5,  6,  7], };
        let db2: DataBlock = DataBlock { address: 4, data_blob: vec![ 8,  9, 10, 11, 12, 13, 14, 15], };
        let db3: DataBlock = DataBlock { address: 8, data_blob: vec![16, 17, 18, 19, 20, 21, 22, 23], };
        sorted.data_vec   = vec![db1.clone(), db2.clone(), db3.clone()];

        let mut test = sorted.clone();
        let result = test.sort_and_check();  
        assert_eq!(result.is_err() && result.unwrap_err() == Error::DataParserError("Data is Overlapping".to_string()), true);
    } 
 
    #[test]
    fn no_data_error() {
        let mut test: ParsedData = ParsedData::default();
        let result = test.sort_and_check();  
        assert_eq!(result.is_err() && result.unwrap_err() == Error::DataParserError("No Input Data".to_string()), true);
    } 
}