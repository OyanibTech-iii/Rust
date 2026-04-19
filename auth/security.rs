pub struct Security;

impl Security {
    // Input validation layer (defense before logic)
    pub fn validate_code(code: &str) -> bool {
        // Must be exactly 6 digits
        if code.len() != 6 {
            return false;
        }

        // Must contain only numbers
        code.chars().all(|c| c.is_numeric())
    }
}