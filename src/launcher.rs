use std::collections::HashMap;

use crate::enums;
use reqwest::{Client, Response};
use sys_locale::get_locale;

#[derive(Debug)]
pub struct Launcher {
    language: String,
    version: enums::GameVersion,
    query: HashMap<String, String>,
    url: String,
    client: Client,
}

impl Launcher {
    pub fn new(version: enums::GameVersion, language: Option<&str>) -> Launcher {
        let language: String = language
            .unwrap_or({
                get_locale()
                    .unwrap_or_else(|| String::from("en-us"))
                    .as_str()
            })
            .to_lowercase();
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36")
            .build().expect("Failed to create reqwest client");
        let query: HashMap<String, String>;
        let url: &str;
        match version {
            enums::GameVersion::Overseas => {
                query = HashMap::from([
                    ("channel_id".to_string(), "1".to_string()),
                    ("key".to_string(), "gcStgarh".to_string()),
                    ("launcher_id".to_string(), "10".to_string()),
                ]);
                url = "https://sdk-os-static.hoyoverse.com/hk4e_global";
            }
            enums::GameVersion::China => {
                query = HashMap::from([
                    ("channel_id".to_string(), "1".to_string()),
                    ("key".to_string(), "eYd89JmJ".to_string()),
                    ("launcher_id".to_string(), "18".to_string()),
                ]);
                url = "https://sdk-static.mihoyo.com/hk4e_cn";
            }
            enums::GameVersion::Bilibili => {
                query = HashMap::from([
                    ("channel_id".to_string(), "14".to_string()),
                    ("key".to_string(), "KAtdSsoQ".to_string()),
                    ("launcher_id".to_string(), "17".to_string()),
                ]);
                url = "https://sdk-static.mihoyo.com/hk4e_cn";
            }
        }
        Launcher {
            language,
            version,
            query,
            url: url.to_string(),
            client,
        }
    }
    pub async fn get_launcher_resources(&self, advanced: bool) -> Response {
        let filter_adv: &str = match advanced {
            true => "false",
            false => "true",
        };
        let mut query = self.query.clone();
        query.insert("language".to_string(), self.language.clone());
        query.insert("filter_adv".to_string(), filter_adv.to_string());
        let rsp = self
            .client
            .get(format!("{}{}", self.url, "/mdk/launcher/api/content"))
            .query(&query)
            .send()
            .await
            .expect("Failed to get launcher resources");
        return rsp;
    }
    pub fn get_game_resources(&self) {
        println!("TODO");
    }
}
