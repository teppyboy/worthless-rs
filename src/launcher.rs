use std::collections::HashMap;

use crate::{enums, response::launcher::LauncherResponse};
use reqwest::Client;
// use serde_json::Value;
use sys_locale::get_locale;

pub struct Launcher<'a> {
    language: String,
    version: enums::GameVersion,
    query: HashMap<&'a str, &'a str>,
    url: &'a str,
    client: Client,
}

impl Launcher<'_> {
    pub fn new(version: enums::GameVersion, language: Option<&str>) -> Launcher {
        let language: String = language
            .unwrap_or({
                get_locale()
                    .unwrap_or_else(|| String::from("en-us"))
                    .as_str()
            })
            .to_lowercase().replace("_", "-");
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36")
            .build().expect("Failed to create reqwest client");
        let query: HashMap<&str, &str>;
        let url: &str;
        match version {
            enums::GameVersion::Overseas => {
                query = HashMap::from([
                    ("channel_id", "1"),
                    ("key", "gcStgarh"),
                    ("launcher_id", "10"),
                ]);
                url = "https://sdk-os-static.hoyoverse.com/hk4e_global";
            }
            enums::GameVersion::China => {
                query = HashMap::from([
                    ("channel_id", "1"),
                    ("key", "eYd89JmJ"),
                    ("launcher_id", "18"),
                ]);
                url = "https://sdk-static.mihoyo.com/hk4e_cn";
            }
            enums::GameVersion::Bilibili => {
                query = HashMap::from([
                    ("channel_id", "14"),
                    ("key", "KAtdSsoQ"),
                    ("launcher_id", "17"),
                ]);
                url = "https://sdk-static.mihoyo.com/hk4e_cn";
            }
        }
        Launcher {
            language,
            version,
            query,
            url,
            client,
        }
    }
    pub async fn get_launcher_resources(&self, advanced: bool) -> LauncherResponse {
        let filter_adv: &str = match advanced {
            true => "false",
            false => "true",
        };
        let mut query = self.query.clone();
        query.insert("language", &self.language);
        query.insert("filter_adv", filter_adv);
        let rsp = self
            .client
            .get(format!("{}{}", self.url, "/mdk/launcher/api/content"))
            .query(&query)
            .send()
            .await
            .expect("Failed to get launcher resources");
        let wrapped_rsp: LauncherResponse = rsp.json().await.expect("Failed to extract response");
        return wrapped_rsp;
    }
    pub fn get_game_resources(&self) {
        println!("TODO");
    }
}
