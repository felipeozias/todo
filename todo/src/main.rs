fn main() {
    loop {
        let todo_add: String;

        println!("🖋  Você deseja adicionar um novo TODO? (Responta do s ou n)");
        let input_user = input();

        if input_user.to_lowercase() == "n" {
            break;
        }

        println!("Digite o seu TODO abaixo: ⤵ ");
        todo_add = input();
        println!("--------------------------------------------");
        println!("✅ 🟢 O TODO adicionado foi: '{}' 🟢", todo_add);
        println!("--------------------------------------------");
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
