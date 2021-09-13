use std::env;

struct TodoItem {
    name: String,
    completed: char
}

struct TodoList {
    list: Vec<TodoItem>
}

impl TodoItem {
    fn create(name: String) -> TodoItem {
       return TodoItem {
            name: name,
            completed: ' ',
        }
    }
}

impl TodoList {
    // create a new list
    fn new() -> TodoList {
        return TodoList{list: Vec::new()};
    }

    // add todo item to list
    fn add(&mut self, name: String) {
        let item = TodoItem::create(name);
        self.list.push(item);
    }

    // display todo list
    fn display(&self) {
        for item in &self.list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}

fn main() {
    // collect the arguments from command line
    let arg: Vec<String> = env::args().collect();
    // extract the command from the arguments
    let command = &arg[1];

    // initialize the empty todo list
    let todo_list = TodoList::new();
    
    // do todo operations according to command received
    match command.as_str() {
        "get" => {
            todo_list.display()
        },
        "add" => {

        },
        _ => return (),
    }
}
 