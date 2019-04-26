use std::collections::HashMap;
use domain::requests::token_request::TokenRequest;
use domain::oidc::token::{JsonWebToken, SignedJsonWebToken, KeyId};
use domain::oidc::enums;

pub fn token(request: &TokenRequest) -> JsonWebToken {
    JsonWebToken {
        algorithm: enums::Algorithm::RS256,
        claims: HashMap::new(),
        key_id: Some(KeyId(String::from("123")))
    }
}

pub fn sign(token: JsonWebToken) -> SignedJsonWebToken {
    SignedJsonWebToken(String::from("oh no"))
}
