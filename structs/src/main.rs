// ---- A named-field struct.
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    login_count: u64,
}

// A struct whose behavior we'll define with methods below.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ---- impl block: where a struct's methods & associated functions live ----
impl Rectangle {
    // ASSOCIATED FUNCTION: no `self`. Called as Rectangle::new(...).
    // This is the idiomatic "constructor" pattern (like String::from).
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }        // field init shorthand (see below)
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // METHOD: takes &self — borrows the instance to READ it. Called as rect.area().
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // METHOD borrowing self AND another Rectangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // METHOD taking &mut self — borrows mutably to MODIFY the instance.
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

// A builder using field init shorthand.
fn build_user(username: String, email: String) -> User {
    User {
        username,        // shorthand for `username: username` (name matches variable)
        email,           // shorthand for `email: email`
        active: true,
        login_count: 0,
    }
}

fn main() {
    // ---- create an instance; `mut` on the whole instance allows field edits ----
    let mut user1 = build_user(String::from("golam mostafa"), String::from("g@example.com"));
    println!("user1        : {:?}", user1);

    user1.login_count += 1;              // access & mutate a field with `.`
    user1.active = false;
    println!("after edits  : active={}, logins={}", user1.active, user1.login_count);

    // ---- struct update syntax: fill the rest from another instance ----
    let user2 = User {
        username: String::from("mostafa"),
        ..user1                          // take email/active/login_count from user1 and compound type fields will move here except username
    };
    println!("user2        : {:?}", user2);

    // ---- associated functions (::) vs methods (.) ----
    let rect = Rectangle::new(30, 50);   // :: because `new` has no self
    let sq = Rectangle::square(20);
    println!("rect area    : {}", rect.area());          // . because area takes &self
    println!("sq area      : {}", sq.area());
    println!("rect>sq?     : {}", rect.can_hold(&sq));

    // ---- &mut self method ----
    let mut r = Rectangle::new(3, 4);
    r.scale(2);
    println!("scaled       : {:?}  area={}", r, r.area());

    // ---- tuple struct (unnamed fields) & unit struct (no fields) ----
    #[derive(Debug)]
    struct Point(i32, i32);
    let p = Point(2, 7);
    println!("tuple struct : {:?}  .0={}", p, p.0);

    struct Marker;                       // unit struct — carries no data
    let _m = Marker;
    println!("unit struct  : created (zero fields)");
}