use iced::border::Radius;
use iced::theme;
use iced::widget::{button, container, scrollable, text::Text};
use iced::Border;
use iced::{alignment, Font};
use iced::{Background::Color as BackgroundColor, Color};

pub const TEXT_CONST: Font = Font::with_name("CaskaydiaCove Nerd Font");
pub const LINK: Font = Font::with_name("FiraMono Nerd Font");

fn icon(unicode: char) -> Text<'static> {
    Text::new(unicode.to_string())
        .font(TEXT_CONST)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
}

pub fn info_icon() -> Text<'static> {
    icon('\u{f059f}')
}

pub fn close_icon() -> Text<'static> {
    icon('\u{f00d}')
}

pub struct ScrollTheme;

impl scrollable::StyleSheet for ScrollTheme {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> scrollable::Appearance {
        scrollable::Appearance {
            scrollbar: scrollable::Scrollbar {
                scroller: scrollable::Scroller {
                    color: Color::TRANSPARENT,
                    border: Border::default(),
                },
                border: Border::default(),
                background: Some(BackgroundColor(Color::TRANSPARENT)),
            },
            ..style.active(&theme::Scrollable::default())
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> scrollable::Appearance {
        scrollable::Appearance {
            scrollbar: scrollable::Scrollbar {
                scroller: scrollable::Scroller {
                    color: Color::TRANSPARENT,
                    border: Border::default(),
                },
                border: Border::default(),
                background: Some(BackgroundColor(Color::TRANSPARENT)),
            },
            ..style.active(&theme::Scrollable::default())
        }
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Appearance {
        scrollable::Appearance {
            ..style.active(&theme::Scrollable::default())
        }
    }
}

pub struct BtnStyle;

impl button::StyleSheet for BtnStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            ..Default::default()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            ..Default::default()
        }
    }
}
pub struct DisabledBtnStyle;

impl button::StyleSheet for DisabledBtnStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::from_rgba(0.0, 0.0, 0.0, 0.7),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::from_rgba(0.0, 0.0, 0.0, 0.7),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::from_rgba(0.0, 0.0, 0.0, 0.7),
            ..Default::default()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::from_rgba(0.0, 0.0, 0.0, 0.7),
            ..Default::default()
        }
    }
}
pub struct InstallBtnStyle;

impl button::StyleSheet for InstallBtnStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            border: Border {
                radius: Radius::from(15),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::from_rgba(0.0, 1.0, 0.0, 1.0))),
            text_color: Color::BLACK,
            border: Border {
                radius: Radius::from(15),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::from_rgba(0.0, 1.0, 0.0, 1.0))),
            text_color: Color::BLACK,
            border: Border {
                radius: Radius::from(15),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            border: Border {
                radius: Radius::from(15),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
pub struct UninstallBtnStyle;

impl button::StyleSheet for UninstallBtnStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            border: Border {
                radius: Radius::from(15),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::from_rgba(1.0, 0.0, 0.0, 1.0))),
            text_color: Color::WHITE,
            border: Border {
                radius: Radius::from(15),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::from_rgba(1.0, 0.0, 0.0, 1.0))),
            text_color: Color::WHITE,
            border: Border {
                radius: Radius::from(15),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            border: Border {
                radius: Radius::from(15),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
pub struct CloseBtnStyle;

impl button::StyleSheet for CloseBtnStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            border: iced::Border::with_radius(100.0),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::from_rgba(1.0, 0.0, 0.0, 1.0))),
            text_color: Color::BLACK,
            border: iced::Border::with_radius(100.0),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::from_rgba(1.0, 0.0, 0.0, 1.0))),
            text_color: Color::BLACK,
            border: iced::Border::with_radius(100.0),
            ..Default::default()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            border: iced::Border::with_radius(100.0),
            ..Default::default()
        }
    }
}
pub struct EntryStyle;

impl container::StyleSheet for EntryStyle {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::new(0.5, 0.5, 0.5, 1.0)),
            background: Some(BackgroundColor(Color::new(0.0, 1.0, 0.0, 0.5))),
            border: iced::Border {
                radius: iced::border::Radius::from(5),
                ..Default::default()
            },
            ..style.appearance(&theme::Container::default())
        }
    }
}
pub struct ExpandedEntryStyle;

impl container::StyleSheet for ExpandedEntryStyle {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::new(0.5, 0.5, 0.5, 1.0)),
            background: Some(BackgroundColor(Color::WHITE)),
            ..style.appearance(&theme::Container::default())
        }
    }
}

pub struct LPanelStyle;

impl container::StyleSheet for LPanelStyle {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::BLACK),
            background: Some(BackgroundColor(Color::from_rgb(0.87, 0.87, 0.87))),
            ..style.appearance(&iced::theme::Container::default())
        }
    }
}
