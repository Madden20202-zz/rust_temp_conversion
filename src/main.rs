use std::io;
use std::cmp::Ordering;

fn main() {
    ask_scale();
    ask_degrees();
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

fn ask_degrees() {

    let mut degree = String::new();

    println!("What is the Tempurature?");
    io::stdin()
        .read_line(&mut degree)
        .expect("Please input a number");

    let degree: i16 = match degree.trim().parse() {
        Ok(num) => num,
        Err(_) => println!("Please input a number"),
    };
    
    println!("You put {}", degree);

}