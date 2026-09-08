fn main() {
    // ---- 1. Literals, ranges, OR (|), and the _ catch-all — returning a value ----
    let number = 7;
    let description = match number {
        0 => "zero",
        1 => "one",
        2 | 3 => "two or three",       // |  means OR: match either pattern
        4..=9 => "single digit 4-9",   // ..= is an INCLUSIVE range
        _ => "ten or more",            // _  is the catch-all (makes it exhaustive)
    };
    println!("1) {number} is {description}");

    // ---- 2. Match GUARDS: an extra `if` condition on an arm ----
    let pair = (5, -5);
    let msg = match pair {
        (x, y) if x + y == 0 => "sums to zero",   // guard runs only if pattern matched
        (x, _) if x > 0      => "first is positive",
        _                    => "something else",
    };
    println!("2) {pair:?} -> {msg}");

    // ---- 3. @ binding: test a range AND capture the value in one go ----
    let age = 15;
    match age {
        n @ 0..=12  => println!("3) child (age {n})"),
        n @ 13..=19 => println!("3) teenager (age {n})"),
        n           => println!("3) adult (age {n})"),
    }

    // ---- 4. Destructure a TUPLE, matching specific positions ----
    let point = (0, 7);
    match point {
        (0, 0) => println!("4) origin"),
        (0, y) => println!("4) on y-axis at y={y}"),
        (x, 0) => println!("4) on x-axis at x={x}"),
        (x, y) => println!("4) at ({x}, {y})"),
    }

    // ---- 5. Destructure a STRUCT ----
    struct Point { x: i32, y: i32 }
    let p = Point { x: 3, y: 0 };
    match p {
        Point { x, y: 0 } => println!("5) on x-axis at {x}"),
        Point { x: 0, y } => println!("5) on y-axis at {y}"),
        Point { x, y }    => println!("5) at ({x}, {y})"),
    }

    // ---- 6. Match on Option, combining several features ----
    let maybe = Some(10);
    match maybe {
        Some(n) if n > 5 => println!("6) large: {n}"),
        Some(n)          => println!("6) small: {n}"),
        None             => println!("6) nothing"),
    }
}