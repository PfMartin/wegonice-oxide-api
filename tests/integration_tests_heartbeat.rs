mod common;

use anyhow::Result;
use common::ResponseBody;
use pretty_assertions::assert_eq;
use reqwest::StatusCode;

#[tokio::test]
async fn heartbeat_request_successful() -> Result<()> {
    let res = reqwest::get("http://localhost:5000/heart_beat").await?;

    assert_eq!(res.status().is_success(), true);
    assert_eq!(res.status(), StatusCode::OK);

    let response_body = res.json::<ResponseBody>().await?;

    assert_eq!(response_body.data, Some(String::from("Ok")));
    assert_eq!(response_body.error, String::from(""));

    Ok(())
}
