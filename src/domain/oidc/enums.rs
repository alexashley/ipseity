#[derive(Debug)]
pub enum ClientType {
    Public,
    Confidential
}

#[derive(Debug)]
pub enum GrantType {
    AuthorizationCode,
    ClientCredentials,
    Hybrid,
    Implicit,
    Refresh,
    ResourceOwnerCredentials
}

#[derive(Debug)]
pub enum Algorithm {
    RS256
}


