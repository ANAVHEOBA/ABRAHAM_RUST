use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, // subject (user_id)
    exp: usize,  // expiration time
}

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        JwtService { secret }
    }

    pub fn create_token(&self, user_id: &str) -> String {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize
            + 60 * 60; // Token valid for 1 hour

        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_ref()))
            .unwrap()
    }

    pub fn validate_token(&self, token: &str) -> Result<String, String> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &DecodingKey::from_secret(self.secret.as_ref()), &validation);

        match token_data {
            Ok(data) => Ok(data.claims.sub),
            Err(_) => Err("Invalid token".to_string()),
        }
    }
}
