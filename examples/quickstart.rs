//! TORBOX_API_KEY=... cargo run --example quickstart
use torbox::apis::{torrents_api, user_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = torbox::Client::new(std::env::var("TORBOX_API_KEY")?);
    let me = torbox::send(user_api::get_user_data(&client.config, Default::default())).await?;
    println!("signed in as {:?}", me.data.and_then(|d| d.email));

    let params = torrents_api::CreateTorrentParams { magnet: Some("magnet:?xt=urn:btih:0000000000000000000000000000000000000000".into()), ..Default::default() };
    if let Err(e) = torbox::send(torrents_api::create_torrent(&client.config, params)).await {
        println!("could not add: {:?} {:?} {}", e.status, e.error, e.detail);
    }
    Ok(())
}
