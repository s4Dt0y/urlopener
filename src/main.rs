pub mod errors;

use iced::widget::{column, text, row, text_input, button};
use iced::{Alignment, Element, Length, Sandbox, Settings};

use errors::UrlError;

fn open_url(path: String) -> Result<(), UrlError> {
    let path = if !path.starts_with("https://") && !path.starts_with("http://") {
        format!("http://{}", path)
    } else {
        path
    };

    match open::that(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(UrlError::OpenUrlFailed),
    }
}

fn main() -> iced::Result {
    HelloIced::run(Settings::default())
}

#[derive(Default)]
struct HelloIced {
    text: String
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
    TextSubmitted,
}

impl Sandbox for HelloIced {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Open URLs".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextChanged(s) => self.text = s,
            Message::TextSubmitted => { let _ = open_url(self.text.clone()); }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            column![
                row![
                    text_input("URL", self.text.as_str())
                        .on_input(Message::TextChanged)
                        .on_submit(Message::TextSubmitted)
                        .width(600).padding(10),
                    button("Open Link").on_press(Message::TextSubmitted).padding(10).width(100)
                ].spacing(10),
                
            ]
            .align_items(Alignment::Center)
            .width(Length::Fill)
        ]
        .into() 
    }
}
