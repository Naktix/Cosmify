use cosmic::app::Core;
use cosmic::iced::platform_specific::shell::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::widget::image;
use cosmic::iced::window::Id;
use cosmic::iced::{Alignment, Length, Limits, Subscription};
use cosmic::widget;
use cosmic::{Action, Element, Task};

use crate::mpris::{self, Cmd, TrackInfo};

const APP_ID: &str = "dev.naktix.Cosmify";

const APPLET_WIDTH: f32 = 140.0;
const ROW_HEIGHT: f32 = 32.0;
const BTN_SIZE: f32 = 30.0;
const PADDING: [f32; 2] = [3.0, 6.0];
const SPACING: f32 = 6.0;
const TEXT_SIZE: u16 = 12;

const POPUP_WIDTH: u32 = 290;
const POPUP_HEIGHT: u32 = 290;

const ICON_PREV: &str = "media-skip-backward-symbolic";
const ICON_NEXT: &str = "media-skip-forward-symbolic";
const ICON_PLAY: &str = "media-playback-start-symbolic";
const ICON_PAUSE: &str = "media-playback-pause-symbolic";
const ICON_POPUP: &str = "media-optical-symbolic";

pub struct Cosmify {
    core: Core,
    track: TrackInfo,
    popup: Option<Id>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    PlayPause,
    Next,
    Previous,
    Tick,
    TrackUpdated(TrackInfo),
    TogglePopup,
    PopupClosed(Id),
}

impl cosmic::Application for Cosmify {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Msg;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: ()) -> (Self, Task<Action<Msg>>) {
        let app = Cosmify {
            core,
            track: TrackInfo::default(),
            popup: None,
        };
        (app, fetch_track_task())
    }

    fn on_close_requested(&self, id: Id) -> Option<Msg> {
        Some(Msg::PopupClosed(id))
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
            Msg::TogglePopup => {
                if let Some(id) = self.popup.take() {
                    return destroy_popup(id);
                }

                let new_id = Id::unique();
                self.popup = Some(new_id);

                let mut popup_settings = self.core.applet.get_popup_settings(
                    self.core.main_window_id().unwrap(),
                    new_id,
                    None,
                    None,
                    None,
                );

                popup_settings.positioner.size_limits = Limits::NONE
                    .min_width(POPUP_WIDTH as f32)
                    .max_width(POPUP_WIDTH as f32)
                    .min_height(POPUP_HEIGHT as f32)
                    .max_height(POPUP_HEIGHT as f32);

                get_popup(popup_settings)
            }
            Msg::PopupClosed(id) => {
                if self.popup == Some(id) {
                    self.popup = None;
                }
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

        let prev_btn = icon_btn(ICON_PREV, Msg::Previous);
        let play_btn = icon_btn(play_icon, Msg::PlayPause);
        let next_btn = icon_btn(ICON_NEXT, Msg::Next);

        let popup_btn = widget::button::icon(widget::icon::from_name(ICON_POPUP).size(TEXT_SIZE))
            .padding(2)
            .width(Length::Fixed(BTN_SIZE))
            .height(Length::Fixed(BTN_SIZE))
            .class(cosmic::style::Button::AppletIcon)
            .on_press(Msg::TogglePopup);

        let buttons = widget::row()
            .push(prev_btn.into())
            .push(play_btn.into())
            .push(next_btn.into())
            .push(popup_btn)
            .spacing(SPACING)
            .align_y(Alignment::Center);

        let container = widget::container(buttons)
            .padding(PADDING)
            .width(Length::Fixed(APPLET_WIDTH))
            .height(Length::Fixed(ROW_HEIGHT));

        self.core.applet.autosize_window(container).into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Msg> {
        let mut popup_col = widget::column().spacing(10).align_x(Alignment::Center);

        if let Some(bytes) = &self.track.art_bytes {
            let handle = image::Handle::from_bytes(bytes.clone());
            popup_col = popup_col.push(
                image(handle)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(200.0)),
            );
        } else if let Some(path) = &self.track.art_url {
            let handle = image::Handle::from_path(path);
            popup_col = popup_col.push(
                image(handle)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(200.0)),
            );
        } else {
            popup_col = popup_col.push(
                widget::container(widget::icon::from_name("audio-x-generic-symbolic").size(64))
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(200.0))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            );
        }

        popup_col = popup_col
            .push(widget::text(&self.track.title).size(16))
            .push(widget::text(&self.track.artist).size(14));

        let content = widget::container(popup_col)
            .padding(20)
            .width(Length::Fixed(POPUP_WIDTH as f32))
            .height(Length::Fixed(POPUP_HEIGHT as f32))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        self.core.applet.popup_container(content).into()
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
