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
};

#[derive(Clone)]
pub enum MainMenuChoice {
    Quit,
    NewGame,
    JoinGame,
}

pub fn main_menu(window: &mut RenderWindow, font: &Font) -> MainMenuChoice {
    let mut menu = Menu::new();
    menu.add_widget(
        Label::new(
            "WELCOME TO LYCAN",
            &font,
            String::from("welcome"),
        ),
    );
    menu.add_widget(
        Button::new(
            "New game",
            &font,
            MainMenuChoice::NewGame,
            String::from("new"),
        ),
    );
    menu.add_widget(
        Button::new(
            "Join game",
            &font,
            MainMenuChoice::JoinGame,
            String::from("join"),
        ),
    );
    menu.add_widget(
        Button::new(
            "Quit",
            &font,
            MainMenuChoice::Quit,
            String::from("quit"),
        ),
    );
    menu.handle(window)
}
