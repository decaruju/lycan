use sfml::{
    graphics::{
        Font,
        RenderWindow,
    },
};

use crate::menu::{
    menu::Menu,
    button::Button,
    text_field::TextField,
};

#[derive(Clone)]
pub enum NameEntryChoice {
    Back,
    Name(String),
}

pub fn name_entry(window: &mut RenderWindow, font: &Font) -> NameEntryChoice {
    let mut menu = Menu::new();
    menu.add_text_field(
        TextField::new(
            &font,
        ),
    );
    menu.add_button(
        Button::new(
            "Join game",
            &font,
            NameEntryChoice::Name(String::from("")),
        ),
    );
    menu.add_button(
        Button::new(
            "Back",
            &font,
            NameEntryChoice::Back,
        ),
    );
    match menu.handle(window) {
        NameEntryChoice::Back => NameEntryChoice::Back,
        NameEntryChoice::Name(_) => NameEntryChoice::Name(menu.text()),
    }
}
