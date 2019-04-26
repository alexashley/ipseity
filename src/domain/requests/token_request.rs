use domain::oidc::enums::GrantType;
use domain::oidc::client::Client;

#[derive(Debug)]
pub struct TokenRequest {
    pub grant_type: GrantType,
    pub client: Option<Box<Client>>,
    pub authorization_code: Option<String>
}
