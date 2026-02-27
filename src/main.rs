pub mod engine;
pub mod ui;
pub mod config;
pub mod strings;

use ui::RusticSortApp;
use iced::application;
use iced::window;

fn main() -> iced::Result {
    let icon_data = include_bytes!("../assets/icons/rusticsort-64.png");
    let icon = window::icon::from_file_data(icon_data, None).ok();

    let mut app = application(RusticSortApp::title, RusticSortApp::update, RusticSortApp::view)
        .window_size((800.0, 600.0))
        .font(include_bytes!("../assets/fonts/NotoSansArabic-Regular.ttf").as_slice());

    if let Some(icon) = icon {
        app = app.window(window::Settings {
            icon: Some(icon),
            ..Default::default()
        });
    }

    app.run_with(RusticSortApp::new)
}
