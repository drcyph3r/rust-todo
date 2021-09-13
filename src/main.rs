use std::env;

struct TodoItem {
    name: String,
    completed: char
}

fn main() {
    let arg: Vec<String> = env::args().collect();
    let command = &arg[1];

    // example todo
    let todo_item = TodoItem {
        name: "Go thru the solana docs".to_string(),
        completed: ' '
    };

    let todo_list = vec![todo_item];
    
    match command.as_str() {
        "get" => {
            for item in todo_list {
                println!("[{}] - {}", item.completed, item.name);
            }
        },
        _ => return (),
    }
}
 