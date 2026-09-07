fn main() {
    // ---- String slices (&str): a view into part of a String ----
    let s = String::from("hello world");
    let hello: &str = &s[0..5];      // bytes 0 up to (not including) 5
    let world: &str = &s[6..11];     // bytes 6 up to 11
    let whole: &str = &s[..];        // the whole thing
    let from2: &str = &s[2..];       // index 2 to the end
    let upto5: &str = &s[..5];       // start up to index 5
    println!("slices : '{hello}' '{world}' '{whole}' '{from2}' '{upto5}'");

    // ---- Array / Vec slices (&[T]) ----
    let arr = [10, 20, 30, 40, 50];
    let middle: &[i32] = &arr[1..4]; // borrows [20, 30, 40]
    println!("array slice : {:?}  len={}", middle, middle.len());

    let v = vec![1, 2, 3, 4, 5];
    let tail: &[i32] = &v[2..];      // borrows [3, 4, 5]
    println!("vec slice   : {:?}", tail);

    // ---- Canonical example: first word of a sentence ----
    let sentence = String::from("the quick brown fox");
    let word = first_word(&sentence);
    println!("first word  : '{word}'");

    // ---- Slices make FLEXIBLE parameters ----
    // &str accepts a String (auto-sliced) AND a string literal:
    println!("str_len String : {}", str_len(&sentence));
    println!("str_len literal: {}", str_len("literal"));
    // &[i32] accepts an array AND a Vec:
    println!("sum of array: {}", sum(&arr));
    println!("sum of vec  : {}", sum(&v));
}

// Returns a slice up to the first space. Borrows its input, owns nothing.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Takes &str, so it works for both String and literals (idiomatic).
fn str_len(s: &str) -> usize {
    s.len()
}

// Takes &[i32], so it works for both arrays and Vecs.
fn sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for &n in nums { // &n: here dereferencing the nums elements 
        total += n;
    }
    total
}