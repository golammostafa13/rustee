use std::collections::HashMap;            // 1) import the ITEM (idiomatic for types)
use std::cmp;                             // 2) import the PARENT module (idiomatic for fns)
use std::collections::BTreeMap as Tree;   // 3) `as` renames the import
use std::io::{self, Write};              // 4) group: the `io` module ITSELF + the `Write` trait

// 6) pub use — RE-EXPORT an item to a shorter/public path
mod shapes {
    pub mod circle {
        pub fn area(r: f64) -> f64 { std::f64::consts::PI * r * r }
    }
    pub use self::circle::area;                 // lift circle::area up to shapes::area
}
use shapes::area;

use rand;

fn main() {
    // 1) type imported → use the name directly
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("golam", 90);
    println!("scores : {scores:?}");

    // 2) function reached through its parent module → `cmp::max` shows where it's from
    println!("max    : {}", cmp::max(3, 9));

    // 3) the renamed alias
    let mut t: Tree<i32, &str> = Tree::new();
    t.insert(1, "one");
    println!("tree   : {t:?}");

    // 6) the re-exported function, reachable by the short path
    println!("area   : {:.2}", area(2.0));

    // 4) both pieces of {self, Write} in action:
    //    io::stdout() uses the `io` module; write_all/flush come from the `Write` trait
    io::stdout().write_all(b"written via the Write trait\n").unwrap();
    io::stdout().flush().unwrap();

    let number = rand::random_range(1..=100);
    println!("The secret number is: {}", number);
}