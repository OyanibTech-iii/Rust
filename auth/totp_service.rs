use totp_rs::{Algorithm, TOTP};
use crate::models::user_secret::UserSecret;
use crate::auth::security::Security;

pub struct TotpService {
    totp: TOTP,
}

impl TotpService {
    pub fn new(secret: UserSecret) -> Self {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.as_bytes().to_vec(),
        ).unwrap();

        Self { totp }
    }

    pub fn generate_otp(&self) -> String {
        self.totp.generate_current().unwrap()
    }

    pub fn verify(&self, code: &str) -> bool {
        Security::validate_code(code) && self.totp.check_current(code).unwrap()
    }
}