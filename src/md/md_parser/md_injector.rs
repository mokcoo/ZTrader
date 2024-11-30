use std::{error::Error, fs::{self, File}, io::{BufRead, BufReader}};



pub fn parse_data_to_md(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // Process each line here
    }
    
    
    Ok(())
}