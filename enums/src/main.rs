
// ---- 1. A simple enum: a fixed set of states (like a C enum) ----
#[derive(Debug)]
enum Direction { North, South, East, West }

impl Direction {                     // enums can have methods too
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
            Direction::West  => Direction::East,
        }
    }
}

// ---- 2. The powerful part: variants that CARRY DIFFERENT DATA ----
#[derive(Debug)]
enum Shape {
    Circle(f64),                        // tuple-like: one value (the radius)
    Rectangle { width: f64, height: f64 },  // struct-like: named fields
    Triangle { base: f64, height: f64 },
}

impl Shape {
    // `match` extracts each variant's data and computes accordingly.
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

// ---- 3. Option<T>: the built-in enum that REPLACES null ----
//   enum Option<T> { Some(T), None }   (already in the language)
// Returns Some(value) if found, None if not. No null, no exceptions.
fn find_first_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    // simple enum + method
    let d = Direction::North;
    println!("{:?} -> opposite {:?}", d, d.opposite());

    // data-carrying enum + match
    let shapes = [
        Shape::Circle(2.0),
        Shape::Rectangle { width: 3.0, height: 4.0 },
        Shape::Triangle { base: 6.0, height: 2.0 },
    ];
    for s in &shapes {
        println!("{:?}  area = {:.2}", s, s.area());
    }

    // Option with match — you MUST handle both Some and None
    let numbers = [1, 2, 3, 4, 5];
    match find_first_even(&numbers) {
        Some(n) => println!("first even: {n}"),
        None => println!("no even number"),
    }

    // `if let` — concise when you only care about ONE variant
    if let Some(n) = find_first_even(&numbers) {
        println!("(via if let) got {n}");
    }

    // the None case
    match find_first_even(&[1, 3, 5]) {
        Some(n) => println!("first even: {n}"),
        None => println!("no even number in [1,3,5]"),
    }
}