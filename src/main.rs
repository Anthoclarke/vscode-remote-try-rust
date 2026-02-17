use std::io::{self,Write};




struct Task {
    name: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        println!("\n1: Add Task");
        println!("2: View tasks\n");
        match read_input("Choose an option: ").as_str() {
            "1" => {
                let task = Task {
                    name: read_input("Task name: "),
                    completed: false
                };
                println!("Task \"{}\" has been added to the list", &task.name);
                tasks.push(task);
                },
            "2" => {
                show(tasks);
            }
            "3" => {
                read_input("Choose a task # to delete (0 to exit): ");
                // à continuer!
            }
            _ => break,
        }
    }
}


fn show(mut tasks: Vec<Task>) {
    println!("");
    for (i, task) in tasks.iter().enumerate() {
        if i > 0 {
            println!("|\n| {} - {} ({})", i+1, task.name, if task.completed {"✅"} else {"❌"})
        }
        else {
            println!("| {} - {} ({})", i+1, task.name, if task.completed {"✅"} else {"❌"})
        }
    }
    println!("");
}


fn read_input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();


    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


