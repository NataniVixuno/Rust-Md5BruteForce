use std::time::{Instant, Duration};
use std::io;
use md5;

const DEFAULT_CHARSET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    println!("Originally written by Crybik");
    println!("GitHub: https://github.com/crybik\n");
    println!("Modified and maintained by Natani Vixuno");
    println!("Github: https://github.com/NataniVixuno\n");


    println!("What's the hash you want to crack?");
    let mut hash = String::new();
    io::stdin().read_line(&mut hash).expect("Failed to read input");
    let hash = hash.trim();

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

    let start_time = Instant::now();
    let found_password = brute_force_md5(charset, max_length, hash);

    let elapsed_time = start_time.elapsed();
    let total_combinations = count_combinations(charset, max_length);
    let hashes_per_second = total_combinations as f64 / elapsed_time.as_secs_f64();

    match found_password {
        Some(password) => println!("Password found: {}", password),
        None => println!("Couldn't find the password. Keep trying!"),
    }

    println!(
        "Cracking completed in {:.2} seconds. Speed: {:.2} hashes/sec",
        elapsed_time.as_secs_f64(),
        hashes_per_second
    );
}

fn brute_force_md5(charset: &str, max_length: usize, hash: &str) -> Option<String> {
    let mut current: Vec<usize> = vec![0; max_length];
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

        // Print update every 5 seconds
        if last_update.elapsed() >= UPDATE_INTERVAL {
            let elapsed = start_time.elapsed();
            let hashes_per_second = attempts as f64 / elapsed.as_secs_f64();
            println!(
                "[Update] Attempts: {} | Current: {} | Speed: {:.2} hashes/sec | Elapsed: {:.2}s",
                attempts,
                password,
                hashes_per_second,
                elapsed.as_secs_f64()
            );
            last_update = Instant::now();
        }

        let mut index = max_length - 1;
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
