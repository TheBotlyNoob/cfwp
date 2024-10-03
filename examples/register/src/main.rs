use std::io::Write as _;

use warp_client::config::Config;

#[tokio::main]
async fn main() {
    let priv_key = x25519_dalek::ReusableSecret::random();
    let pub_key = x25519_dalek::PublicKey::from(&priv_key);
    let config = Config::default();
    accept_tos();
    println!(
        "{:#?}",
        warp_client::api::register(&config, pub_key.as_bytes(), "PC")
            .await
            .unwrap()
    );
}

fn accept_tos() {
    eprint!("Do you accept Cloudflare's Terms of Service? (y/n): ");
    std::io::stdout().flush().unwrap();
    let mut letter = String::new();
    std::io::stdin().read_line(&mut letter).unwrap();
    if letter.starts_with('n') {
        panic!("You must accept the Terms of Service to use this software.");
    } else if !letter.starts_with('y') {
        println!("Invalid input.");
        accept_tos()
    }
}
