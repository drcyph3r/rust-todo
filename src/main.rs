use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn create(name: String) -> TodoItem {
       return TodoItem {
            name: name,
            completed: ' ',
        }
    }
}

fn main() {
    let arg: Vec<String> = env::args().collect();
    let command = &arg[1];

    // example todo
    let todo_item_1 = TodoItem::create("Go thru the solana docs.".to_string());

    let todo_list = vec![todo_item_1];
    
    match command.as_str() {
        "get" => {
            for item in todo_list {
                println!("[{}] - {}", item.completed, item.name);
            }
        },
        _ => return (),
    }
}
 