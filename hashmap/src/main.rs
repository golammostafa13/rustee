use std::collections::HashMap;

fn main() {
    // ---- create + insert ----
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("golam"), 90);
    scores.insert(String::from("mostafa"), 75);

    // ---- access with get -> Option<&V> (safe, like vec.get — no panic) ----
    match scores.get("golam") {
        Some(s) => println!("golam's score : {s}"),
        None => println!("no score"),
    }
    // get gives Option<&i32>; .copied() -> Option<i32>, .unwrap_or() supplies a default
    let g = scores.get("nobody").copied().unwrap_or(0);
    println!("nobody (or 0) : {g}");

    // ---- membership ----
    println!("has mostafa?  : {}", scores.contains_key("mostafa"));

    // ---- OVERWRITE: insert with an existing key replaces the value ----
    scores.insert(String::from("golam"), 100);
    println!("golam now     : {:?}", scores.get("golam"));

    // ---- INSERT ONLY IF ABSENT: entry(...).or_insert(...) ----
    scores.entry(String::from("newbie")).or_insert(50);   // added (was absent)
    scores.entry(String::from("golam")).or_insert(0);     // golam exists -> unchanged
    println!("golam kept    : {:?}", scores.get("golam"));
    println!("newbie added  : {:?}", scores.get("newbie"));

    // ---- UPDATE BASED ON OLD VALUE: the classic word-count ----
    let text = "the quick brown the lazy the";
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);   // &mut to the value
        *count += 1;                                    // bump it through the reference
    }
    println!("word counts   : {:?}", counts);

    // ---- iterate (ORDER IS NOT GUARANTEED) ----
    println!("all scores:");
    for (name, score) in &scores {
        println!("   {name} -> {score}");
    }

    // ---- remove ----
    scores.remove("mostafa");
    println!("removed mostafa; still there? {}", scores.contains_key("mostafa"));
}