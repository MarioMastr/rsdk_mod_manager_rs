use crate::{core::{json::ManagerSettings, rsdk::{ModInfo, RSDKInfo}}, ui::iced::{Message, Tab}};
use iced::{
    Alignment, Element, font, widget::{Column, Container, Row, button, checkbox, row}
};
use iced_aw::tab_bar::TabLabel;
use iced::widget::{
    center_x, center_y, column, container, scrollable, table, text
};
use iced::Font;

#[derive(Default)]
pub struct ModsTab {
    game: RSDKInfo,
    manager: ManagerSettings
}

#[derive(Debug, Clone)]
pub enum ModsMessage {
    EnableAll,
    DisableAll,
    Checkbox(bool),
    Save
}

impl ModsTab {
    pub fn new(game: RSDKInfo, manager: ManagerSettings) -> Self {
        Self {
            game,
            manager
        }
    }

    pub fn update(&mut self, message: ModsMessage) {
        match message {
            ModsMessage::EnableAll => {
                for mi in &mut self.game.mods {
                    mi.enabled = true;
                }
            },
            ModsMessage::DisableAll => {
                for mi in &mut self.game.mods {
                    mi.enabled = false;
                }
            },
            ModsMessage::Checkbox(_state) => {

            }
            ModsMessage::Save => {
                self.game.save().expect("Unable to save changes");
            }
        }
    }
}

impl Tab for ModsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(String::from("Mods"))
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let table = {
            let bold = |header| {
                text(header).font(Font {
                    weight: font::Weight::Bold,
                    ..Font::DEFAULT
                })
            };

            let columns = [
                table::column(bold("Name"), |mi: &ModInfo| {
                    text(&mi.name)
                }),
                table::column(bold("Author"), |mi: &ModInfo| text(&mi.author)),
                table::column(bold("Version"), |mi: &ModInfo| text(&mi.version))
            ];

            table(columns, &self.game.mods)
            .padding_x(10.0)
            .padding_y(5.0)
            .separator_x(1.0)
            .separator_y(1.0)
        };

        let controls = {
            let labeled_button =
            |label,
            on_change: Message| {
                button(label).on_press(on_change)
            };

            column![
                labeled_button("Enable All", Message::Mods(ModsMessage::EnableAll)),
                labeled_button("Disable All", Message::Mods(ModsMessage::DisableAll)),
                labeled_button("Save", Message::Mods(ModsMessage::Save))
            ]
            .spacing(10)
            .height(400)
        };

        Container::new(
            Column::new()
            .align_x(Alignment::Center)
            .max_width(600)
            .padding(20)
            .spacing(16)
            .push(
                column![
                    center_y(scrollable(center_x(table)).spacing(10)).padding(10),
                    center_x(controls).padding(10).style(container::dark)
                ]
            )
        ).into()
    }
}
