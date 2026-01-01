use std::io;

pub fn is_valid_md5_hash(hash: &str) -> bool {
    // MD5 hashes are exactly 32 hexadecimal characters
    if hash.len() != 32 {
        return false;
    }
    
    // Check if all characters are valid hexadecimal (0-9, a-f, A-F)
    hash.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn get_yes_no_input(prompt: &str) -> bool {
    loop {
        println!("{} (yes/no):", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();
        
        match input.as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please enter 'yes' or 'no'."),
        }
    }
}

