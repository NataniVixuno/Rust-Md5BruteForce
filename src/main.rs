mod format_helpers;
mod cracking_helpers;

use std::time::{Instant, Duration};
use std::io;
use std::thread;
use std::sync::{Arc, Mutex, mpsc, atomic::{AtomicBool, Ordering}};
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
            result = brute_force_md5_multithreaded(charset, length, &hash);
            if result.is_some() {
                break;
            }
        }
        result
    } else {
        // Only check max_length
        brute_force_md5_multithreaded(charset, max_length, &hash)
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

fn brute_force_md5_multithreaded(charset: &str, length: usize, hash: &str) -> Option<String> {
    let charset_vec: Vec<char> = charset.chars().collect();
    
    // Shared state for cancellation and attempts tracking
    let found_flag = Arc::new(AtomicBool::new(false));
    let total_attempts = Arc::new(Mutex::new(0u64));
    let start_time = Arc::new(Mutex::new(Instant::now()));
    let last_update = Arc::new(Mutex::new(Instant::now()));
    
    // Channel to receive results
    let (tx, rx) = mpsc::channel();
    
    // Spawn one thread per character in the charset
    let mut handles = Vec::new();
    
    for (char_idx, &first_char) in charset_vec.iter().enumerate() {
        let charset_clone = charset.to_string();
        let hash_clone = hash.to_string();
        let found_flag_clone = Arc::clone(&found_flag);
        let total_attempts_clone = Arc::clone(&total_attempts);
        let start_time_clone = Arc::clone(&start_time);
        let last_update_clone = Arc::clone(&last_update);
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            brute_force_md5_single_thread(
                &charset_clone,
                length,
                &hash_clone,
                first_char,
                char_idx,
                found_flag_clone,
                total_attempts_clone,
                start_time_clone,
                last_update_clone,
                tx_clone,
            );
        });
        
        handles.push(handle);
    }
    
    // Drop the original sender - channel will close when all thread senders are dropped
    drop(tx);
    
    // Wait for a result or all threads to finish
    let result = loop {
        // Try to receive a result
        match rx.try_recv() {
            Ok(password) => {
                found_flag.store(true, Ordering::Relaxed);
                break Some(password);
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                // All threads finished and all senders dropped, no password found
                break None;
            }
            Err(mpsc::TryRecvError::Empty) => {
                // No result yet, check if all threads finished
                let all_finished = handles.iter().all(|h| h.is_finished());
                if all_finished {
                    // All threads done, wait a tiny bit for senders to be dropped, then check one more time
                    thread::sleep(Duration::from_millis(1));
                    match rx.try_recv() {
                        Ok(password) => break Some(password),
                        _ => break None, // Channel disconnected or still empty
                    }
                }
                // Brief sleep to avoid busy-waiting
                thread::sleep(Duration::from_millis(10));
            }
        }
    };
    
    // Signal all threads to stop (in case one found it)
    found_flag.store(true, Ordering::Relaxed);
    
    // Wait for all threads to finish
    for handle in handles {
        let _ = handle.join();
    }
    
    result
}

fn brute_force_md5_single_thread(
    charset: &str,
    length: usize,
    hash: &str,
    first_char: char,
    char_idx: usize,
    found_flag: Arc<AtomicBool>,
    total_attempts: Arc<Mutex<u64>>,
    start_time: Arc<Mutex<Instant>>,
    last_update: Arc<Mutex<Instant>>,
    result_tx: mpsc::Sender<String>,
) {
    const UPDATE_INTERVAL: Duration = Duration::from_secs(1);
    
    // If length is 1, just check the single character
    if length == 1 {
        let password = first_char.to_string();
        let hashed = format!("{:x}", md5::compute(password.as_bytes()));
        
        {
            let mut attempts = total_attempts.lock().unwrap();
            *attempts += 1;
        }
        
        if hashed == hash && !found_flag.load(Ordering::Relaxed) {
            found_flag.store(true, Ordering::Relaxed);
            let _ = result_tx.send(password);
            return;
        }
        return;
    }
    
    // For length > 1, fix the first character and check remaining positions
    let remaining_length = length - 1;
    let mut current: Vec<usize> = vec![0; remaining_length];
    let charset_len = charset.len();
    let mut local_attempts = 0u64;
    
    loop {
        // Check if another thread found the password
        if found_flag.load(Ordering::Relaxed) {
            // Add remaining attempts before exiting
            let mut attempts = total_attempts.lock().unwrap();
            *attempts += local_attempts;
            return;
        }
        
        // Build password with fixed first character
        let mut password = first_char.to_string();
        password.extend(current.iter().map(|&idx| charset.chars().nth(idx).unwrap()));
        
        let hashed = format!("{:x}", md5::compute(password.as_bytes()));
        local_attempts += 1;
        
        // Update global attempts counter periodically
        if local_attempts % 1000 == 0 {
            let mut attempts = total_attempts.lock().unwrap();
            *attempts += local_attempts;
            local_attempts = 0;
            
            // Print update every second
            let now = Instant::now();
            let mut last_update_guard = last_update.lock().unwrap();
            if now.duration_since(*last_update_guard) >= UPDATE_INTERVAL {
                let start_time_guard = start_time.lock().unwrap();
                let elapsed = start_time_guard.elapsed();
                let attempts_total = *attempts;
                let hashes_per_second = attempts_total as f64 / elapsed.as_secs_f64();
                println!(
                    "[Update] Length {} | Thread {} ({}) | Attempts: {} | Current: {} | Speed: {} hashes/sec | Elapsed: {:.2}s",
                    length,
                    char_idx,
                    first_char,
                    format_number(attempts_total),
                    password,
                    format_float(hashes_per_second),
                    elapsed.as_secs_f64()
                );
                *last_update_guard = now;
            }
        }
        
        if hashed == hash && !found_flag.load(Ordering::Relaxed) {
            // Add remaining attempts before returning
            {
                let mut attempts = total_attempts.lock().unwrap();
                *attempts += local_attempts;
            }
            found_flag.store(true, Ordering::Relaxed);
            let _ = result_tx.send(password);
            return;
        }
        
        // Increment to next combination
        let mut index = remaining_length - 1;
        loop {
            if current[index] < charset_len - 1 {
                current[index] += 1;
                break;
            } else {
                current[index] = 0;
                if index == 0 {
                    // Finalize remaining local attempts before returning
                    let mut attempts = total_attempts.lock().unwrap();
                    *attempts += local_attempts;
                    return;
                }
                index -= 1;
            }
        }
    }
}

fn count_combinations(charset: &str, max_length: usize) -> u64 {
    (charset.len() as u64).pow(max_length as u32)
}
