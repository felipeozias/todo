mod modules;

fn main() {
    let terminal = modules::terminal::Terminal::new();
    let result = modules::todo::start_todo(terminal);

    if let Err(error) = result {
        println!(
            "Ocorreu um erro, contacte o administrador. Erro: {:?}",
            &error
        )
    }
}
