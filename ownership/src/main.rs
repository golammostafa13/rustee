

fn main() {
    scope_and_drop();
    moving();
    copy_types();
    cloning();
    passing_moves();
    returning_gives_back();
    borrowing();
    mutable_borrowing();
}

// 1) SCOPE & DROP -------------------------------------------------
fn scope_and_drop() {
    println!("\n== 1. scope & drop ==");
    {
        let s = String::from("inside");   // s becomes valid here
        println!("s is alive: {s}");
    }                                     // s goes out of scope -> dropped (freed) here
    // println!("{s}");                   // would FAIL: s no longer exists
}

// 2) MOVE ---------------------------------------------------------
fn moving() {
    println!("\n== 2. move ==");
    let s1 = String::from("hello");
    let s2 = s1;                          // ownership MOVES from s1 into s2
    println!("s2 owns it now: {s2}");
    // println!("{s1}");                  // would FAIL (E0382): s1 was moved
}

// 3) COPY TYPES ---------------------------------------------------
fn copy_types() {
    println!("\n== 3. copy types ==");
    let x = 5;
    let y = x;                            // integers are Copy: x is duplicated, not moved
    println!("both valid: x={x}, y={y}");
}

// 4) CLONE --------------------------------------------------------
fn cloning() {
    println!("\n== 4. clone ==");
    let s1 = String::from("hello");
    let s2 = s1.clone();                  // explicit deep copy on the heap
    println!("both valid: s1={s1}, s2={s2}");
}

// 5) PASSING A VALUE MOVES IT -------------------------------------
fn passing_moves() {
    println!("\n== 5. passing a value moves it ==");
    let s = String::from("hello");
    take(s);                              // s is moved into take()
    // println!("{s}");                   // would FAIL: this scope no longer owns s
}
fn take(text: String) {
    println!("take() received: {text}");
}                                         // text dropped here

// 6) RETURNING GIVES OWNERSHIP BACK -------------------------------
fn returning_gives_back() {
    println!("\n== 6. returning gives ownership back ==");
    let s1 = give();                      // give() hands ownership out to s1
    let s2 = take_and_return(s1);         // s1 moved in, ownership returned as s2
    println!("got ownership back: {s2}");
    // println!("{s1}");                  // would FAIL: s1 was moved into the function
}
fn give() -> String {
    String::from("made inside give()")
}
fn take_and_return(text: String) -> String {
    text                                  // returning moves ownership back out
}

// 7) BORROWING — use without taking ownership ---------------------
fn borrowing() {
    println!("\n== 7. borrowing with & ==");
    let s = String::from("hello");
    let len = length(&s);                 // pass a REFERENCE; ownership stays here
    println!("still own '{s}', its length = {len}");
}
fn length(text: &String) -> usize {       // borrows, does not own
    text.len()
}                                         // only the reference ends; String not dropped

// 8) MUTABLE BORROWING — borrow AND modify ------------------------
fn mutable_borrowing() {
    println!("\n== 8. mutable borrowing with &mut ==");
    let mut s = String::from("hello");
    append(&mut s);                       // lend a mutable reference
    println!("after append: {s}");
}
fn append(text: &mut String) {
    text.push_str(", world");
}