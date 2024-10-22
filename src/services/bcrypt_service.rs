use bcrypt::{hash, verify, DEFAULT_COST};
use std::error::Error;

pub struct BcryptService;

impl BcryptService {
    pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    pub fn verify_password(password: &str, hashed: &str) -> Result<bool, Box<dyn Error>> {
        let is_valid = verify(password, hashed)?;
        Ok(is_valid)
    }
}
