use iced::alignment::{self, Alignment};
use iced::font::{Family, Style, Weight};
use iced::theme::{self, Container as ContainerTheme};
use iced::widget::{button, column, container, keyed_column, row, text, Container, Text};
use iced::{
    window, Application, Background::Color as BackgroundColor, Color, Command, Font, Pixels,
    Result as IcedResult, Settings, Shadow, Size,
};
use iced::{Element, Length};

use serde::{Deserialize, Serialize};
use std::fs;

const ENTRIES_LIST: &str = "/home/varuos/.config/dev_helper/entries.json";

fn main() -> IcedResult {
    let app_window_settings = Settings {
        window: window::Settings {
            min_size: Some(Size {
                width: 1000.0,
                height: 700.0,
            }),
            position: window::Position::Centered,
            ..Default::default()
        },
        default_font: Font {
            family: Family::Name("CaskaydiaCove Nerd Font"),
            weight: Weight::Normal,
            style: Style::Normal,
            ..Default::default()
        },
        antialiasing: true,
        default_text_size: Pixels::from(18.0),
        fonts: vec![include_bytes!("../fonts/regular.ttf").as_slice().into()],
        ..Default::default()
    };

    AppObj::run(app_window_settings)
}

#[derive(Debug, Clone)]
enum Message {
    FilterChanged(Filter),
    ExpandedEntryMessage(usize, ExpandedEntryMessages),
    EntryMessage(usize, EntryMessages),
}

struct AppObj {
    filter: Filter,
    seleced_entity: Vec<Entries>,
    entries: Vec<Entries>,
}

impl Application for AppObj {
    type Theme = iced::Theme;
    type Flags = ();
    type Message = Message;
    type Executor = iced::executor::Default;

    fn title(&self) -> String {
        String::from("DeveL")
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                filter: Filter::All,
                seleced_entity: vec![],
                entries: Entries::load(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        #[tokio::main]
        async fn install(package_name: &str) {
            perform(package_name, TaskType::Install).await;
        }

        #[tokio::main]
        async fn uninstall(package_name: &str) {
            perform(package_name, TaskType::Uninstall).await;
        }

        match message {
            Message::FilterChanged(filter) => self.filter = filter,
            Message::EntryMessage(i, EntryMessages::Select) => {
                self.seleced_entity = vec![self.entries[i].clone()];
            }
            Message::EntryMessage(i, entry_message) => {
                if let Some(entity) = self.entries.get_mut(i) {
                    entity.update(entry_message);
                }
            }
            Message::ExpandedEntryMessage(i, ExpandedEntryMessages::Install) => {
                install(&self.seleced_entity[i].package_name);
            }
            Message::ExpandedEntryMessage(i, ExpandedEntryMessages::UnInstall) => {
                uninstall(&self.seleced_entity[i].package_name);
            }
            Message::ExpandedEntryMessage(_, ExpandedEntryMessages::Close) => {
                self.seleced_entity = vec![];
            }
            Message::ExpandedEntryMessage(i, entry_message) => {
                if let Some(entity) = self.seleced_entity.get_mut(i) {
                    entity.expanded_update(entry_message);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let title = text("DeveL")
            .size(80)
            .height(200)
            .width(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center);

        let controls = view_controls(&self.entries, self.filter);
        let filtered_entries = self
            .entries
            .iter()
            .filter(|entries| self.filter.matches(entries));

        let entries: Element<_> = if filtered_entries.count() > 0 {
            keyed_column(
                self.entries
                    .iter()
                    .enumerate()
                    .filter(|(_, entity)| self.filter.matches(entity))
                    .map(|(i, entity)| {
                        (
                            entity.id,
                            entity
                                .view()
                                .map(move |message| Message::EntryMessage(i, message)),
                        )
                    }),
            )
            .spacing(10)
            .into()
        } else {
            empty_message(match self.filter {
                Filter::All => "Nothing to Install!",
                Filter::Notinstalled => "Bucket is empty!",
                Filter::Installed => "Install Something!",
            })
        };

        let lpanel = Container::new(column![title, controls, entries].spacing(20))
            .width(500.0)
            .height(Length::Fill)
            .padding(18.0)
            .style(ContainerTheme::Custom(Box::new(LPanelStyle)));

        let filtered_enpanded_entries = self.seleced_entity.iter();
        let expanded_entries: iced::Element<_> = if filtered_enpanded_entries.count() > 0 {
            keyed_column(self.seleced_entity.iter().enumerate().map(|(i, entry)| {
                (
                    entry.id,
                    entry
                        .expanded_view()
                        .map(move |message| Message::ExpandedEntryMessage(i, message)),
                )
            }))
            .into()
        } else {
            empty_expanded_msg("Nothing to install")
        };

        let rpanel = Container::new(column![expanded_entries].padding(10).spacing(10))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10.0);

        row![lpanel, rpanel].into()
    }
}

fn empty_message(message: &str) -> iced::Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
    .center_x()
    .center_y()
    .height(200)
    .into()
}

fn empty_expanded_msg(message: &str) -> iced::Element<'_, Message> {
    container(text(message))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
}

fn view_controls(entities: &[Entries], current_filter: Filter) -> iced::Element<Message> {
    let packages_installed = entities.iter().filter(|entity| entity.installed).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label);

        let button = button(label).style(if filter == current_filter {
            iced::theme::Button::Positive
        } else {
            iced::theme::Button::Text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8.0)
    };

    let install_status = format!(
        "{packages_installed} {} Installed",
        if packages_installed == 1 {
            "Package"
        } else {
            "Packages"
        }
    );

    row![
        text(install_status).width(Length::Fill),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Not Installed", Filter::Notinstalled, current_filter),
            filter_button("Installed", Filter::Installed, current_filter),
        ]
        .width(Length::Shrink)
        .spacing(10)
    ]
    .spacing(20)
    .align_items(Alignment::Center)
    .into()
}

enum TaskType {
    Install,
    Uninstall,
}

async fn perform(package_name: &str, task: TaskType) {
    use async_process::{Command, Stdio};

    match task {
        TaskType::Install => {
            let _ = Command::new("alacritty")
                .arg("-e")
                .arg("pkexec")
                .arg("pacman")
                .arg("-S")
                .arg(package_name)
                .stdout(Stdio::piped())
                .spawn();
        }
        TaskType::Uninstall => {
            let _ = Command::new("alacritty")
                .arg("-e")
                .arg("pkexec")
                .arg("pacman")
                .arg("-S")
                .arg(package_name)
                .stdout(Stdio::piped())
                .spawn();
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Filter {
    #[default]
    All,
    Installed,
    Notinstalled,
}

impl Filter {
    fn matches(self, entity: &Entries) -> bool {
        match self {
            Filter::All => true,
            Filter::Installed => entity.installed,
            Filter::Notinstalled => !entity.installed,
        }
    }
}

#[derive(Clone, Debug)]
enum EntryMessages {
    Select,
    VisitWebsite,
}

#[derive(Clone, Debug)]
enum ExpandedEntryMessages {
    Install,
    UnInstall,
    VisitWebsite,
    Close,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Entries {
    id: usize,
    name: String,
    execute_cmd: String,
    package_name: String,
    desc: String,
    website: String,
    catagories: Vec<String>,

    #[serde(skip)]
    installed: bool,
}
impl Default for Entries {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            execute_cmd: "".to_string(),
            package_name: "".to_string(),
            desc: "".to_string(),
            website: "".to_string(),
            catagories: vec![],
            installed: false,
        }
    }
}

impl Entries {
    fn load() -> Vec<Entries> {
        let json = fs::read_to_string(ENTRIES_LIST).expect("Failed to read file!");

        let mut entry: Vec<Entries> =
            serde_json::from_str(&json).expect("Something is worng with json!");

        let status = |name| {
            use std::process::Command;
            let output = Command::new("pacman")
                .arg("-Q")
                .arg(name)
                .output()
                .expect("command not found!");

            output.status.success()
        };

        let mut result: Vec<Entries> = Vec::new();
        for entry in entry.iter_mut() {
            entry.installed = status(&entry.package_name);
            result.push(entry.clone());
        }

        return result;
    }

    fn update(&mut self, message: EntryMessages) {
        match message {
            EntryMessages::VisitWebsite => {
                let path = &self.website;
                match open::that(path) {
                    Ok(()) => println!("Opened '{}' successfully.", path),
                    Err(err) => panic!("An error occurred when opening '{}': {}", path, err),
                }
            }
            EntryMessages::Select => {}
        }
    }

    fn expanded_update(&mut self, message: ExpandedEntryMessages) {
        match message {
            ExpandedEntryMessages::Install => {}
            ExpandedEntryMessages::UnInstall => {}
            ExpandedEntryMessages::VisitWebsite => {
                let path = &self.website;
                match open::that(path) {
                    Ok(()) => println!("Opened '{}' successfully.", path),
                    Err(err) => panic!("An error occurred when opening '{}': {}", path, err),
                }
            }
            ExpandedEntryMessages::Close => {}
        }
    }

    fn view(&self) -> iced::Element<EntryMessages> {
        let m_button = button(text(&self.name))
            .on_press(EntryMessages::Select)
            .style(iced::theme::Button::Text)
            .width(Length::Fill);

        container(row![
            m_button,
            button(info_icon())
                .on_press(EntryMessages::VisitWebsite)
                .style(theme::Button::Custom(Box::new(BtnStyle)))
        ])
        .padding(8)
        .style(ContainerTheme::Custom(Box::new(EntryStyle)))
        .into()
    }

    fn expanded_view(&self) -> iced::Element<ExpandedEntryMessages> {
        let title = text(&self.name)
            .width(Length::Fill)
            .style(Color::from_rgb(0.0, 0.0, 0.0))
            .size(20);

        let perform = button(if self.installed == true {
            delete_icon()
        } else {
            download_icon()
        })
        .on_press(if self.installed == true {
            ExpandedEntryMessages::UnInstall
        } else {
            ExpandedEntryMessages::Install
        })
        .style(if self.installed == true {
            theme::Button::Custom(Box::new(UninstallBtnStyle))
        } else {
            theme::Button::Custom(Box::new(InstallBtnStyle))
        });

        let visit = button(info_icon())
            .style(theme::Button::Custom(Box::new(BtnStyle)))
            .on_press(ExpandedEntryMessages::VisitWebsite);

        let close = button(close_icon())
            .on_press(ExpandedEntryMessages::Close)
            .style(theme::Button::Custom(Box::new(CloseBtnStyle)));
        container(
            column![
                row![title, perform, visit, close]
                    .spacing(10)
                    .width(Length::Fill),
                text(&self.desc)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .size(20),
            ]
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .padding(8)
        .height(Length::Fill)
        .width(Length::Fill)
        .style(ContainerTheme::Custom(Box::new(ExpandedEntryStyle)))
        .into()
    }
}

const ICONS: Font = Font::with_name("CaskaydiaCove Nerd Font");

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
}

fn info_icon() -> Text<'static> {
    icon('\u{f05a}')
}

fn download_icon() -> Text<'static> {
    icon('\u{f0ed}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{f09e7}')
}

fn close_icon() -> Text<'static> {
    icon('\u{f00d}')
}

struct BtnStyle;

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

struct InstallBtnStyle;

impl button::StyleSheet for InstallBtnStyle {
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
            text_color: Color::from_rgba(0.0, 1.0, 0.0, 0.5),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::from_rgba(0.0, 1.0, 0.0, 0.5),
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

struct UninstallBtnStyle;

impl button::StyleSheet for UninstallBtnStyle {
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
            text_color: Color::from_rgba(1.0, 0.0, 0.0, 0.5),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BackgroundColor(Color::TRANSPARENT)),
            text_color: Color::from_rgba(1.0, 0.0, 0.0, 0.5),
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

struct CloseBtnStyle;

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
struct EntryStyle;

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
            shadow: Shadow {
                color: Color::BLACK,
                offset: iced::Vector { x: 0.0, y: 0.0 },
                blur_radius: 5.0,
            },
            ..style.appearance(&theme::Container::default())
        }
    }
}

struct ExpandedEntryStyle;

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

struct LPanelStyle;

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
