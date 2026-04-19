use rand::{RngCore, rngs::OsRng};

#[derive(Clone)]
pub struct UserSecret {
    // Secure raw bytes stored in memory
    secret: Vec<u8>,
}

impl UserSecret {
    // Generates cryptographically secure random secret
    pub fn new() -> Self {
        let mut buffer = vec![0u8; 20];
        OsRng.fill_bytes(&mut buffer);

        Self {
            secret: buffer,
        }
    }

    // Controlled access (prevents direct mutation)
    pub fn as_bytes(&self) -> &[u8] {
        &self.secret
    }
}