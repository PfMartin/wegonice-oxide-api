mod common;

use anyhow::Result;
use common::{delete_users, get_config, register_user, AuthPayload, ResponseBody};
use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};

#[tokio::test]
async fn login() -> Result<()> {
    let config = get_config(Some(".env"))?;

    struct TestCase {
        title: String,
        login_payload: AuthPayload,
        response_code: StatusCode,
        is_user_active: bool,
        is_success: bool,
    }

    let test_cases = vec![
        TestCase {
            title: "Successfully logs user in".into(),
            login_payload: AuthPayload {
                email: "loginUser@email.com".into(),
                password: "test_password".into(),
            },
            is_user_active: true,
            is_success: true,
            response_code: StatusCode::ACCEPTED,
        },
        TestCase {
            title: "Fails to login with wrong password".into(),
            login_payload: AuthPayload {
                email: "loginUser@email.com".into(),
                password: "wrong_password".into(),
            },
            is_user_active: true,
            is_success: false,
            response_code: StatusCode::UNAUTHORIZED,
        },
        TestCase {
            title: "Fails to login with wrong password".into(),
            login_payload: AuthPayload {
                email: "wrongUser@email.com".into(),
                password: "test_password".into(),
            },
            is_user_active: true,
            is_success: false,
            response_code: StatusCode::NOT_FOUND,
        },
        TestCase {
            title: "Fails to login with wrong password".into(),
            login_payload: AuthPayload {
                email: "loginUser@email.com".into(),
                password: "test_password".into(),
            },
            is_user_active: false,
            is_success: false,
            response_code: StatusCode::UNAUTHORIZED,
        },
    ];

    for t in test_cases {
        let _ = register_user(t.is_user_active).await?;
        let client = Client::new();
        let res = client
            .post(format!("http://{}/auth/login", config.server_host))
            .json(&t.login_payload)
            .send()
            .await?;

        assert_eq!(res.status().is_success(), t.is_success, "{}", &t.title);
        assert_eq!(res.status(), t.response_code, "{}", &t.title);

        let response_body = res.json::<ResponseBody>().await?;

        assert_eq!(response_body.data.is_some(), t.is_success, "{}", &t.title);

        if t.is_success {
            assert_eq!(response_body.error, String::from(""), "{}", &t.title);
        }

        delete_users().await?;
    }

    Ok(())
}
