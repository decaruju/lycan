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
pub enum GameJoinChoice {
    Back,
    Game(String),
}

pub fn game_join(window: &mut RenderWindow, font: &Font) -> GameJoinChoice {
    let mut menu = Menu::new();
    menu.add_widget(
        Label::new(
            "Enter the game identifier",
            &font,
            String::from("welcome"),
        ),
    );
    menu.add_widget(
        TextField::new(
            &font,
            String::from("game_id")
        ),
    );
    menu.add_widget(
        Button::new(
            "Join game",
            &font,
            GameJoinChoice::Game(String::from("")),
            String::from("join"),
        ),
    );
    menu.add_widget(
        Button::new(
            "Back",
            &font,
            GameJoinChoice::Back,
            String::from("back"),
        ),
    );
    match menu.handle(window) {
        GameJoinChoice::Back => GameJoinChoice::Back,
        GameJoinChoice::Game(_) => GameJoinChoice::Game(menu.data().get("game_id").unwrap().to_string()),
    }
}
