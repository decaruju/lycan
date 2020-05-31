use sfml::{
    graphics::{
        Font,
        RenderWindow,
    },
};

use crate::menu::{
    menu::Menu,
    button::Button,
    label::Label,
    text_field::TextField,
};

#[derive(Clone)]
pub enum NameEntryChoice {
    Back,
    Name(String),
}

pub fn name_entry(window: &mut RenderWindow, font: &Font) -> NameEntryChoice {
    let mut menu = Menu::new();
    menu.add_widget(
        Label::new(
            "Enter your player name",
            &font,
            String::from("welcome"),
        ),
    );
    menu.add_widget(
        TextField::new(
            &font,
            String::from("player_name"),
        ),
    );
    menu.add_widget(
        Button::new(
            "Join game",
            &font,
            NameEntryChoice::Name(String::from("")),
            String::from("join"),
        ),
    );
    menu.add_widget(
        Button::new(
            "Back",
            &font,
            NameEntryChoice::Back,
            String::from("back"),
        ),
    );
    match menu.handle(window) {
        NameEntryChoice::Back => NameEntryChoice::Back,
        NameEntryChoice::Name(_) => NameEntryChoice::Name(menu.data().get("player_name").unwrap().to_string()),
    }
}
