// A simple CLI in rust
// This CLI will get string in input and will return reverse string

use std::env;

fn main() {
    // collect the arguements in a vec
    let arguements: Vec<String> = env::args().collect();

    // check if the user has entered a string
    // by checking the number of arg

    println!("arguements: {:?}", arguements);

    if arguements.len() < 2{
        println!("Please enter a string");
        return;
    }

    let input: String = arguements[1].clone();

    let reversed: String = input.chars().rev().collect();

    println!("Reversed string: {}", reversed);
}
