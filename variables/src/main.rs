fn main() {
    let x = 3;
    println!("The value of x is {}", x);
    
    // shadowing
    let x = x + 3;
    println!("The value of x is {}", x);

    let mut y = 1;
    println!("The value of y is {}", y);

    y = y + 2;
    println!("The value of y is {}", y);

    const PORT: i32 = 8080;
    println!("The value of port is {}", PORT);

    // PORT can't be changed or mutated or re-assigned
}
