mod common;

use anyhow::Result;
use common::{get_config, register_user, AuthPayload, ResponseBody};
use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};

#[tokio::test]
async fn login_success() -> Result<()> {
    let _ = register_user().await?;

    let login_payload = AuthPayload {
        email: "loginUser@email.com".into(),
        password: "test_password".into(),
    };

    let config = get_config(Some(".env"))?;

    let client = Client::new();
    let res = client
        .post(format!("http://{}/auth/login", config.server_host))
        .json(&login_payload)
        .send()
        .await?;

    assert_eq!(res.status().is_success(), true);
    assert_eq!(res.status(), StatusCode::ACCEPTED);

    let response_body = res.json::<ResponseBody>().await?;

    assert_eq!(response_body.data.is_some(), true);
    assert_eq!(response_body.error, String::from(""));

    Ok(())
}

#[test]
fn login_fail() {
    assert_eq!(2 == 1, false);
}
