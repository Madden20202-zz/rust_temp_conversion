use std::io;

fn main() {
    ask_scale();
}

fn ask_scale() {
    let mut scale = String::new();
    println!("Will we be using Celcius or Fahrenheit?");
    println!("Input C or F");
    io::stdin()
        .read_line(&mut scale)
        .expect("Please input C or F");
    println!("You put {}", scale);
}