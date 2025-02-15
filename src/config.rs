use anyhow::Result;
use std::env;
use tracing::info;

#[derive(Debug)]
pub struct Config {
    pub db_name: String,
    pub db_user_name: String,
    pub db_user_password: String,
    pub db_host: String,
    pub server_host: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn new(config_path: Option<&str>) -> Result<Self> {
        match config_path {
            Some(path) => {
                if dotenv::from_path(path).is_err() {
                    info!("Using config from env variables");
                };
            }
            None => info!("Using config from env variables"),
        }

        let db_name = env::var("MONGO_WEGONICE_DB")?;
        let db_user_name = env::var("MONGO_WEGONICE_USER")?;
        let db_user_password = env::var("MONGO_WEGONICE_PASSWORD")?;
        let db_host = env::var("MONGO_WEGONICE_HOST")?;
        let server_host = env::var("SERVER_HOST")?;
        let jwt_secret = env::var("JWT_SECRET")?;

        Ok(Self {
            db_name,
            db_user_name,
            db_user_password,
            db_host,
            server_host,
            jwt_secret,
        })
    }
}

#[cfg(test)]
pub mod unit_tests_config {
    use std::{collections::HashMap, fs};

    use super::*;
    use crate::test_utils::print_assert_failed;

    #[test]
    fn get_config() -> Result<()> {
        struct TestCase {
            title: String,
            expected_config: Option<Config>,
            env_file_path: String,
            setup_env_file: Option<String>,
        }

        let test_cases = vec![
            TestCase {
                title: "Successfully gets config".into(),
                expected_config: Some(Config {
                    db_name: "wegonice".into(),
                    db_user_name: "niceUser".into(),
                    db_user_password: "nicePassword".into(),
                    db_host: "127.0.0.1:27017".into(),
                    server_host: "127.0.0.1:3000".into(),
                    jwt_secret: "test".into()
                }),
                env_file_path: "src/test-env".into(),
                setup_env_file: Some(
                    r#"
                    MONGO_WEGONICE_DB=wegonice
                    MONGO_WEGONICE_USER=niceUser
                    MONGO_WEGONICE_PASSWORD=nicePassword
                    MONGO_WEGONICE_HOST=127.0.0.1:27017
                    SERVER_HOST=127.0.0.1:3000
                    JWT_SECRET=test
                    "#
                    .into(),
                ),
            },
            TestCase {
                title: "Fails to get config due to non-existing env and env variables not set file path".into(),
                expected_config: None,
                env_file_path: "src/non-existing".into(),
                setup_env_file: None,
            },
            TestCase {
                title: "Fails to get config due to missing environment variables".into(),
                expected_config: None,
                env_file_path: "src/.env_missing_vars".into(),
                setup_env_file: Some(
                    r#"
                    MONGO_WEGONICE_DB=wegonice
                    MONGO_WEGONICE_PASSWORD=nicePassword
                    MONGO_WEGONICE_HOST=127.0.0.1:27017
                    SERVER_HOST=127.0.0.1:3000
                    "#
                    .into(),
                ),
            },
        ];

        for t in test_cases {
            let saved_vars = clear_env_vars()?;

            if let Some(content) = &t.setup_env_file {
                fs::write(&t.env_file_path, content)?
            }

            let config = Config::new(Some(&t.env_file_path));

            match t.expected_config {
                Some(expected_config) => {
                    assert!(config.is_ok());
                    let config = config?;

                    assert_eq!(
                        config.db_name,
                        expected_config.db_name,
                        "{}",
                        print_assert_failed(&t.title, &expected_config.db_name, &config.db_name)
                    );
                    assert_eq!(
                        config.db_user_name,
                        expected_config.db_user_name,
                        "{}",
                        print_assert_failed(
                            &t.title,
                            &expected_config.db_user_name,
                            &config.db_user_name
                        )
                    );
                    assert_eq!(
                        config.db_user_password,
                        expected_config.db_user_password,
                        "{}",
                        print_assert_failed(
                            &t.title,
                            &expected_config.db_user_password,
                            &config.db_user_password
                        )
                    );
                    assert_eq!(
                        config.server_host,
                        expected_config.server_host,
                        "{}",
                        print_assert_failed(
                            &t.title,
                            &expected_config.server_host,
                            &config.server_host
                        )
                    );
                }
                None => assert!(config.is_err()),
            }

            if t.setup_env_file.is_some() {
                fs::remove_file(&t.env_file_path)?;
            }

            restore_env_vars(saved_vars);
        }

        Ok(())
    }

    fn clear_env_vars() -> Result<HashMap<String, String>> {
        let mut saved_vars = HashMap::new();

        let env_var_keys = vec![
            "MONGO_WEGONICE_DB",
            "MONGO_WEGONICE_USER",
            "MONGO_WEGONICE_PASSWORD",
            "MONGO_WEGONICE_HOST",
            "SERVER_HOST",
        ];

        for key in env_var_keys {
            let var_result = env::var(key);

            match var_result {
                Ok(value) => {
                    saved_vars.insert(String::from(key), value);

                    env::remove_var(key)
                }
                Err(_) => {}
            };
        }

        Ok(saved_vars)
    }

    fn restore_env_vars(saved_vars: HashMap<String, String>) {
        for (key, value) in saved_vars {
            env::set_var(key, value);
        }
    }
}
