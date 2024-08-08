use std::{error, fs};
use tokio::task::JoinHandle;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,

    pub handles: Vec<JoinHandle<()>>,

    pub loading: bool,
    pub reading: bool,

    pub current_book: usize,
    pub text_library: Vec<String>,

    pub current_working_directory: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_working_directory: String::from(""),
            reading: false,
            loading: false,
            running: true,
            current_book: 0,
            counter: 0,
            text_library: Vec::new(),
            handles: Vec::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn load_file(&mut self) {
        let contents =
            fs::read_to_string("item.txt").expect("Should have been able to read the file");
        self.text_library.push(contents);
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
