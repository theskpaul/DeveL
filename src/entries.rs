use iced::theme::{
    self, Container as ContainerTheme, Scrollable as ScrollableTheme, Text as TextTheme,
};
use iced::widget::{button, column, container, keyed_column, row, scrollable, text};
use iced::Color;
use iced::Length;

use crate::decorations::*;
use crate::empty_learn_btns;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entries {
    pub name: String,
    pub executable_name: String,
    pub package_name: String,
    pub desc_file: String,
    pub website: String,
    pub catagories: Vec<String>,
    pub install_cmd: String,
    pub uninstall_cmd: String,
    pub learn_form: Vec<Site>,

    #[serde(skip)]
    pub id: usize,
    #[serde(skip)]
    pub installed: bool,
    #[serde(skip)]
    pub desc: String,
}

impl Entries {
    pub fn load() -> Vec<Vec<Entries>> {
        use std::env;
        use std::fs;
        use std::path::{Path, PathBuf};

        fn load_file_content(def_dir: &str, dir: &str, file: &str) -> String {
            let def_file = Path::new(def_dir).join(file);
            let file = Path::new(dir).join(file);

            let content = move |file_name| match fs::read_to_string(file_name) {
                Ok(val) => val,
                Err(_) => fs::read_to_string(def_file).expect("Reinstall The Program!"),
            };

            content(file)
        }

        let default_config_dir = format!("{}/assets/configs/", env!("CARGO_MANIFEST_DIR"));
        let user_config_dir = format!("{}/.config/dev_helper/", env!("HOME"));
        let json_file = load_file_content(&default_config_dir, &user_config_dir, "entries.json");

        let mut entry: Vec<Entries> =
            serde_json::from_str(&json_file).expect("Something is worng with json!");

        fn find_it<P>(executable_name: P) -> Option<PathBuf>
        where
            P: AsRef<Path>,
        {
            env::var_os("PATH").and_then(|paths| {
                env::split_paths(&paths)
                    .filter_map(|dir| {
                        let full_path = dir.join(&executable_name);

                        if full_path.is_file() {
                            Some(full_path)
                        } else {
                            None
                        }
                    })
                    .next()
            })
        }

        let mut result: Vec<Entries> = Vec::new();
        for (pos, entry) in entry.iter_mut().enumerate() {
            entry.installed = find_it(&entry.executable_name).is_some();

            let desc = load_file_content(&default_config_dir, &user_config_dir, &entry.desc_file);

            entry.id = pos;
            entry.desc = desc.trim().to_string();

            result.push(entry.clone());
        }

        let mut sliced_result: Vec<Vec<Entries>> = Vec::new();
        for slice in result.chunks(5) {
            sliced_result.push(slice.to_vec())
        }
        sliced_result
    }

    pub fn update(&mut self, message: EMessages) {
        match message {
            EMessages::VisitWebsite => {
                let path = &self.website;
                match open::that(path) {
                    Ok(()) => println!("Opened '{}' successfully.", path),
                    Err(err) => panic!("An error occurred when opening '{}': {}", path, err),
                }
            }
            EMessages::Select => {}
        }
    }

    pub fn expanded_update(&mut self, message: ExpandedEMessages) {
        match message {
            ExpandedEMessages::Install => {}
            ExpandedEMessages::UnInstall => {}
            ExpandedEMessages::Learn(i, site_msg) => {
                if let Some(site) = self.learn_form.get_mut(i) {
                    site.update(site_msg)
                }
            }
            ExpandedEMessages::Close => {}
        }
    }

    pub fn view(&self) -> iced::Element<EMessages> {
        let entry_btn = button(text(&self.name))
            .on_press(EMessages::Select)
            .style(iced::theme::Button::Text)
            .width(Length::Fill)
            .height(Length::Fill);

        let info_btn = button(info_icon())
            .on_press(EMessages::VisitWebsite)
            .height(50)
            .style(theme::Button::Custom(Box::new(BtnStyle)));

        container(row![entry_btn, info_btn])
            .padding(8)
            .height(50)
            .style(ContainerTheme::Custom(Box::new(EntryStyle)))
            .into()
    }

    pub fn expanded_view(&self) -> iced::Element<ExpandedEMessages> {
        let title = text(&self.name)
            .width(Length::Fill)
            .style(Color::from_rgb(0.0, 0.0, 0.0))
            .size(20);

        let perform = button(if self.installed {
            "Uninstall"
        } else {
            "Install"
        })
        .on_press(if self.installed {
            ExpandedEMessages::UnInstall
        } else {
            ExpandedEMessages::Install
        })
        .padding([5, 15])
        .style(if self.installed {
            theme::Button::Custom(Box::new(UninstallBtnStyle))
        } else {
            theme::Button::Custom(Box::new(InstallBtnStyle))
        });

        let close = button(close_icon())
            .on_press(ExpandedEMessages::Close)
            .style(theme::Button::Custom(Box::new(CloseBtnStyle)));

        let link_box: iced::Element<_> = if !self.learn_form.is_empty() {
            row![
                text("Learn from: ")
                    .width(Length::Shrink)
                    .style(TextTheme::Color(Color::BLACK))
                    .vertical_alignment(iced::alignment::Vertical::Center),
                keyed_column(self.learn_form.iter().enumerate().map(|(i, site)| {
                    (
                        site.id,
                        site.view()
                            .map(move |message| ExpandedEMessages::Learn(i, message)),
                    )
                }))
                .spacing(3)
            ]
            .width(Length::Fill)
            .into()
        } else {
            empty_learn_btns()
        };

        container(
            column![
                row![title, perform, close].spacing(10).width(Length::Fill),
                scrollable(
                    text(&self.desc)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .size(20)
                        .font(TEXT_CONST)
                )
                .height(Length::Fill)
                .direction(scrollable::Direction::Vertical(
                    scrollable::Properties::new()
                ))
                .style(ScrollableTheme::Custom(Box::new(ScrollTheme))),
                link_box
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

#[derive(Clone, Debug)]
pub enum EMessages {
    Select,
    VisitWebsite,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpandedEMessages {
    Install,
    UnInstall,
    Close,
    Learn(usize, SiteMessage),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    id: usize,
    website: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SiteMessage {
    Url,
}

impl Default for Site {
    fn default() -> Self {
        Site {
            id: 0,
            website: "".to_string(),
        }
    }
}

impl Site {
    fn update(&mut self, message: SiteMessage) {
        match message {
            SiteMessage::Url => {
                let path = &self.website;
                match open::that(path) {
                    Ok(()) => println!("Opened '{}' successfully.", path),
                    Err(err) => panic!("An error occurred when opening '{}': {}", path, err),
                }
            }
        }
    }

    fn view(&self) -> iced::Element<SiteMessage> {
        let ftext = format!("{} ", &self.website);
        let content = text(ftext)
            .style(TextTheme::Color(iced::Color::from_rgb(0.0, 0.0, 1.0)))
            .font(LINK);

        button(content)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(0)
            .style(theme::Button::Text)
            .on_press(SiteMessage::Url)
            .into()
    }
}
