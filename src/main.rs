mod format_helpers;
mod cracking_helpers;

use std::time::{Instant, Duration};
use std::io;
use md5;
use format_helpers::{format_number, format_float};
use cracking_helpers::{is_valid_md5_hash, get_yes_no_input};

const DEFAULT_CHARSET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    println!("Originally written by Crybik");
    println!("GitHub: https://github.com/crybik\n");
    println!("Modified and maintained by Natani Vixuno");
    println!("Github: https://github.com/NataniVixuno\n");


    let hash = loop {
        println!("What's the hash you want to crack?");
        let mut hash_input = String::new();
        io::stdin().read_line(&mut hash_input).expect("Failed to read input");
        let hash = hash_input.trim();
        
        if is_valid_md5_hash(hash) {
            break hash.to_string();
        } else {
            println!("Invalid MD5 hash! MD5 hashes must be exactly 32 hexadecimal characters (0-9, a-f, A-F).");
            println!("Please try again.\n");
        }
    };

    println!("Enter the charset (leave blank for default alphanumeric characters):");
    let mut charset_input = String::new();
    io::stdin().read_line(&mut charset_input).expect("Failed to read input");
    let charset = if charset_input.trim().is_empty() {
        DEFAULT_CHARSET
    } else {
        charset_input.trim()
    };

    println!("How long should the password be at most?");
    let mut max_length_input = String::new();
    io::stdin().read_line(&mut max_length_input).expect("Failed to read input");
    let max_length: usize = max_length_input.trim().parse().unwrap_or(7);

    let check_all_lengths = get_yes_no_input("Should I check all password lengths from 1 to the maximum length?");

    let start_time = Instant::now();
    let found_password = if check_all_lengths {
        // Check all lengths from 1 to max_length
        let mut result = None;
        for length in 1..=max_length {
            println!("\nChecking passwords of length {}...", length);
            result = brute_force_md5(charset, length, &hash);
            if result.is_some() {
                break;
            }
        }
        result
    } else {
        // Only check max_length
        brute_force_md5(charset, max_length, &hash)
    };

    let elapsed_time = start_time.elapsed();
    let total_combinations = if check_all_lengths {
        // Sum combinations for all lengths from 1 to max_length
        (1..=max_length).map(|len| count_combinations(charset, len)).sum::<u64>()
    } else {
        count_combinations(charset, max_length)
    };
    let hashes_per_second = total_combinations as f64 / elapsed_time.as_secs_f64();

    match found_password {
        Some(password) => println!("Password found: {}", password),
        None => println!("Couldn't find the password. Keep trying!"),
    }

    println!(
        "Cracking completed in {:.2} seconds. Speed: {} hashes/sec",
        elapsed_time.as_secs_f64(),
        format_float(hashes_per_second)
    );
}

fn brute_force_md5(charset: &str, length: usize, hash: &str) -> Option<String> {
    let mut current: Vec<usize> = vec![0; length];
    let charset_len = charset.len();
    let mut attempts: u64 = 0;
    let start_time = Instant::now();
    let mut last_update = Instant::now();
    const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

    loop {
        let password: String = current.iter().map(|&idx| charset.chars().nth(idx).unwrap()).collect();
        let hashed = format!("{:x}", md5::compute(password.as_bytes()));
        attempts += 1;

        if hashed == hash {
            return Some(password);
        }

        // Print update every second
        if last_update.elapsed() >= UPDATE_INTERVAL {
            let elapsed = start_time.elapsed();
            let hashes_per_second = attempts as f64 / elapsed.as_secs_f64();
            println!(
                "[Update] Length {} | Attempts: {} | Current: {} | Speed: {} hashes/sec | Elapsed: {:.2}s",
                length,
                format_number(attempts),
                password,
                format_float(hashes_per_second),
                elapsed.as_secs_f64()
            );
            last_update = Instant::now();
        }

        let mut index = length - 1;
        loop {
            if current[index] < charset_len - 1 {
                current[index] += 1;
                break;
            } else {
                current[index] = 0;
                if index == 0 {
                    return None;
                }
                index -= 1;
            }
        }
    }
}

fn count_combinations(charset: &str, max_length: usize) -> u64 {
    (charset.len() as u64).pow(max_length as u32)
}
