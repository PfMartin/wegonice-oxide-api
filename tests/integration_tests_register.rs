mod common;

use anyhow::Result;
use common::ResponseBody;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RegisterPayload {
    email: String,
    password: String,
}

#[tokio::test]
async fn register_success() -> Result<()> {
    let register_payload = RegisterPayload {
        email: "registerUser@gmail.com".into(),
        password: "test_password".into(),
    };

    let client = Client::new();
    let res = client
        .post("http://localhost:5000/auth/register")
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
