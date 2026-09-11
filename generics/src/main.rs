// ---- 1. Generic FUNCTION. <T: PartialOrd> = "T must be orderable",
//         which is required to use `>` inside. That's a TRAIT BOUND. ----
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in list {
        if item > biggest {        // `>` is only allowed because of the PartialOrd bound
            biggest = item;
        }
    }
    biggest
}

// ---- 2. Generic STRUCT with one type parameter ----
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {                 // methods for ANY T
    fn new(x: T, y: T) -> Point<T> { Point { x, y } }
    fn x(&self) -> &T { &self.x }
}

impl Point<f64> {                  // a method that exists ONLY when T is f64
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// ---- 3. Generic struct with TWO type parameters (types can differ) ----
#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    // one function, different element types:
    let nums = vec![3, 7, 2, 9, 4];
    let chars = vec!['a', 'z', 'm'];
    println!("largest number : {}", largest(&nums));
    println!("largest char   : {}", largest(&chars));

    // one struct, different concrete types:
    let int_point = Point::new(3, 4);
    let float_point = Point::new(1.5, 2.5);
    println!("int point   : {:?}  x={}", int_point, int_point.x());
    println!("float point : {:?}", float_point);
    println!("distance    : {:.2}", float_point.distance_from_origin());  // only on Point<f64>

    // two type parameters — first and second can be different types:
    let mixed = Pair { first: 42, second: "hello" };
    println!("pair        : {:?}", mixed);

    // Option<T> and Result<T, E> are GENERIC ENUMS you've used all along:
    let maybe: Option<i32> = Some(5);
    let res: Result<i32, String> = Ok(10);
    println!("Option<i32> : {maybe:?}");
    println!("Result      : {res:?}");
}