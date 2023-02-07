use std::io::{Error, Stdin, Stdout, Write};

fn main() {
    let terminal = Terminal::new();
    start_todo(terminal);
}

fn start_todo(mut terminal: Terminal) {
    loop {
        let todo = terminal.ask_for_new_todo();

        match &todo {
            Ok(t) => match t {
                Some(value) => {
                    let _result = terminal.show_todo(value);
                }
                None => {
                    break;
                }
            },
            Err(_e) => {
                break println!(
                    "Ocorreu um erro, contacte o administrador. Erro: {:?}",
                    &todo
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    fn new(message: String) -> Self {
        Todo { message }
    }
}

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        let response = self.should_ask_for_new_todo()?;

        if !response {
            Ok(None)
        } else {
            self.print_str("Digite o seu TODO abaixo: ⤵ ")?;

            let value = self.input_terminal()?;
            Ok(Some(Todo::new(value)))
        }
    }

    fn should_ask_for_new_todo(&mut self) -> Result<bool, TerminalError> {
        loop {
            self.print_str(
                "🖋  Você deseja adicionar um novo TODO? (Responda s para SIM ou n para NÃO)",
            )?;

            let response = self.input_terminal()?;

            match response.to_lowercase().as_str() {
                "n" => return Ok(false),
                "s" => return Ok(true),
                _ => self.print_str("Não entendi sua resposta 😕")?,
            }
        }
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        self.print_str("---------------------------------------------------")?;
        self.print_str(format!("✅ 🟢 O TODO adicionado foi: '{}' 🟢", todo.message).as_str())?;
        self.print_str("---------------------------------------------------")?;

        Ok(())
    }

    fn input_terminal(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        self.stdin
            .read_line(&mut buf)
            .map_err(TerminalError::Stdin)?;
        Ok(buf.trim().to_string())
    }

    fn print_str(&mut self, s: &str) -> Result<(), TerminalError> {
        writeln!(self.stdout, "{}", s).map_err(TerminalError::Stdout)?;
        Ok(())
    }
}

#[derive(Debug)]
enum TerminalError {
    Stdout(Error),
    Stdin(Error),
}
