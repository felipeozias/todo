fn main() {
    println!("🖋  Você deseja adicionar um novo TODO? (Responta do s ou n)");
    let mut input_user: String = input();

    while input_user != "n" && input_user != "N" {
        println!("Digite o seu TODO abaixo: ⤵ ");
        let todo_add: String = input();
        println!("--------------------------------------------");
        println!("✅ 🟢 O TODO adicionado foi: '{}' 🟢", todo_add);
        println!("--------------------------------------------");

        println!("🖋  Você deseja adicionar um novo TODO? (Responta do s ou n)");
        input_user = input();

        if input_user == "n" || input_user == "N" {
            return;
        }
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
