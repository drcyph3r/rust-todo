use std::env;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let command = &arg[1];
    println!("{:#?}", command);
}
