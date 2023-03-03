use crate::modules::todo::Todo;
use console::style;
use std::io::{Error, Stdin, Stdout, Write};

pub struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    pub fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        let response = self.should_ask_for_new_todo()?;

        if !response {
            Ok(None)
        } else {
            self.print_str(format!(
                "{}",
                style("Digite o seu TODO abaixo: ⤵ ").on_blue()
            ))?;

            let value = self.input_terminal()?;
            Ok(Some(Todo::new(value)))
        }
    }

    fn should_ask_for_new_todo(&mut self) -> Result<bool, TerminalError> {
        loop {
            self.print_str(format!(
                "🖋  Você deseja adicionar um novo TODO? (Responda {} para {} ou {} para {})",
                style("S").on_green(),
                style("SIM").on_green(),
                style("N").on_red(),
                style("NÃO").on_red(),
            ))?;

            let response = self.input_terminal()?;

            match response.to_lowercase().as_str() {
                "n" => return Ok(false),
                "s" => return Ok(true),
                _ => {
                    self.print_str(format!("{}", style("Não entendi sua resposta 😕").on_red()))?
                }
            }
        }
    }

    pub fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        self.print_str(format!(
            "
---------------------------------------------------
✅ 🟢 O TODO adicionado foi: '{}' 🟢
---------------------------------------------------
            ",
            style(&todo.message).bold().underlined().green()
        ))?;

        Ok(())
    }

    fn input_terminal(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        self.stdin
            .read_line(&mut buf)
            .map_err(TerminalError::Stdin)?;
        Ok(buf.trim().to_string())
    }

    fn print_str(&mut self, s: String) -> Result<(), TerminalError> {
        writeln!(self.stdout, "{}", s).map_err(TerminalError::Stdout)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum TerminalError {
    Stdout(Error),
    Stdin(Error),
}
