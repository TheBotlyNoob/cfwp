use warp_client::config::Config;

#[tokio::main]
async fn main() {
    let priv_key = x25519_dalek::ReusableSecret::random();
    let pub_key = x25519_dalek::PublicKey::from(&priv_key);
    let mut config = Config::default();
    dbg!(
        warp_client::api::register(&config, pub_key.as_bytes(), "PC")
            .await
            .unwrap()
    );
}
