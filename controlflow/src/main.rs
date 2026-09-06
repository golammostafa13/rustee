fn main() {
    // if expression
    let number = 7;
    if number < 5 {
        println!("less than 5");
    } else if number == 5 {
        println!("equal to 5");
    } else {
        println!("greater than 5");
    }

    let condition = true;
    let x = if condition { 10 } else { 20 };
    println!("x = {x}");

    // match
    let day = 3;
    let name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4..=5 => "Thursday-Friday",
        _ => "Weekend",
    };
    println!("day = {name}");

    // loop expression
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {result}");

    // while expression
    let mut n = 3;
    while n > 0 {
        println!("{n}");
        n -= 1;
    }
    println!("liftoff");

    // for expression
    for i in 1..=5 {
        println!("i = {i}");
    }

    let arr = [10, 20, 30];
    for element in arr {
        println!("element = {element}");
    }
}