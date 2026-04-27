use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Str};
use zbus::{proxy, Connection};

const MPRIS_PLAYERS: &[&str] = &[
    "org.mpris.MediaPlayer2.spotify",
    "org.mpris.MediaPlayer2.vlc",
    "org.mpris.MediaPlayer2.rhythmbox",
    "org.mpris.MediaPlayer2.plasma-browser-integration",
];

#[proxy(
    interface = "org.mpris.MediaPlayer2.Player",
    default_service = "org.mpris.MediaPlayer2.spotify",
    default_path = "/org/mpris/MediaPlayer2"
)]
trait MediaPlayer2Player {
    fn play_pause(&self) -> zbus::Result<()>;
    fn next(&self) -> zbus::Result<()>;
    fn previous(&self) -> zbus::Result<()>;
    #[zbus(property)]
    fn playback_status(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn metadata(&self) -> zbus::Result<HashMap<String, OwnedValue>>;
}

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub is_playing: bool,
    pub art_url: Option<String>,
    pub art_bytes: Option<Vec<u8>>,
}

impl Default for TrackInfo {
    fn default() -> Self {
        Self {
            title: "No Track".into(),
            artist: "Waiting...".into(),
            is_playing: false,
            art_url: None,
            art_bytes: None,
        }
    }
}

impl TrackInfo {
    pub fn unavailable() -> Self {
        Self {
            title: "No Player".into(),
            artist: String::new(),
            is_playing: false,
            art_url: None,
            art_bytes: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cmd {
    PlayPause,
    Next,
    Previous,
}

pub async fn send_command(cmd: Cmd) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::session().await?;
    for player in MPRIS_PLAYERS {
        if let Ok(proxy) = MediaPlayer2PlayerProxy::builder(&conn)
            .destination(*player)?
            .build()
            .await
        {
            let result = match cmd {
                Cmd::PlayPause => proxy.play_pause().await,
                Cmd::Next => proxy.next().await,
                Cmd::Previous => proxy.previous().await,
            };
            if result.is_ok() {
                return Ok(());
            }
        }
    }
    Err("No Media Player found".into())
}

pub async fn fetch_track() -> TrackInfo {
    try_fetch_track()
        .await
        .unwrap_or_else(|_| TrackInfo::unavailable())
}

async fn try_fetch_track() -> Result<TrackInfo, Box<dyn std::error::Error>> {
    let conn = Connection::session().await?;

    for player in MPRIS_PLAYERS {
        let proxy = match MediaPlayer2PlayerProxy::builder(&conn)
            .destination(*player)?
            .build()
            .await
        {
            Ok(p) => p,
            Err(_) => continue,
        };

        let status = proxy.playback_status().await.unwrap_or_default();
        if status != "Playing" && status != "Paused" {
            continue;
        }

        let meta = match proxy.metadata().await {
            Ok(m) => m,
            Err(_) => continue,
        };

        let art_url_raw = meta_string(&meta, "mpris:artUrl");

        let (art_url, art_bytes) = resolve_art(art_url_raw).await;

        return Ok(TrackInfo {
            title: meta_string(&meta, "xesam:title").unwrap_or_else(|| "Unknown Title".into()),
            artist: meta_string_array(&meta, "xesam:artist")
                .unwrap_or_else(|| "Unknown Artist".into()),
            is_playing: status == "Playing",
            art_url,
            art_bytes,
        });
    }

    Err("No active player found".into())
}

async fn resolve_art(raw: Option<String>) -> (Option<String>, Option<Vec<u8>>) {
    match raw {
        None => (None, None),
        Some(url) if url.starts_with("file://") => {
            let path = url.trim_start_matches("file://").to_string();
            (Some(path), None)
        }
        Some(url) if url.starts_with("http://") || url.starts_with("https://") => {
            match reqwest::get(&url).await {
                Ok(resp) => match resp.bytes().await {
                    Ok(bytes) => (None, Some(bytes.to_vec())),
                    Err(_) => (None, None),
                },
                Err(_) => (None, None),
            }
        }
        Some(_) => (None, None),
    }
}

fn meta_string(meta: &HashMap<String, OwnedValue>, key: &str) -> Option<String> {
    Some(meta.get(key)?.downcast_ref::<Str>().ok()?.to_string())
}

fn meta_string_array(meta: &HashMap<String, OwnedValue>, key: &str) -> Option<String> {
    let arr: &zbus::zvariant::Array = meta.get(key)?.downcast_ref().ok()?;
    let val = arr.get::<zbus::zvariant::Value>(0).ok()??;
    Some(val.downcast_ref::<Str>().ok()?.to_string())
}
