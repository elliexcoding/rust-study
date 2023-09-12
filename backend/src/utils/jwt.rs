use axum::http::StatusCode;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let now = now + chrono::Duration::hours(1);
    let exp = now.timestamp() as usize;

    let secret = EncodingKey::from_secret(dotenvy::var("JWT_SECRET").unwrap().as_ref());

    let claim = Claims {
        exp,
        iat,
    };
    let token = encode(&Header::default(), &claim, &secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    todo!()
}

pub fn is_valid() -> Result<bool, StatusCode> {
    todo!()
}