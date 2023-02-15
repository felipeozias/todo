use crate::modules::terminal::{Terminal, TerminalError};

pub fn start_todo(mut terminal: Terminal) -> Result<(), TerminalError> {
    loop {
        let todo = terminal.ask_for_new_todo()?;

        match todo {
            Some(value) => terminal.show_todo(&value)?,
            None => return Ok(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Todo {
    pub message: String,
}

impl Todo {
    pub fn new(message: String) -> Self {
        Todo { message }
    }
}
