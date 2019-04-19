use url::{Url};
use domain::oidc::enums::ClientType;

#[derive(Debug)]
pub struct Client {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub client_type: ClientType,
    pub redirect_uris: Vec<Url>
}