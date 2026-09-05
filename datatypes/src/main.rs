fn main() {
    // ========== SCALAR TYPES (a single value) ==========

    // --- Integers ---
    // Signed (i) hold negatives; unsigned (u) hold only 0 and up.
    // Widths: i8/u8, i16/u16, i32/u32, i64/u64, i128/u128,
    //         plus isize/usize (width matches the CPU: 64-bit here).
    let small_signed: i8 = -128;          // range -128 ..= 127
    let small_unsigned: u8 = 255;         // range 0 ..= 255
    let default_int: i32 = -2_000_000;    // i32 is the DEFAULT; `_` is just a visual separator
    let big: u64 = 18_000_000_000;
    let huge: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // i128 max
    let index: usize = 42;                // usize: used for lengths & indexing
    println!("integers : {small_signed}, {small_unsigned}, {default_int}, {big}, {huge}, {index}");

    // --- Boolean --- only `true` or `false`
    let is_active: bool = true;
    let is_ready = false;                 // type inferred as bool
    println!("booleans : {is_active}, {is_ready}");

    // --- Float --- f64 (default, double precision) and f32
    let pi: f64 = 3.14159;
    let ratio: f32 = 1.5;
    println!("floats   : {pi}, {ratio}");

    // --- Character --- ONE Unicode scalar, single quotes, 4 bytes.
    // Note: 'G' (a char) is NOT the same as "G" (a string!).
    let letter: char = 'G';
    let crab: char = '🦀';                // a char can be an emoji
    let bangla: char = 'অ';               // ...or any Unicode letter
    println!("chars    : {letter}, {crab}, {bangla}");

    // ========== COMPOUND TYPES (group several values) ==========

    // --- Array --- FIXED length, all SAME type, lives on the stack. Type: [T; N]
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let zeros = [0; 3];                   // shorthand for [0, 0, 0]
    println!("array    : {:?}  first={} len={}", arr, arr[0], arr.len());
    println!("zeros    : {:?}", zeros);

    // --- Tuple --- FIXED length, can MIX types. Access by .0/.1, or destructure.
    let person: (&str, i32, bool) = ("Golam", 25, true);
    let (name, age, employed) = person;   // destructuring
    println!("tuple    : name={name} age={age} employed={employed}  (by index .0 = {})", person.0);

    // --- Vec --- GROWABLE, all same type, lives on the HEAP. (std library type)
    let mut nums: Vec<i32> = vec![1, 2, 3];
    nums.push(4);
    nums.push(5);
    println!("vec      : {:?}  len={}", nums, nums.len());

    // --- String --- GROWABLE, heap-allocated, UTF-8 text. (std library type)
    // `String` OWNS its data; `&str` is a borrowed VIEW (slice) into text.
    let mut greeting: String = String::from("Hello");
    greeting.push_str(", Rust!");
    let slice: &str = &greeting[0..5];    // a &str view of the first 5 bytes
    println!("String   : {greeting}   &str slice: {slice}");

    // ========== CUSTOM TYPES (types you define) ==========

    // --- Struct --- a named bundle of fields (like a typed record/object).
    let user = User {
        username: String::from("golam"),
        active: true,
        login_count: 7,
    };
    println!("struct   : {:?}  (one field: {})", user, user.username);

    // --- Tuple struct --- a struct with UNNAMED fields (good for wrappers).
    let black = Color(0, 0, 0);
    println!("tuple st.: {:?}  red channel = {}", black, black.0);

    // --- Enum --- a value that is ONE OF several variants; variants can carry data.
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hi")),
        Message::ChangeColor(255, 0, 0),
    ];
    for m in &messages {
        describe(m);                      // `match` handles each variant
    }
}

// ---------- custom type definitions ----------

#[derive(Debug)]                          // derive Debug so {:?} can print it
struct User {
    username: String,
    active: bool,
    login_count: u32,
}

#[derive(Debug)]
struct Color(u8, u8, u8);                 // tuple struct: fields are .0 .1 .2

#[derive(Debug)]
enum Message {
    Quit,                                 // variant with no data
    Move { x: i32, y: i32 },              // variant with named fields
    Write(String),                        // variant holding a String
    ChangeColor(u8, u8, u8),              // variant holding three u8s
}

// `match` is the idiomatic way to act on each enum variant:
fn describe(m: &Message) {
    match m {
        Message::Quit => println!("enum     : Quit"),
        Message::Move { x, y } => println!("enum     : Move to ({x}, {y})"),
        Message::Write(text) => println!("enum     : Write \"{text}\""),
        Message::ChangeColor(r, g, b) => println!("enum     : Color #{:02x}{:02x}{:02x}", r, g, b),
    }
}