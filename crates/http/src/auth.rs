use crate::error::Result;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header};

use serde::{Deserialize, Serialize};

/// The claims of a JWT
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
}

impl Claims {
    /// Create a claim with a given subject
    /// The expiration time is set to 7 days from the moment of creation
    pub fn new(iss: String) -> Self {
        let now = std::time::SystemTime::now();
        let iat = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize;

        let token_life = std::time::Duration::from_secs(10 * 60);
        let exp = (now + token_life)
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        Self { exp, iat, iss }
    }

    /// Encode the claims into a JWT string
    pub async fn encode(&self, key: &EncodingKey) -> Result<String> {
        let header = Header::new(Algorithm::RS256);
        let token = encode(&header, self, key)?;

        Ok(token)
    }

    pub async fn decode(token: &str, key: &DecodingKey) -> Result<Self> {
        let validation = jsonwebtoken::Validation::new(Algorithm::RS256);
        let claims = jsonwebtoken::decode::<Self>(token, key, &validation)?;

        Ok(claims.claims)
    }

    pub async fn decode_validation(
        token: &str,
        key: &DecodingKey,
        validation: &jsonwebtoken::Validation,
    ) -> Result<Self> {
        let claims = jsonwebtoken::decode::<Self>(token, key, validation)?;

        Ok(claims.claims)
    }
}
