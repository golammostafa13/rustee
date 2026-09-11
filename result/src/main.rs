use std::num::ParseIntError;

fn main() {
    // ---- 1. Result + match: handle both cases, program keeps running ----
    println!("== 1. Result + match ==");
    match divide(10, 2) {
        Ok(q)  => println!("10 / 2 = {q}"),
        Err(e) => println!("error: {e}"),
    }
    match divide(10, 0) {
        Ok(q)  => println!("10 / 0 = {q}"),
        Err(e) => println!("error: {e}"),        // handled gracefully — no crash
    }

    // ---- 2. unwrap / expect: bridge a Result INTO a panic ----
    println!("\n== 2. unwrap / expect ==");
    let q = divide(20, 4).unwrap();               // Ok -> gives the value
    println!("unwrap ok : {q}");
    let q = divide(20, 4).expect("divisor is 4"); // like unwrap, with a message on Err
    println!("expect ok : {q}");
    // let boom = divide(1, 0).unwrap();          // would PANIC: unwrap on an Err

    // ---- 3. unwrap_or / unwrap_or_else: default instead of panicking ----
    println!("\n== 3. defaults ==");
    println!("unwrap_or      : {}", divide(1, 0).unwrap_or(-1));
    println!("unwrap_or_else : {}", divide(1, 0).unwrap_or_else(|_| -2));

    // ---- 4. ? operator: propagate the Err up to the caller ----
    println!("\n== 4. ? propagation ==");
    println!("double(\"21\")   : {:?}", double_str("21"));
    println!("double(\"nope\") : {:?}", double_str("nope"));

    // ---- 5. panic! family: for UNRECOVERABLE bugs ----
    println!("\n== 5. panic family ==");
    assert!(divide(9, 3).unwrap() == 3, "math is broken");   // passes -> no panic
    println!("assert passed");
    // panic!("stop now");                        // would PANIC: explicit stop
    // let _ = vec![1, 2, 3][99];                 // would PANIC: index out of bounds
    // todo!();                                   // would PANIC: unfinished-code placeholder
}

// RECOVERABLE error: return Result so the CALLER decides what to do.
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// ? propagates the Err: if parse fails, this function returns that Err immediately.
fn double_str(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n * 2)
}