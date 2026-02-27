pub mod engine;
pub mod ui;
pub mod config;

use iced::{application, Settings};
use ui::RusticSortApp;

fn main() -> iced::Result {
    // Starting the Iced application with version 0.13 window settings builder
    application(RusticSortApp::title, RusticSortApp::update, RusticSortApp::view)
        .window_size((800.0, 600.0))
        .default_font(iced::Font::with_name("Noto Sans Arabic"))
        .font(include_bytes!("../assets/fonts/NotoSansArabic-Regular.ttf").as_slice())
        .run_with(RusticSortApp::new)
}
