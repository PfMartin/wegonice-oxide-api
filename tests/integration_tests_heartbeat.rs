use anyhow::Result;
use pretty_assertions::assert_eq;
use reqwest::StatusCode;

#[tokio::test]
async fn heartbeat_request_successful() -> Result<()> {
    let res = reqwest::get("http://localhost:5000/heart_beat").await?;

    assert_eq!(res.status().is_success(), true);
    assert_eq!(res.status(), StatusCode::OK);

    Ok(())
}
