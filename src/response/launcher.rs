use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LauncherResponse {
    retcode: i32,
    message: String,
    data: Data,
}

#[allow(dead_code)]
impl LauncherResponse {
    pub fn retcode(&self) -> i32 {
        self.retcode
    }
    pub fn message(&self) -> &String {
        &self.message
    }
    pub fn background_url(&self) -> &String {
        &self.data.adv.background
    }
}

#[derive(Deserialize, Debug)]
struct Data {
    adv: Adv,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Adv {
    background: String,
    icon: String,
    url: String,
    version: String,
    bg_checksum: String,
}
