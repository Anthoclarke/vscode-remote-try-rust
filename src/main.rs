use std::io::{self,Write};

struct Task {
    name: String,
    id: u32,
    completed: bool,
}
fn main() {
    println!("yo");
    let input = read_input("Ton nom: ");
    println!("{}", &input);
}

fn read_input(text: &str) -> String {
    print!("{text}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}