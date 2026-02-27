pub mod engine;
pub mod ui;
pub mod config;
pub mod strings;

use ui::RusticSortApp;
use iced::application;

fn main() -> iced::Result {
    application(RusticSortApp::title, RusticSortApp::update, RusticSortApp::view)
        .window_size((800.0, 600.0))
        .font(include_bytes!("../assets/fonts/NotoSansArabic-Regular.ttf").as_slice())
        .run_with(RusticSortApp::new)
}
