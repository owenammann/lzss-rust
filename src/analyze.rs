use std::fs;
use std::io;
use std::io::prelude::*;

pub fn analyze() -> io::Result<()> {
        // Open the encoded file
        let encoded_file = fs::File::open("encoded.bin")?;
        let encoded_size = encoded_file.metadata()?.len();
    
        // Open the decoded file
        let decoded_file = fs::File::open("decoded.bin")?;
        let decoded_size = decoded_file.metadata()?.len();
    
        // Calculate the ratio
        let ratio = encoded_size as f64 / decoded_size as f64;
    
        // Write the ratio to analysis.txt
        let mut analysis_file = fs::File::create("analysis.txt")?;
        write!(analysis_file, "Decoded size to Encoded size ratio: {:.2}", ratio)?;
    
        Ok(())
}