use worthless::{enums, launcher};

extern crate worthless;

#[tokio::main]
async fn main() {
    println!("This should print the launcher response for vi-vn locale");
    let lc = launcher::Launcher::new(enums::GameVersion::Overseas, Some("vi-vn"));
    let rsp = lc.get_launcher_resources(false).await;
    println!("{:?}", rsp.text().await.unwrap());
}
