use std::io;

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

    loop {
        // changes the scale value and gives an error message if not string
        io::stdin()
            .read_line(&mut scale) 
            .expect("Error reading line");

        let scale: String = match scale.trim().parse(){
            Ok(str) => return check_scale(scale),
            Err(_) => return number_error(),

        };
        println!("You put {}", scale);
        break;
    }
}

fn ask_degrees() {

    let mut degree = String::new();

    println!("What is the Tempurature?");

    // loop added to ask after a word is put in 
    // currently it only breaks out of the loop 
    // must research how to put an error message in
    loop {
        io::stdin()
            .read_line(&mut degree)
            .expect("Please input a number");
    
        let degree: i16 = match degree.trim().parse() {
            Ok(num) => num,
            // ends the operation if input isnt a number
            Err(_) => return word_error(), 
        };

        println!("Did you say {}", degree);
        break;
    }

}

// These will print different messages based on what is wrong
fn check_scale(scale: String) {
    if scale == "C" {
        println!("You picked: Celcius");
    } else if scale == "F" {
        println!("You picked: Fahrenheit");
    } else {
        number_error();
    }
}

fn number_error() {
    println!("Please use C or F");
}

fn word_error() {
    println!("Numbers only at this stage");
}