// use::std::fs::File;
// use::std::io::ErrorKind;

// fn main() {
//     let greeting_from_file = File::open("hello.txt");
//     let greeting_file = match greeting_from_file {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt"){
//                 Ok(f) => f,
//                 Err(e) => panic!("Prolem creating the file {e:?}")
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error)
//             }
//         }
//     };
// }

use std::num::ParseIntError;

fn main() {
    // ---- Result<T, E> is Ok(value) OR Err(error). Sibling of Option. ----

    // 1) full control with match
    match "42".parse::<i32>() {
        Ok(n)  => println!("match         : parsed {n}"),
        Err(e) => println!("match         : failed ({e})"),
    }

    // 2) shortcuts when a full match is overkill:
    println!("unwrap        : {}", "42".parse::<i32>().unwrap());         // PANICS on Err
    println!("expect        : {}", "42".parse::<i32>().expect("a number"));// panic + message
    println!("unwrap_or     : {}", "oops".parse::<i32>().unwrap_or(0));    // default on Err
    println!("unwrap_or_else: {}", "oops".parse::<i32>().unwrap_or_else(|_| -1));

    // 3) the ? operator — propagate errors up (see double_number below)
    println!("double(\"21\")  : {:?}", double_number("21"));
    println!("double(\"nope\"): {:?}", double_number("nope"));
}

// `?` = "if this is Err, return that Err from THIS function immediately;
//        if it's Ok, unwrap the value and keep going."
// It requires the enclosing function to return a Result.
fn double_number(text: &str) -> Result<i32, ParseIntError> {
    let n = text.parse::<i32>()?;   // the ? replaces a whole match-and-early-return
    Ok(n * 2)
}