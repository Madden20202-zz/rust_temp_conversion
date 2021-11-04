use std::io;

fn main() {
    ask_scale();
}

fn ask_scale() {

    // adds a variable that lets 
    // the formula be picked
    let mut scale = String::new();

    // Prompts user
    println!("Will we be using Celcius or Fahrenheit?");
    println!("Input C or F");

    // changes the scale value and gives an error message if not string
    io::stdin()
        .read_line(&mut scale)
        .expect("Please input C or F");
    println!("You put {}", scale);

}