// ---- 1. Define a trait: a set of methods types can share ----
trait Summary {
    // REQUIRED method: every implementor must provide this.
    fn summarize(&self) -> String;

    // DEFAULT method: implementors get this for free (may override it).
    fn preview(&self) -> String {
        format!("Read more -> {}", self.summarize())
    }
}

// ---- 2. Two unrelated types ----
struct Article { title: String, body: String }
struct Tweet   { username: String, text: String }

// ---- 3. Implement the SAME trait for EACH type, differently ----
impl Summary for Article {
    fn summarize(&self) -> String {
        let n = self.body.len().min(20);
        format!("{} — {}...", self.title, &self.body[..n])
    }
    // (no preview -> uses the default)
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.text)
    }
    fn preview(&self) -> String {            // override the default
        format!("Tweet by @{}", self.username)
    }
}

// ---- 4. Accept ANY type that implements Summary (these two are equivalent) ----
fn notify(item: &impl Summary) {             // `impl Trait` syntax
    println!("[notify]  {}", item.summarize());
}
fn notify_generic<T: Summary>(item: &T) {    // trait-bound generic (same meaning)
    println!("[generic] {}", item.summarize());
}

fn main() {
    let article = Article {
        title: String::from("Rust 2.0"),
        body: String::from("A major release with lots of new things."),
    };
    let tweet = Tweet {
        username: String::from("golam"),
        text: String::from("loving Rust!"),
    };

    // both types now have summarize(), because both impl Summary:
    println!("{}", article.summarize());
    println!("{}", tweet.summarize());

    // default vs overridden:
    println!("{}", article.preview());   // default preview
    println!("{}", tweet.preview());     // overridden preview

    // trait-bounded functions accept either type:
    notify(&article);
    notify_generic(&tweet);
}