use std::collections::HashMap;
use std::any::Any;

use domain::oidc::enums;

#[derive(Debug)]
pub struct SignedJsonWebToken(pub String);

#[derive(Debug)]
pub struct KeyId(pub String);

#[derive(Debug)]
pub struct JsonWebToken {
    pub algorithm: enums::Algorithm,
    // TODO: claims value should be something like std::any::Any or enum type of (str, int, list)?
    pub claims: HashMap<String, String>,
    pub key_id: Option<KeyId>
}
