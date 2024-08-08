use megan::app::{App, AppResult};
use megan::event::{Event, EventHandler};
use megan::handler::{handle_file_loads, handle_key_events};
use megan::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::{env, io};

#[tokio::main]
async fn main() -> AppResult<()> {
    let args: Vec<String> = env::args().collect();
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    app.current_working_directory = std::env::current_dir().unwrap().display().to_string();
    //dbg!(format!("{}", app.current_working_directory.to_string()));
    app.text_library.push("Default".to_string());

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;

        if app.loading {
            handle_file_loads("".to_string(), &mut app).await?;
        }

        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
