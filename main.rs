use totp_rs::{Algorithm, TOTP, Secret};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io;

fn main() {
    println!("Rust Authenticator (TOTP Demo)");

    // Generate a secret (in real apps, store this securely per user)
    let secret = Secret::generate_secret();

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,          // 6-digit code
        1,          // step: 30 seconds default
        30,
        secret.to_bytes().unwrap(),
    ).unwrap();

    println!("\nYour secret key (save this!): {}", secret.to_encoded());

    loop {
        println!("\nChoose option:");
        println!("1. Generate OTP");
        println!("2. Verify OTP");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let otp = totp.generate_current().unwrap();
                println!("Current OTP: {}", otp);
            }

            "2" => {
                println!("Enter OTP:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let input = input.trim();

                let is_valid = totp.check_current(input).unwrap();

                if is_valid {
                    println!("Authentication SUCCESS");
                } else {
                    println!("Authentication FAILED");
                }
            }

            "3" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid option"),
        }
    }
}