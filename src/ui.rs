use crate::app::App;
use ratatui::widgets::block::{Position, Title};
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    if !app.loading && !app.reading {
        frame.render_widget(
            Paragraph::new(format!(
                "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
                app.counter
            ))
            .block(
                Block::bordered()
                    .title("[ Title ]")
                    .title(
                        Title::from(format!("[ {} ]", app.current_working_directory))
                            .position(Position::Bottom)
                            .alignment(Alignment::Right),
                    )
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
            frame.size(),
        );
    } else {
        let mut display_text: String = String::from("Loading");
        if !app.loading && app.reading {
            display_text = app.text_library[app.current_book].clone();
        }
        frame.render_widget(
            Paragraph::new(format!(
                "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Attempting to load file:\n\
                Contents: {}",
                display_text
            ))
            .block(
                Block::bordered()
                    .title(
                        Title::from("[ Title ]".to_string())
                            .position(Position::Bottom)
                            .alignment(Alignment::Right),
                    )
                    .title(
                        Title::from(format!("[ {} ]", app.current_working_directory))
                            .position(Position::Top)
                            .alignment(Alignment::Center),
                    )
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
            frame.size(),
        )
    }

}
