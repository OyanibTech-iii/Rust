mod auth;
mod models;
mod utils;

use auth::totp_service::TotpService;
use models::user_secret::UserSecret;

fn main() {
    println!("Secure Rust Authenticator System");

    // Create user secret (simulated user account)
    let user_secret = UserSecret::new();

    let service = TotpService::new(user_secret.clone());

    // Generate OTP
    let otp = service.generate_otp();
    println!("Generated OTP: {}", otp);

    // Verify OTP (demo)
    let result = service.verify(&otp);
    println!("Verification result: {}", result);
}