use std::io::{ Stdin, Stdout, Write };

fn main() {
    loop {
        let mut terminal1 = Terminal::new();
        let todo = terminal1.ask_for_new_todo();
        terminal1.show_todo(&todo);
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

    fn ask_for_new_todo(&mut self) -> Todo {
        if !self.should_ask_for_new_todo() {
            std::process::exit(0);
        } else {
            writeln!(self.stdout,"Digite o seu TODO abaixo: ⤵ ").unwrap();
            Todo::new(self.input_terminal())
        }
    }

    fn should_ask_for_new_todo(&mut self) -> bool {
        
        loop {
            writeln!(self.stdout,"🖋  Você deseja adicionar um novo TODO? (Responda s para SIM ou n para NÃO)").unwrap();

            let response = self.input_terminal().to_lowercase();

            if response == "n" {
                break false;
            } else if response == "s" {
                break true;
            } else {
                writeln!(self.stdout,"Não entendi sua resposta 😕").unwrap();
            }
        }
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout,"---------------------------------------------------").unwrap();
        writeln!(self.stdout, "✅ 🟢 O TODO adicionado foi: '{}' 🟢", todo.message).unwrap();
        writeln!(self.stdout,"---------------------------------------------------").unwrap();
    }

    fn input_terminal(&mut self) -> String{
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }
}
