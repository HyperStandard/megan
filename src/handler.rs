use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        KeyCode::Char('l') | KeyCode::Char('L') => {
            app.loading = !app.loading;
        }

        // Counter handlers
        KeyCode::Right => {
            app.increment_counter();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

pub async fn handle_file_loads(_filename: String, app: &mut App) -> AppResult<()> {
    let book = tokio::fs::read_to_string("C:\\Users\\amelia\\Objects\\megan\\item.txt").await?;
    app.loading = false;
    app.reading = true;
    app.text_library[app.current_book] = book.to_string();
    Ok(())
}
