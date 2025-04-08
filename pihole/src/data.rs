use serde::{Deserialize, Serialize};

//------------------------------
// Authentication endpoint types
//------------------------------

#[derive(Debug, Serialize)]
pub struct AuthRequest {
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub session: AuthResponseSession,
    pub took: f64,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponseSession {
    pub valid: bool,
    pub totp: bool,
    pub sid: String,
    pub csrf: String,
    pub validity: u32,
    pub message: String,
}

//---------------------------------
// Group management endpoint types
//---------------------------------

#[derive(Debug, Deserialize)]
pub struct GroupsResponse {
    pub groups: Vec<GroupResult>,
    pub took: f64,
}

#[derive(Debug, Deserialize)]
pub struct GroupResult {
    pub name: String,
    pub comment: Option<String>,
    pub enabled: bool,
    pub id: u8,
    pub date_added: i64,
    pub date_modified: i64,
}

//----------------------------------
// Client management endpoint types
//----------------------------------

#[derive(Debug, Deserialize)]
pub struct ClientsResponse {
    pub clients: Vec<ClientResult>,
    pub took: f64,
}

#[derive(Debug, Deserialize)]
pub struct ClientResult {
    pub client: String,
    pub name: String,
    pub comment: String,
    pub groups: Vec<u8>,
    pub id: u8,
    pub date_added: i64,
    pub date_modified: i64,
    pub processed: Option<ProcessedResult>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessedResult {
    pub success: Vec<SuccessItem>,
    pub errors: Vec<ErrorItem>,
}

#[derive(Debug, Deserialize)]
pub struct SuccessItem {
    pub item: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorItem {
    pub item: String,
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct ClientRequest {
    pub comment: String,
    pub groups: Vec<u8>,
}

//------------------------------
// Actions endpoint types
//------------------------------

#[derive(Debug, Deserialize)]
pub struct RestartDnsResponse {
    pub status: String,
    pub took: f64,
}

//------------------------------
// Generic error type
//------------------------------
#[derive(Debug, Deserialize)]
pub struct PiHoleError {
    pub error: PiHoleErrorDetails,
    pub took: f64,
}

#[derive(Debug, Deserialize)]
pub struct PiHoleErrorDetails {
    pub key: String,
    pub message: String,
    pub hint: Option<String>,
}
