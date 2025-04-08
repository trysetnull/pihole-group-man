use crate::data::{
    AuthRequest, AuthResponse, ClientRequest, ClientResult, ClientsResponse, GroupsResponse,
    RestartDnsResponse,
};
use crate::error::PiHoleApiError;
use reqwest::Client;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct PiHoleV6Client {
    base_url: String,
    client: Arc<Client>,
    session: Arc<Mutex<SessionState>>,
}

#[derive(Debug, Default)]
struct SessionState {
    sid: Option<String>,
    csrf: Option<String>,
}

impl PiHoleV6Client {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Arc::new(Client::new()),
            session: Arc::new(Mutex::new(SessionState::default())),
        }
    }

    // Authentication endpoints
    pub async fn login(&mut self, password: &str) -> Result<(), PiHoleApiError> {
        let req = AuthRequest {
            password: password.to_string(),
        };
        let resp: AuthResponse = self
            .execute_api("api/auth", Some(&req), Method::Post, false)
            .await?;
        let mut session = self.session.lock().unwrap();
        session.sid = Some(resp.session.sid);
        session.csrf = Some(resp.session.csrf);
        Ok(())
    }

    pub async fn logout(&mut self) -> Result<(), PiHoleApiError> {
        self.execute_api::<(), ()>("api/auth", None, Method::Delete, true)
            .await?;

        let mut session = self.session.lock().unwrap();
        session.sid = None;
        session.csrf = None;
        Ok(())
    }

    // Group management
    pub async fn get_groups(&self) -> Result<GroupsResponse, PiHoleApiError> {
        self.execute_api("api/groups", None::<&()>, Method::Get, true)
            .await
    }

    pub async fn get_group(&self, group_name: &str) -> Result<GroupsResponse, PiHoleApiError> {
        let endpoint = format!("api/groups/{group_name}");
        self.execute_api(&endpoint, None::<&()>, Method::Get, true)
            .await
    }

    // Client management
    pub async fn get_clients(&self) -> Result<ClientsResponse, PiHoleApiError> {
        self.execute_api("api/clients", None::<&()>, Method::Get, true)
            .await
    }

    pub async fn get_client(&self, client_id: &str) -> Result<ClientResult, PiHoleApiError> {
        let endpoint = format!("api/clients/{client_id}");
        self.execute_api(&endpoint, None::<&()>, Method::Get, true)
            .await
    }

    pub async fn update_client(
        &self,
        client_id: &str,
        comment: String,
        groups: Vec<u8>,
    ) -> Result<ClientResult, PiHoleApiError> {
        let endpoint = format!("api/clients/{client_id}");
        let req = ClientRequest { comment, groups };
        self.execute_api(&endpoint, Some(&req), Method::Put, true)
            .await
    }

    // DNS control
    pub async fn restart_dns(&self) -> Result<RestartDnsResponse, PiHoleApiError> {
        self.execute_api("api/action/restartdns", None::<&()>, Method::Post, true)
            .await
    }

    // Core API execution method
    async fn execute_api<T, R>(
        &self,
        endpoint: &str,
        body: Option<&T>,
        method: Method,
        auth_required: bool,
    ) -> Result<R, PiHoleApiError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = format!(
            "http://{}/{}",
            self.base_url,
            endpoint.trim_start_matches('/')
        );

        let mut request_builder = match method {
            Method::Get => self.client.get(&url),
            Method::Post => self.client.post(&url),
            Method::Put => self.client.put(&url),
            Method::Delete => self.client.delete(&url),
        };

        // Add authentication if required
        if auth_required {
            let session = self.session.lock().unwrap();
            if let Some(sid) = &session.sid {
                request_builder = request_builder
                    .header("X-FTL-SID", sid)
                    .header("X-FTL-CSRF", session.csrf.as_deref().unwrap_or(""));
            } else {
                return Err(PiHoleApiError::AuthenticationRequired);
            }
        }

        // Add JSON body if provided
        if let Some(data) = body {
            request_builder = request_builder.json(data);
        }

        let response = request_builder.send().await?;
        let status = response.status();
        let resp_body = response.text().await?;

        match status {
            StatusCode::NO_CONTENT => serde_json::from_str("null").map_err(|e| e.into()),
            _ => match serde_json::from_str(&resp_body) {
                Ok(r) => Ok(r),
                Err(outer) => match serde_json::from_str(&resp_body) {
                    Ok(e) => Err(PiHoleApiError::HttpApiError(e)),
                    Err(inner) => Err(PiHoleApiError::SerdeJsonBiError(outer, inner)),
                },
            },
        }
    }
}

#[derive(Debug)]
enum Method {
    Get,
    Post,
    Put,
    Delete,
}
