use domain::oidc::client;
use domain::oidc::enums;

lazy_static! {
    static ref CLIENTS: Vec<client::Client> = vec!(
        client::Client {
            client_id: String::from("foobar"),
            client_secret: Some(String::from("password")),
            client_type: enums::ClientType::Confidential,
            redirect_uris: vec!()
        },
        client::Client {
            client_id: String::from("foobar-android"),
            client_secret: None,
            client_type: enums::ClientType::Public,
            redirect_uris: vec!(),
        }
    );
}

pub fn authenticate_client(client: &client::Client, client_secret: Option<&String>) -> bool {
    match client.client_type {
        enums::ClientType::Public => { true }
        enums::ClientType::Confidential => {
            client.client_secret.is_some()
                && client_secret.is_some()
                && client.client_secret.clone().unwrap() == *client_secret.clone().unwrap()
        }
    }
}

pub fn get_client_by_id(client_id: &String) -> Option<&'static client::Client> {
    CLIENTS.iter().find(|client| client.client_id == *client_id)
}