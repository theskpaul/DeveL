mod decorations;
mod entries;

use iced::alignment::{self, Alignment};
use iced::font::{Family, Style, Weight};
use iced::theme::{self, Container as ContainerTheme};
use iced::widget::{
    button, column, container, horizontal_space, keyed_column, row, text, Container,
};
use iced::{window, Application, Command, Font, Pixels, Result as IcedResult, Settings, Size};
use iced::{Element, Length};

use decorations::*;
use entries::{EMessages, Entries, ExpandedEMessages};

// TODO: Taskes need to do.
// 1. Make it modular
// 2. Reduce startup time
// 3. Improve UI, Make it more responsive
// 4. Improve UX also
// 5. Display package catagories and show them horizontally

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
        default_text_size: Pixels::from(16.5),
        fonts: vec![
            include_bytes!("../fonts/regular.ttf").as_slice().into(),
            include_bytes!("../fonts/bold.otf").as_slice().into(),
        ],
        ..Default::default()
    };

    AppObj::run(app_window_settings)
}

#[derive(Debug, Clone)]
enum Message {
    FilterChanged(Filter),
    ExpandedEntryMessage(usize, ExpandedEMessages),
    EntryMessage(usize, EMessages),
    PrevPage,
    NextPage,
}

struct AppObj {
    filter: Filter,
    seleced_entity: Vec<Entries>,
    entries: Vec<Vec<Entries>>,
    current_index: usize,
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
                current_index: 0,
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        #[tokio::main]
        async fn install(package_name: &str) {
            perform(TaskType::Install(package_name.to_string())).await;
        }

        #[tokio::main]
        async fn alt_install(install_cmd: &str) {
            perform(TaskType::AltInstall(install_cmd.to_string())).await;
        }

        #[tokio::main]
        async fn uninstall(package_name: &str) {
            perform(TaskType::Uninstall(package_name.to_string())).await;
        }

        #[tokio::main]
        async fn alt_uninstall(uninstall_cmd: &str) {
            perform(TaskType::AltUninstall(uninstall_cmd.to_string())).await;
        }

        match message {
            Message::FilterChanged(filter) => self.filter = filter,

            Message::PrevPage => {
                if self.current_index != 0 {
                    self.current_index -= 1;
                }
            }

            Message::NextPage => {
                let len = self.entries.len();
                if self.current_index != len - 1 {
                    self.current_index += 1;
                }
            }

            Message::EntryMessage(i, EMessages::Select) => {
                self.seleced_entity = vec![self.entries[self.current_index][i].clone()];
            }
            Message::EntryMessage(i, entry_message) => {
                if let Some(entity) = self.entries[self.current_index].get_mut(i) {
                    entity.update(entry_message);
                }
            }

            Message::ExpandedEntryMessage(i, ExpandedEMessages::Install) => {
                let pname = &self.seleced_entity[i].package_name;
                let icmd = &self.seleced_entity[i].install_cmd;

                if !pname.is_empty() {
                    install(pname);
                } else {
                    alt_install(icmd);
                }
            }

            Message::ExpandedEntryMessage(i, ExpandedEMessages::UnInstall) => {
                let pname = &self.seleced_entity[i].package_name;
                let ucmd = &self.seleced_entity[i].uninstall_cmd;

                if !pname.is_empty() {
                    uninstall(pname);
                } else {
                    alt_uninstall(ucmd);
                }
            }

            Message::ExpandedEntryMessage(_, ExpandedEMessages::Close) => {
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
            .height(Length::Fill)
            .width(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center);

        let controls = view_controls(self.filter);
        let filtered_entries = self.entries[self.current_index]
            .iter()
            .filter(|entries| self.filter.matches(entries));

        let entries: Element<_> = if filtered_entries.count() > 0 {
            keyed_column(
                self.entries[self.current_index]
                    .iter()
                    .enumerate()
                    .filter(|(_, entries)| self.filter.matches(entries))
                    .map(|(i, entries)| {
                        (
                            entries.id,
                            entries
                                .view()
                                .map(move |message| Message::EntryMessage(i, message)),
                        )
                    }),
            )
            .spacing(11)
            .height(294)
            .into()
        } else {
            empty_message(match self.filter {
                Filter::All => "Nothing to Install!",
                Filter::Notinstalled => "Bucket is empty!",
                Filter::Installed => "Install Something!",
            })
        };

        let prev_entries = button("PREV")
            .height(35)
            .style(if self.current_index == 0 {
                theme::Button::Custom(Box::new(DisabledBtnStyle))
            } else {
                theme::Button::Custom(Box::new(BtnStyle))
            })
            .on_press(Message::PrevPage);

        let next_entries = button("NEXT")
            .height(35)
            .style(if self.current_index == self.entries.len() - 1 {
                theme::Button::Custom(Box::new(DisabledBtnStyle))
            } else {
                theme::Button::Custom(Box::new(BtnStyle))
            })
            .on_press(Message::NextPage);

        let entries_group = column![
            entries,
            row![prev_entries, horizontal_space(), next_entries].align_items(Alignment::Center)
        ]
        .spacing(20);

        let lpanel = Container::new(column![title, controls, entries_group].spacing(20))
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

fn empty_learn_btns<'a>() -> iced::Element<'a, ExpandedEMessages> {
    container("").into()
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
    .height(294)
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

fn view_controls<'a>(current_filter: Filter) -> iced::Element<'a, Message> {
    let filter_button = |label, filter, current_filter| {
        let label = text(label);

        let button = button(label).style(if filter == current_filter {
            iced::theme::Button::Positive
        } else {
            iced::theme::Button::Text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8.0)
    };

    row![
        horizontal_space(),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Not Installed", Filter::Notinstalled, current_filter),
            filter_button("Installed", Filter::Installed, current_filter),
        ]
        .width(Length::Shrink)
        .spacing(8)
    ]
    .spacing(13)
    .align_items(Alignment::Center)
    .into()
}

enum TaskType {
    Install(String),
    Uninstall(String),
    AltInstall(String),
    AltUninstall(String),
}

async fn perform(task: TaskType) {
    use async_process::{Command, Stdio};

    match task {
        TaskType::Install(package_name) => {
            let _ = Command::new("alacritty")
                .arg("-e")
                .arg("pkexec")
                .arg("pacman")
                .arg("-S")
                .arg(package_name)
                .stdout(Stdio::piped())
                .spawn();
        }
        TaskType::Uninstall(package_name) => {
            let _ = Command::new("alacritty")
                .arg("-e")
                .arg("pkexec")
                .arg("pacman")
                .arg("-R")
                .arg(package_name)
                .stdout(Stdio::piped())
                .spawn();
        }
        TaskType::AltInstall(cmd) => {
            let args: Vec<&str> = cmd.split_whitespace().collect();
            let _ = Command::new("alacritty")
                .arg("-e")
                .args(args)
                .stdout(Stdio::piped())
                .spawn();
        }
        TaskType::AltUninstall(cmd) => {
            let args: Vec<&str> = cmd.split_whitespace().collect();
            let _ = Command::new("alacritty")
                .arg("-e")
                .args(args)
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
