use iced::widget::{button, container};
use iced::{Background, Border, Color, Theme};

// Color palette
const ACCENT: Color = Color::from_rgb(0.18, 0.55, 0.82);
const ACCENT_HOVER: Color = Color::from_rgb(0.22, 0.65, 0.95);
const DANGER: Color = Color::from_rgb(0.85, 0.25, 0.25);
const DANGER_HOVER: Color = Color::from_rgb(0.95, 0.35, 0.35);
const SURFACE: Color = Color::from_rgb(0.94, 0.95, 0.96);
const HEADER_BG: Color = Color::from_rgb(0.12, 0.14, 0.18);

pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(ACCENT)),
        text_color: Color::WHITE,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        shadow: Default::default(),
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(ACCENT_HOVER)),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.14, 0.45, 0.72))),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.6, 0.7, 0.8))),
            text_color: Color::from_rgb(0.85, 0.85, 0.85),
            ..base
        },
        _ => base,
    }
}

pub fn danger_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(DANGER)),
        text_color: Color::WHITE,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        shadow: Default::default(),
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(DANGER_HOVER)),
            ..base
        },
        _ => base,
    }
}

pub fn outline_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::WHITE)),
        text_color: Color::from_rgb(0.3, 0.3, 0.4),
        border: Border {
            color: Color::from_rgb(0.8, 0.82, 0.85),
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Default::default(),
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.96, 0.97, 0.98))),
            border: Border {
                color: ACCENT,
                width: 1.0,
                radius: 6.0.into(),
            },
            text_color: ACCENT,
            ..base
        },
        _ => base,
    }
}

pub fn header_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(HEADER_BG)),
        text_color: Some(Color::WHITE),
        border: Border::default(),
        shadow: Default::default(),
    }
}

pub fn card_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            color: Color::from_rgb(0.88, 0.89, 0.92),
            width: 1.0,
            radius: 10.0.into(),
        },
        shadow: Default::default(),
        text_color: None,
    }
}

pub fn main_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border::default(),
        shadow: Default::default(),
        text_color: None,
    }
}

pub fn modal_overlay(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
        border: Border::default(),
        shadow: Default::default(),
        text_color: None,
    }
}

pub fn modal_card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            color: Color::from_rgb(0.85, 0.86, 0.9),
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Default::default(),
        text_color: None,
    }
}

pub fn success_card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.92, 0.98, 0.92))),
        border: Border {
            color: Color::from_rgb(0.6, 0.85, 0.6),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Default::default(),
        text_color: None,
    }
}

pub fn error_card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.99, 0.93, 0.93))),
        border: Border {
            color: Color::from_rgb(0.9, 0.6, 0.6),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Default::default(),
        text_color: None,
    }
}

pub fn ghost_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: None,
        text_color: Color::from_rgb(0.5, 0.52, 0.56),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 4.0.into(),
        },
        shadow: Default::default(),
    };
    match status {
        button::Status::Hovered => button::Style {
            text_color: ACCENT,
            background: Some(Background::Color(Color::from_rgb(0.92, 0.94, 0.96))),
            ..base
        },
        _ => base,
    }
}
