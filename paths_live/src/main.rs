use paths_live::{Alu, Season};

fn main() {
    let mut alu = Alu::new("potato", 10);
    println!("start        : {:#?}", alu);
    alu.apply_season(Season::Summer);
    println!("after Summer : {:#?}", alu);
    alu.apply_season(Season::Winter);
    println!("after Winter : {:#?}", alu);
}