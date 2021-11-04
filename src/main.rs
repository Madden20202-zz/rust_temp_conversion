use std::io;
fn main() {
    let scale = String::new();
    ask_scale(scale);
}

fn ask_scale(scale: String) {
    let mut scale = String::new();
    println!("Will we be using Celcius or Fahrenheit?");
    println!("Input C or F");
    io::stdin()
        .read_line(&mut scale)
        .expect("Please input C or F");
}