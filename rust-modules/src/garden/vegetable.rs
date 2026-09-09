// A closed set of seasons — invalid seasons are now IMPOSSIBLE to express.
#[derive(Debug, Clone, Copy)]
pub enum Season {
    Summer,
    Winter,
    Spring,
    Autumn,
}

impl Season {
    // Each season knows its own price adjustment. No `_` arm: the match is
    // exhaustive, so adding a new season later forces you to handle it here.
    fn price_change(&self) -> i32 {
        match self {
            Season::Summer => 10,
            Season::Winter => 5,
            Season::Spring => 0,
            Season::Autumn => 2,
        }
    }
}

#[derive(Debug)]
pub struct Alu {
    pub name: String,
    pub price: i32,
}

impl Alu {
    // Associated function: the idiomatic constructor.
    pub fn new(name: &str, price: i32) -> Alu {
        Alu { name: String::from(name), price }
    }

    // Method: &mut self is passed automatically when you call alu.apply_season(..)
    pub fn apply_season(&mut self, season: Season) {
        self.price += season.price_change();
    }
}