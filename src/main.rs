use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::widget;
use cosmic::{app, Action, Element, Task};
use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Str};
use zbus::{proxy, Connection};

const APP_ID: &str = "dev.naktix.Cosmify";

// Liste der unterstützten Media Player
const MPRIS_PLAYERS: &[&str] = &[
    "org.mpris.MediaPlayer2.spotify",
    "org.mpris.MediaPlayer2.vlc",
    "org.mpris.MediaPlayer2.rhythmbox",
    "org.mpris.MediaPlayer2.plasma-browser-integration",
];

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<Cosmify>(())
}

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
struct TrackInfo {
    title: String,
    artist: String,
    #[allow(dead_code)]
    album: String,
    is_playing: bool,
    #[allow(dead_code)]
    available: bool,
}

impl Default for TrackInfo {
    fn default() -> Self {
        Self {
            title: "No Track".to_string(),
            artist: "Waiting for a Media Player...".to_string(),
            album: String::new(),
            is_playing: false,
            available: false,
        }
    }
}

impl TrackInfo {
    fn unavailable() -> Self {
        Self {
            title: "No Track".to_string(),
            artist: "No Media Player active...".to_string(),
            album: String::new(),
            is_playing: false,
            available: false,
        }
    }
}

struct Cosmify {
    core: app::Core,
    track_info: TrackInfo,
}

#[derive(Debug, Clone)]
enum Message {
    PlayPause,
    Next,
    Previous,
    Tick,
    UpdateTrackInfo(TrackInfo),
}

#[derive(Debug, Clone, Copy)]
enum MprisCommand {
    PlayPause,
    Next,
    Previous,
}

impl cosmic::Application for Cosmify {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut app::Core {
        &mut self.core
    }

    fn init(core: app::Core, _flags: Self::Flags) -> (Self, Task<Action<Self::Message>>) {
        let app = Cosmify {
            core,
            track_info: TrackInfo::default(),
        };

        let task = Task::perform(fetch_mpris_data(), |info| {
            Action::App(Message::UpdateTrackInfo(info))
        });

        (app, task)
    }

    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::PlayPause => execute_command(MprisCommand::PlayPause),
            Message::Next => execute_command(MprisCommand::Next),
            Message::Previous => execute_command(MprisCommand::Previous),
            Message::Tick => Task::perform(fetch_mpris_data(), |info| {
                Action::App(Message::UpdateTrackInfo(info))
            }),
            Message::UpdateTrackInfo(info) => {
                self.track_info = info;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        cosmic::iced::time::every(std::time::Duration::from_secs(2)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let buttons = create_control_buttons(self.track_info.is_playing);
        let track_info = create_track_info_view(&self.track_info);

        let content_row = widget::row()
            .push(buttons)
            .push(track_info)
            .spacing(16.0)
            .align_y(Alignment::Center)
            .height(Length::Fixed(48.0));

        let content = widget::container(content_row)
            .center_x(Length::Fill)
            .padding([8.0, 8.0, 8.0, 8.0])
            .width(Length::Fill)
            .height(Length::Shrink);

        self.core.applet.popup_container(content).into()
    }
}

fn create_control_buttons(is_playing: bool) -> widget::Row<'static, Message> {
    let play_pause_icon = if is_playing {
        "media-playback-pause-symbolic"
    } else {
        "media-playback-start-symbolic"
    };

    widget::row()
        .push(icon_button(
            "media-skip-backward-symbolic",
            Message::Previous,
        ))
        .push(icon_button(play_pause_icon, Message::PlayPause))
        .push(icon_button("media-skip-forward-symbolic", Message::Next))
        .spacing(8.0)
        .align_y(Alignment::Center)
        .width(Length::Shrink)
}

fn icon_button<'a>(icon: &'a str, message: Message) -> impl Into<Element<'a, Message>> {
    widget::button::icon(widget::icon::from_name(icon))
        .on_press(message)
        .width(Length::Fixed(32.0))
        .height(Length::Fixed(32.0))
}

fn create_track_info_view(track_info: &TrackInfo) -> widget::Column<'static, Message> {
    widget::column()
        .push(widget::text(truncate_text(&track_info.title, 25)).size(14))
        .push(widget::text(truncate_text(&track_info.artist, 20)).size(12))
        .spacing(2.0)
        .align_x(Alignment::Start)
        .width(Length::Fill)
}

fn execute_command(command: MprisCommand) -> Task<Action<Message>> {
    Task::perform(
        async move {
            let _ = send_mpris_command(command).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            fetch_mpris_data().await
        },
        |info| Action::App(Message::UpdateTrackInfo(info)),
    )
}

async fn send_mpris_command(command: MprisCommand) -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::session().await?;

    for player in MPRIS_PLAYERS {
        if let Ok(proxy) = MediaPlayer2PlayerProxy::builder(&connection)
            .destination(*player)?
            .build()
            .await
        {
            let result = match command {
                MprisCommand::PlayPause => proxy.play_pause().await,
                MprisCommand::Next => proxy.next().await,
                MprisCommand::Previous => proxy.previous().await,
            };

            if result.is_ok() {
                return Ok(());
            }
        }
    }

    Err("No Media Player found".into())
}

async fn fetch_mpris_data() -> TrackInfo {
    try_fetch_mpris_data()
        .await
        .unwrap_or_else(|_| TrackInfo::unavailable())
}

async fn try_fetch_mpris_data() -> Result<TrackInfo, Box<dyn std::error::Error>> {
    let connection = Connection::session().await?;

    for player in MPRIS_PLAYERS {
        if let Ok(proxy) = MediaPlayer2PlayerProxy::builder(&connection)
            .destination(*player)?
            .build()
            .await
        {
            if let Ok(metadata) = proxy.metadata().await {
                let title = extract_string(&metadata, "xesam:title")
                    .unwrap_or_else(|| "Unknown Title".to_string());

                let artist = extract_string_array(&metadata, "xesam:artist")
                    .unwrap_or_else(|| "Unknown Artist".to_string());

                let album = extract_string(&metadata, "xesam:album").unwrap_or_default();

                let status = proxy.playback_status().await.unwrap_or_default();
                let is_playing = status == "Playing";

                return Ok(TrackInfo {
                    title,
                    artist,
                    album,
                    is_playing,
                    available: true,
                });
            }
        }
    }

    Err("No Player found".into())
}

fn extract_string(metadata: &HashMap<String, OwnedValue>, key: &str) -> Option<String> {
    let v = metadata.get(key)?;
    let s: Str = v.downcast_ref::<Str>().ok()?;
    Some(s.to_string())
}

fn extract_string_array(metadata: &HashMap<String, OwnedValue>, key: &str) -> Option<String> {
    let v = metadata.get(key)?;
    let arr: &zbus::zvariant::Array = v.downcast_ref().ok()?;
    let first_val = arr.get::<zbus::zvariant::Value>(0).ok()??;
    let s: &Str = first_val.downcast_ref().ok()?;
    Some(s.to_string())
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.chars().count() > max_len {
        let truncated: String = text.chars().take(max_len - 1).collect();
        format!("{}…", truncated)
    } else {
        text.to_string()
    }
}
