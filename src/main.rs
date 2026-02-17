use std::io::{self,Write};




struct Task {
    name: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        println!("\n1: Add Task");
        println!("2: View tasks");
        println!("3: Remove task\n");
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
                show(&tasks);
            }
            "3" => {
                show(&tasks);
                println!("");
                let index: usize = read_input("Choose a task # to delete (0 to exit): ").parse().unwrap();
                let max_index = tasks.len();
                if index > max_index {
                    println!("Nope");
                }
                else {
                    println!("Removed {} from task list", tasks[index-1].name);
                    tasks.remove(index-1);
                }
            }
            "4" => {
                show(&tasks);
                println!("");
                let index: usize = read_input("Choose a task # to delete (0 to exit): ").parse().unwrap();
                let max_index = tasks.len();
                if index > max_index {
                    println!("Nope");
                }
                else {
                    let task = &tasks[index-1];
                    if task.completed {
                        println!("Set {} to Incomplete", &task.name);
                        task.completed = !task.completed
                    }
                    else {
                        println!("Set {} to Complete", &task.name);
                    }
                }
            }
            _ => break,
        }
    }
}


fn show(tasks: &Vec<Task>) {
    println!("");
    for (i, task) in tasks.iter().enumerate() {
        if i > 0 {
            println!("|\n| {} - {} ({})", i+1, task.name, if task.completed {"✅"} else {"❌"})
        }
        else {
            println!("| {} - {} ({})", i+1, task.name, if task.completed {"✅"} else {"❌"})
        }
    }
}


fn read_input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();


    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}