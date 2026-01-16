#![windows_subsystem = "windows"]
use iced::{
    Element, Padding, Task,
    widget::{button, column, container, text, text_editor},
    window::{self, Icon, icon},
};

#[derive(Debug, Default)]
struct AppState {
    editor_content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Messages {
    Exit,
    Edit(text_editor::Action),
}

fn update(state: &mut AppState, message: Messages) -> Task<Messages> {
    match message {
        Messages::Exit => window::latest().and_then(window::close),
        Messages::Edit(action) => {
            state.editor_content.perform(action);
            Task::none()
        }
    }
}

fn view(state: &AppState) -> Element<'_, Messages> {
    // text("Hello").into()
    let words = &state
        .editor_content
        .text()
        .split_whitespace()
        .collect::<Vec<_>>()
        .len();
    container(
        column![
            text("Rust-made Simple Word Counter").size(32),
            button(text("close window"))
                .padding(Padding::from([5, 10]))
                .on_press(Messages::Exit),
            text_editor(&state.editor_content)
                .padding(Padding::from([5, 10]))
                .height(150)
                .on_action(Messages::Edit),
            text(format!("words: {}", words))
        ]
        .spacing(20),
    )
    .padding(20)
    .into()
}

fn main() -> iced::Result {
    let image_bytes = include_bytes!("../window_icon.png");
    let img_rgba8 = image::load_from_memory(image_bytes)
        .expect("msg")
        .into_rgba8();
    let (img_w, img_h) = img_rgba8.dimensions();
    let icon = icon::from_rgba(img_rgba8.into_raw(), img_w, img_h).expect("msg");

    iced::application(|| AppState::default(), update, view)
        .title("Rust-made Simple Word Counter")
        .window(window::Settings {
            size: [600.0, 400.0].into(),
            resizable: true,
            icon: Some(icon),
            ..Default::default()
        })
        .run()
}
