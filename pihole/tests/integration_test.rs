use mockito::{Mock, Server};
use pihole::api::PiHoleV6Client;
use pihole::error::PiHoleApiError;
use serde_json::json;

#[tokio::test]
async fn test_full_workflow() {
    let mut server = Server::new_async().await;
    let mut client = PiHoleV6Client::new(server.host_with_port());

    // Test successful login
    let login_mock = mock_api_call(
        &mut server,
        "POST",
        "/api/auth",
        Some(json!({"password": "valid_password"})),
        json!({
            "session": {
                "valid": true,
                "totp": false,
                "sid": "test_session_id",
                "csrf": "test_csrf_token",
                "validity": 1800,
                "message": "success"
            },
            "took": 0.001
        }),
        200,
        false,
    )
    .await;

    client.login("valid_password").await.unwrap();
    login_mock.assert_async().await;

    // Test protected endpoints with valid session
    let groups_mock = mock_api_call(
        &mut server,
        "GET",
        "/api/groups",
        None,
        json!({
            "groups": [],
            "took": 0.001
        }),
        200,
        true,
    )
    .await;

    client.get_groups().await.unwrap();
    groups_mock.assert_async().await;

    // Test logout
    let logout_mock =
        mock_api_call(&mut server, "DELETE", "/api/auth", None, json!(null), 204, false).await;

    client.logout().await.unwrap();
    logout_mock.assert_async().await;
}

#[tokio::test]
async fn test_authentication_failure() {
    let mut server = Server::new_async().await;
    let mut client = PiHoleV6Client::new(server.host_with_port());

    // Mock failed login response
    let mock = mock_api_call(
        &mut server,
        "POST",
        "/api/auth",
        Some(json!({"password": "wrong_password"})),
        json!({
            "error": {
                "key": "AUTH_ERROR",
                "message": "Authentication failed",
                "hint": "Check your password"
            },
            "took": 0.001
        }),
        401,
        false,
    )
    .await;

    let result = client.login("wrong_password").await;
    assert!(matches!(result, Err(PiHoleApiError::HttpApiError(_))));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_protected_endpoint_without_auth() {
    let mut server = Server::new_async().await;
    let client = PiHoleV6Client::new(server.host_with_port());

    // Mock protected endpoint
    let _mock = mock_api_call(
        &mut server,
        "GET",
        "/api/groups",
        None,
        json!({
            "error": {
                "key": "unauthorized",
                "message": "Unauthorized",
                "hint": null
            },
            "took": 0.001
        }),
        401,
        false,
    )
    .await;

    let result = client.get_groups().await;
    assert!(matches!(result, Err(PiHoleApiError::AuthenticationRequired)));
}

#[tokio::test]
async fn test_expired_session() {
    let mut server = Server::new_async().await;
    let mut client = PiHoleV6Client::new(server.host_with_port());

    // Initial successful login
    let _login_mock = mock_api_call(
        &mut server,
        "POST",
        "/api/auth",
        Some(json!({"password": "valid"})),
        json!({
            "session": {
                "valid": true,
                "totp": false,
                "sid": "expired_session",
                "csrf": "expired_csrf",
                "validity": 1,
                "message": "success"
            },
            "took": 0.001
        }),
        200,
        false,
    )
    .await;
    client.login("valid").await.unwrap();

    // Mock expired session response
    let groups_mock = mock_api_call(
        &mut server,
        "GET",
        "/api/groups",
        None,
        json!({
            "error": {
                "key": "unauthorized",
                "message": "Session expired",
                "hint": "Re-authenticate"
            },
            "took": 0.001
        }),
        401,
        false,
    )
    .await;

    let result = client.get_groups().await;
    assert!(matches!(result, Err(PiHoleApiError::HttpApiError(_))));
    groups_mock.assert_async().await;
}

async fn mock_api_call(
    server: &mut Server,
    method: &str,
    path: &str,
    body: Option<serde_json::Value>,
    response: serde_json::Value,
    status: usize,
    auth_headers: bool,
) -> Mock {
    let mut mock = server
        .mock(method, path)
        .with_status(status)
        .with_body(response.to_string());

    // Only match content-type for requests with bodies
    if body.is_some() {
        mock = mock.match_header("content-type", "application/json");
    }

    // Add authentication header expectations if required
    if auth_headers {
        mock = mock
            .match_header("X-FTL-SID", "test_session_id")
            .match_header("X-FTL-CSRF", "test_csrf_token");
    }

    if let Some(body) = body {
        mock = mock.match_body(body.to_string().as_str());
    }

    mock.create_async().await
}
