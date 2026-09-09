use crate::garden::vegetable::{Alu, Season};

pub mod garden;

fn main() {
    let mut alu: Alu = Alu::new("potato", 10);
    println!("start        : {:#?}", alu);
    alu.apply_season(Season::Summer);       
    println!("after Summer : {:#?}", alu);
 
    alu.apply_season(Season::Winter);
    println!("after Winter : {:#?}", alu);
}
