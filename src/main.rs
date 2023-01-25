use std::io::{ Stdin, Stdout, Write, Error };

fn main() {
    let terminal = Terminal::new();
    start_todo(terminal);
}

fn start_todo(mut terminal:Terminal) {
    loop {
        let todo = terminal.ask_for_new_todo();

        match &todo {
            Ok(t) => {
                match t {
                    Some(value) => {
                        let _result = terminal.show_todo(value);
                    }
                    None => {
                        break;
                    }
                }
            }
            Err(_e) => {
                break println!("Ocorreu um erro, contacte o administrador. Erro: {:?}", &todo);
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
        let response = self.should_ask_for_new_todo();

        match response {
            Ok(value) => {
                if !value {
                    Ok(None)
                } else {
                    writeln!(self.stdout, "Digite o seu TODO abaixo: ⤵ ").map_err(
                        TerminalError::Stdout
                    )?;

                    let response = self.input_terminal();

                    match response {
                        Ok(t) => { Ok(Some(Todo::new(t))) }
                        Err(error) => Err(error),
                    }
                }
            }
            Err(error) => Err(error),
        }
    }

    fn should_ask_for_new_todo(&mut self) -> Result<bool, TerminalError> {
        loop {
            writeln!(
                self.stdout,
                "🖋  Você deseja adicionar um novo TODO? (Responda s para SIM ou n para NÃO)"
            ).map_err(TerminalError::Stdout)?;

            let response = self.input_terminal();

            match response {
                Ok(t) => {
                    if t.to_lowercase() == "n" {
                        break Ok(false);
                    } else if t.to_lowercase() == "s" {
                        break Ok(true);
                    } else {
                        writeln!(self.stdout, "Não entendi sua resposta 😕").map_err(
                            TerminalError::Stdout
                        )?;
                    }
                }
                Err(error) => {
                    break Err(error);
                }
            }
        }
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(self.stdout, "---------------------------------------------------").map_err(
            TerminalError::Stdout
        )?;
        writeln!(self.stdout, "✅ 🟢 O TODO adicionado foi: '{}' 🟢", todo.message).map_err(
            TerminalError::Stdout
        )?;
        writeln!(self.stdout, "---------------------------------------------------").map_err(
            TerminalError::Stdout
        )?;
        Ok(())
    }

    fn input_terminal(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).map_err(TerminalError::Stdin)?;
        Ok(buf.trim().to_string())
    }
}

#[derive(Debug)]
enum TerminalError {
    Stdout(Error),
    Stdin(Error),
}