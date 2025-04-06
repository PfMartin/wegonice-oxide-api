mod common;

use anyhow::Result;
use common::{get_config, AuthPayload, ResponseBody};
use reqwest::{Client, StatusCode};

#[tokio::test]
async fn register_success() -> Result<()> {
    let register_payload = AuthPayload {
        email: "registerUser@gmail.com".into(),
        password: "test_password".into(),
    };

    let config = get_config(Some(".env"))?;

    let client = Client::new();
    let res = client
        .post(format!("http://{}/auth/register", config.server_host))
        .json(&register_payload)
        .send()
        .await?;

    assert_eq!(res.status().is_success(), true);
    assert_eq!(res.status(), StatusCode::ACCEPTED);

    let response_body = res.json::<ResponseBody>().await?;

    assert_eq!(response_body.data.is_some(), true);
    assert_eq!(response_body.error, String::from(""));

    Ok(())
}
