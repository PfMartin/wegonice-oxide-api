use anyhow::Result;
use dotenv::dotenv;
use std::env;

pub struct Config {
    pub db_name: String,
    pub db_user_name: String,
    pub db_user_password: String,
    pub db_host: String,
}

impl Config {
    pub fn new() -> Result<Config> {
        dotenv()?;

        let db_name = env::var("MONGO_WEGONICE_DB")?;
        let db_user_name = env::var("MONGO_WEGONICE_USER")?;
        let db_user_password = env::var("MONGO_WEGONICE_PASSWORD")?;
        let db_host = env::var("MONGO_WEGONICE_HOST")?;

        Ok(Config {
            db_name,
            db_user_name,
            db_user_password,
            db_host,
        })
    }
}

#[cfg(test)]
pub mod unit_tests_config {
    use super::*;
    use crate::test_utils::print_assert_failed;

    #[test]
    fn get_config() -> Result<()> {
        struct TestCase {
            title: String,
            expected_config: Config,
        }

        let test_cases = vec![TestCase {
            title: String::from("Successfully gets config"),
            expected_config: Config {
                db_name: "wegonice".into(),
                db_user_name: "niceUser".into(),
                db_user_password: "nicePassword".into(),
                db_host: "127.0.0.1:27017".into(),
            },
        }];

        for t in test_cases {
            let config = Config::new()?;

            assert_eq!(
                config.db_name,
                t.expected_config.db_name,
                "{}",
                print_assert_failed(&t.title, &t.expected_config.db_name, &config.db_name)
            );
            assert_eq!(
                config.db_user_name,
                t.expected_config.db_user_name,
                "{}",
                print_assert_failed(
                    &t.title,
                    &t.expected_config.db_user_name,
                    &config.db_user_name
                )
            );
            assert_eq!(
                config.db_user_password,
                t.expected_config.db_user_password,
                "{}",
                print_assert_failed(
                    &t.title,
                    &t.expected_config.db_user_password,
                    &config.db_user_password
                )
            );
        }

        Ok(())
    }
}
