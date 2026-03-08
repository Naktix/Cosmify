use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::widget;
use cosmic::{app, Action, Element, Task};

use crate::mpris::{self, Cmd, TrackInfo};

const APP_ID: &str = "dev.naktix.Cosmify";

const APPLET_WIDTH: f32 = 360.0;
const ROW_HEIGHT: f32 = 32.0;
const BTN_SIZE: f32 = 30.0;
const PADDING: [f32; 2] = [3.0, 6.0];
const SPACING: f32 = 6.0;
const TEXT_SIZE: u16 = 12;
const MAX_TITLE: usize = 20;
const MAX_ARTIST: usize = 18;

const ICON_PREV: &str = "media-skip-backward-symbolic";
const ICON_NEXT: &str = "media-skip-forward-symbolic";
const ICON_PLAY: &str = "media-playback-start-symbolic";
const ICON_PAUSE: &str = "media-playback-pause-symbolic";

pub struct Cosmify {
    core: app::Core,
    track: TrackInfo,
}

#[derive(Debug, Clone)]
pub enum Msg {
    PlayPause,
    Next,
    Previous,
    Tick,
    TrackUpdated(TrackInfo),
}

impl cosmic::Application for Cosmify {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Msg;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut app::Core {
        &mut self.core
    }

    fn init(core: app::Core, _flags: ()) -> (Self, Task<Action<Msg>>) {
        let app = Cosmify {
            core,
            track: TrackInfo::default(),
        };
        (app, fetch_track_task())
    }

    fn update(&mut self, msg: Msg) -> Task<Action<Msg>> {
        match msg {
            Msg::PlayPause => send_and_refresh(Cmd::PlayPause),
            Msg::Next => send_and_refresh(Cmd::Next),
            Msg::Previous => send_and_refresh(Cmd::Previous),
            Msg::Tick => fetch_track_task(),
            Msg::TrackUpdated(info) => {
                self.track = info;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Msg> {
        cosmic::iced::time::every(std::time::Duration::from_secs(1)).map(|_| Msg::Tick)
    }

    fn view(&self) -> Element<'_, Msg> {
        let play_icon = if self.track.is_playing {
            ICON_PAUSE
        } else {
            ICON_PLAY
        };

        let buttons = widget::row()
            .push(icon_btn(ICON_PREV, Msg::Previous).into())
            .push(icon_btn(play_icon, Msg::PlayPause).into())
            .push(icon_btn(ICON_NEXT, Msg::Next).into())
            .spacing(SPACING)
            .align_y(Alignment::Center);

        let text = widget::text(self.track.display_text(MAX_TITLE, MAX_ARTIST)).size(TEXT_SIZE);

        let row = widget::row()
            .push(buttons)
            .push(text)
            .spacing(SPACING)
            .align_y(Alignment::Center)
            .height(Length::Fixed(ROW_HEIGHT));

        let container = widget::container(row)
            .padding(PADDING)
            .width(Length::Fixed(APPLET_WIDTH))
            .height(Length::Shrink);

        self.core.applet.autosize_window(container).into()
    }

    fn view_window(&self, _id: cosmic::iced::window::Id) -> Element<'_, Msg> {
        widget::text("").into()
    }
}

fn icon_btn(name: &str, msg: Msg) -> impl Into<Element<'_, Msg>> {
    widget::button::icon(widget::icon::from_name(name).size(TEXT_SIZE))
        .on_press(msg)
        .padding(2)
        .width(Length::Fixed(BTN_SIZE))
        .height(Length::Fixed(BTN_SIZE))
        .class(cosmic::style::Button::AppletIcon)
}

fn fetch_track_task() -> Task<Action<Msg>> {
    Task::perform(mpris::fetch_track(), |info| {
        Action::App(Msg::TrackUpdated(info))
    })
}

fn send_and_refresh(cmd: Cmd) -> Task<Action<Msg>> {
    Task::perform(
        async move {
            let _ = mpris::send_command(cmd).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            mpris::fetch_track().await
        },
        |info| Action::App(Msg::TrackUpdated(info)),
    )
}
