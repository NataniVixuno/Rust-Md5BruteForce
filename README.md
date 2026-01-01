Here is a more complete README for Rust-Md5BruteForce:

# Rust-Md5BruteForce

Rust-Md5BruteForce is an extremely fast MD5 hash cracking tool written in Rust. It utilizes a brute force algorithm to attempt all possible combinations of characters to recover passwords hashed with MD5.

Originally created by Cybrik - https://github.com/Crybik/Rust-Md5Forcer

## Features

- Lightning fast MD5 cracking capabilities powered by Rust's performance 
- Multi-threaded brute force algorithm for maximum CPU utilization
- Supports custom charsets for maximum flexibility
- Simple command line interface for ease of use
- Cracks hashes in seconds depending on password complexity

## Getting Started  

### Prerequisites

You'll need Rust installed on your system. I recommend using the latest stable version.

### Usage

1. Clone the repo: `git clone https://github.com/NataniVixuno/Rust-Md5BruteForce`
2. Compile: `cargo build`
3. Run the cracker: `cargo run --release`  
4. Enter the hash you wish to crack when prompted.
5. Enter the charset to use when prompted. Leave blank for default alphanumeric.
6. Enter max password length to try. More length means more combinations.
7. Wait for the cracking to finish! Cracked passwords will be printed.


## Performance

Performance will vary based on hardware.

In general, Rust-Md5BruteForce can crack 6 character alphanumeric MD5 hashes in seconds . More complex passwords take longer, but ultimately any MD5 hash can be reversed given enough time and computing power.

## Extending the Cracker

Rust-Md5BruteForce is designed to be easily extensible:

- Support additional hash types like SHA1 by adding new hash functions
- Add a more optimized brute force algorithm 
- Implement a hybrid attack using wordlists 
- Add GPU acceleration for even more speed

PRs with improvements and features are welcome!

## Disclaimer

This tool is provided for educational and ethical security research purposes only. Do not use it for illegal activity.

## Contact 

You can reach me at natani@techern.org
