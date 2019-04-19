use url::{Url};

#[derive(Debug)]
pub enum ClientType {
    Public,
    Confidential
}

#[derive(Debug)]
pub struct Client {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub client_type: ClientType,
    pub redirect_uris: Vec<Url>
}