use cosmic::iced::{Alignment, Length, window};
use cosmic::iced_widget::row;
use cosmic::widget;
use cosmic::{app, Element, Task, Action};

const APP_ID: &str = "dev.naktix.Cosmify";

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<Cosmify>(())
}

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<Cosmify>(())
}

struct Cosmify {
    core: app::Core,
    track_title: String,
    track_artist: String,
    is_playing: bool,
}

#[derive(Debug, Clone)]
enum Message {
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

    fn init(
        core: app::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<Action<Self::Message>>) {
        let app = Cosmify {
            core,
            track_title: "No Track".to_string(),
            track_artist: "Spotify".to_string(),
            is_playing: false,
        };

        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::PlayPause => {
                self.is_playing = !self.is_playing;
                if self.is_playing {
                    self.track_title = "Now Playing".to_string();
                } else {
                    self.track_title = "Paused".to_string();
                }
            }
            Message::Next => {
                self.track_title = "Next Track".to_string();
            }
            Message::Previous => {
                self.track_title = "Previous Track".to_string();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_,Self::Message> {
        let icon_name = if self.is_playing {
            "media-playback-pause-symbolic"
        } else {
            "media-playback-start-symbolic"
        };

        self.core
            .applet
            .icon_button(icon_name)
            .on_press(Message::PlayPause)
            .into()
    }

    fn view_window(&self, _id: window::Id) -> Element<'_, Self::Message> {
        let play_pause_icon = if self.is_playing {
            "media-playback-pause-symbolic"
        } else {
            "media-playback-start-symbolic"
        };

        let content = widget::column::with_capacity(4)
            .push(
                widget::text::title3(&self.track_title)
                    .width(Length::Fill),
            )
            .push(
                widget::text(&self.track_artist)
                    .size(14)
                    .width(Length::Fill),
            )
            .push(widget::divider::horizontal::default())
            .push(
                row![
                    widget::button::icon(
                        widget::icon::from_name("media-skip-backward-symbolic")
                    )
                    .on_press(Message::Previous),
                    
                    widget::button::icon(
                        widget::icon::from_name(play_pause_icon)
                    )
                    .on_press(Message::PlayPause),
                    
                    widget::button::icon(
                        widget::icon::from_name("media-skip-forward-symbolic")
                    )
                    .on_press(Message::Next),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .spacing(12)
            .padding(16);

        self.core.applet.popup_container(content).into()
    }
}