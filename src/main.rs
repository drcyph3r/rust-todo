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
        TodoItem {
            name: name,
            completed: ' ',
        }
    }
}

impl TodoList {
    // create a new list
    fn new() -> TodoList {
        TodoList{list: Vec::new()}
    }

    // add todo item to list
    fn add(&mut self, name: String) {
        let item = TodoItem::create(name);
        self.list.push(item);
    }

    fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    fn done(&mut self, index: usize) {
        self.list[index].completed = 'x';
    }

    // display todo list
    fn display(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{} [{}] - {}", index, item.completed, item.name);
        }
    }
}

enum Command {
    Get,
    Add(String),
    Done(usize),
    Remove(usize),
}

fn main() {
    // collect the arguments from command line
    let arg: Vec<String> = env::args().collect();
    // extract the command from the arguments
    let command = match arg[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(arg[2].clone()),
        "done" => Command::Done(arg[2].parse::<usize>().expect("error parsing the item index")),
        "remove" => Command::Remove(arg[2].parse::<usize>().expect("error parsing the item index")),
        _ => panic!("Invalid todo command.")
    };

    // initialize the empty todo list
    let mut todo_list = TodoList::new();
    
    // do todo operations according to command received
    match command {
        Command::Get => todo_list.display(),
        Command::Add(item) => {
            todo_list.add(item);
            todo_list.display();
        },
        Command::Done(index) => {
            todo_list.done(index);
            todo_list.display();
        },
        Command::Remove(index) => {
            todo_list.remove(index);
            todo_list.display();
        }
    }
}