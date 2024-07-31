use aho_corasick::AhoCorasick;
use std::fs::File;
use std::io::{self, Read};

fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let patterns = &["Knowledge", "gifted", "misery"];
    let path = "/home/sam/rust_playground/Rapid-regex/core-module/text.txt";
    
    // Read file contents
    let contents = match read_file_to_string(path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
            return Err(e);
        }
    };

    // Create AhoCorasick automaton
    let ac = AhoCorasick::new(patterns).expect("Failed to create AhoCorasick automaton");

    // Split contents into paragraphs
    let paragraphs: Vec<&str> = contents.split("\n\n").collect();

    // Find matches in each paragraph
    for (index, paragraph) in paragraphs.iter().enumerate() {
        let matches: Vec<_> = ac.find_iter(paragraph).collect();
        if !matches.is_empty() {
            println!("Match found in paragraph {}: Patterns found:", index + 1);
            for mat in matches {
                println!("- Pattern '{}' at position {}.", patterns[mat.pattern()], mat.start());
            }
            println!("---");
        }
    }
    
    Ok(())
}
