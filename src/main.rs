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

        if self.should_ask_for_new_todo() == false {
            std::process::exit(0);
        } else {
            let mut todo_add = String::new();
            println!("Digite o seu TODO abaixo: ⤵ ");
            self.stdin.read_line(&mut todo_add).unwrap();
            Todo::new(todo_add.trim().to_string())
        }
    }

    fn should_ask_for_new_todo(&mut self) -> bool {
        loop {

            println!("🖋  Você deseja adicionar um novo TODO? (Responta do s para SIM ou n para NÃO)");

            let mut buf = String::new();
            self.stdin.read_line(&mut buf).unwrap();
            let response = buf.trim().to_lowercase();

            if response == "n" {
                break false;
            } else if response == "s" {
                break true;
            } else {
                println!("Não entendi sua resposta 😕");
            }
        }
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout,"----------------------------------------------------");
        writeln!(self.stdout, "✅ 🟢 O TODO adicionado foi: '{}' 🟢", todo.message).unwrap();
        writeln!(self.stdout,"----------------------------------------------------");
    }
}