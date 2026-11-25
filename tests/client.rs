//! Runs against a Prism mock of the spec: TORBOX_MOCK_URL=http://127.0.0.1:4011 cargo test
use torbox::apis::{torrents_api, user_api};

fn mock() -> Option<torbox::Client> {
    let url = std::env::var("TORBOX_MOCK_URL").ok()?;
    Some(torbox::Client::builder("00000000-0000-4000-8000-000000000001").base_url(url).max_retries(0).build())
}

#[tokio::test]
async fn envelope_is_checked() {
    let Some(c) = mock() else { return };
    let me = torbox::send(user_api::get_user_data(&c.config, Default::default())).await.unwrap();
    assert!(me.data.and_then(|d| d.email).is_some());
}

#[tokio::test]
async fn multipart_upload() {
    let Some(c) = mock() else { return };
    let path = std::env::temp_dir().join("torbox-sdk-test.torrent");
    std::fs::write(&path, b"d8:announce0:e").unwrap();
    let params = torrents_api::CreateTorrentParams { file: Some(path), ..Default::default() };
    let res = torbox::send(torrents_api::create_torrent(&c.config, params)).await.unwrap();
    assert!(res.data.and_then(|d| d.torrent_id).is_some());
}

#[tokio::test]
async fn download_link_takes_the_key_as_token() {
    let Some(c) = mock() else { return };
    let params = torrents_api::RequestTorrentDownloadLinkParams { token: c.api_key().to_string(), torrent_id: 1, file_id: Some(0), ..Default::default() };
    torbox::send(torrents_api::request_torrent_download_link(&c.config, params)).await.unwrap();
}

#[tokio::test]
async fn errors_are_typed() {
    let Ok(url) = std::env::var("TORBOX_MOCK_URL") else { return };
    let mut c = torbox::Client::builder("x").base_url(url).max_retries(0).build();
    c.config.bearer_access_token = None; // no key -> 401
    let err = torbox::send(user_api::get_user_data(&c.config, Default::default())).await.unwrap_err();
    assert_eq!(err.status, Some(401));
    assert!(!err.detail.is_empty());
}
