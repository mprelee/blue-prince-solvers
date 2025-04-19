use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use blue_prince_solvers::core_cipher;

fn main() -> io::Result<()> {
    // Open the vault box file
    let file_path = Path::new("data/vault_box_53.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    // Process each line
    for line in reader.lines() {
        let line = line?;
        let words: Vec<&str> = line.split_whitespace().collect();
        
        // Process each word in the line
        let mut result_word = String::new();
        for word in words {
            // Convert to character
            let character = core_cipher::numeric_core::numeric_core_word(word).unwrap_or_else(|| {
                eprintln!("Error converting word to character: {}", word);
                '?'
            });
            // Add to result word
            result_word.push(character);
        }
        
        // Print the result word
        println!("{}", result_word);
    }
    
    Ok(())
} 