use worthless::{enums, launcher};

extern crate worthless;

#[tokio::main]
async fn main() {
    println!("This should print the launcher response in your system locale");
    let lc = launcher::Launcher::new(enums::GameVersion::Overseas, None);
    let rsp = lc.get_launcher_resources(false).await;
    println!("Background image url: {}", rsp.message());
    println!("{:?}", rsp);
}
