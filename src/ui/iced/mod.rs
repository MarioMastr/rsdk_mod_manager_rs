pub mod mods_tab;
pub mod options_tab;
pub mod utils;

use mods_tab::{ModsMessage, ModsTab};
use utils::MyTheme;
use iced::{
    Length, Element,
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text, pane_grid},
};
use iced_aw::{TabLabel, Tabs};

use crate::core::{json::ManagerSettings, rsdk::RSDKInfo};

pub struct RMM {
    active_tab: TabId,
    mods_tab: ModsTab,
    // options_tab: OptionsTab,
    game: RSDKInfo,
    manager: ManagerSettings,
    panes: pane_grid::State<PaneID>,
    selected_theme: MyTheme
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum TabId {
    #[default]
    Mods,
    Options,
}

#[derive(Clone, Copy)]
pub enum PaneID {
    Sidebar,
    Content,
}

#[derive(Clone, Debug)]
pub enum Message {
    TabSelected(TabId),
    Mods(ModsMessage),
    // Options(OptionsMessage),
    TabClosed(TabId),
}

impl RMM {
    pub fn new() -> Self {
        let manager = ManagerSettings::read_json().expect("Unable to read/create managerSettings.json");
        let game = RSDKInfo::get(&manager).expect("Unable to get information on selected game");
        let mods_tab = ModsTab::new(game.clone(), manager.clone());

        let (mut panes, first_pane) = pane_grid::State::new(PaneID::Sidebar);
        let split = panes.split(pane_grid::Axis::Vertical, first_pane, PaneID::Content);
        panes.resize(split.unwrap().1, 0.2);

        Self {
            active_tab: TabId::Mods,
            mods_tab,
            game,
            manager,
            panes,
            selected_theme: MyTheme::Dark,
        }
    }

    pub fn title(&self) -> String {
        "RSDK Mod Manager".to_string()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Mods(message) => self.mods_tab.update(message),
            // Message::Options(message) => self.options_tab.update(message),
            Message::TabClosed(id) => println!("Tab {:?} event hit", id),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        Tabs::new(Message::TabSelected)
            .tab_icon_position(iced_aw::tabs::Position::Bottom)
            .on_close(Message::TabClosed)
            .push(
                TabId::Mods,
                self.mods_tab.tab_label(),
                self.mods_tab.view(),
            )
            .set_active_tab(&self.active_tab)
            .into()
    }   
}

pub trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(33))
            .push(self.content())
            .align_x(iced::Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(33)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
