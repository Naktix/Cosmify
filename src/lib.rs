mod app;
mod mpris;

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<app::Cosmify>(())
}
