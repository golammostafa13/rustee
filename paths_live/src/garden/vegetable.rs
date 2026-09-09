#[derive(Debug, Clone, Copy)]
pub enum Season { Summer, Winter, Spring, Autumn }

impl Season {
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
pub struct Alu { pub name: String, pub price: i32 }

impl Alu {
    pub fn new(name: &str, price: i32) -> Alu {
        Alu { name: String::from(name), price }
    }
    pub fn apply_season(&mut self, season: Season) {
        self.price += season.price_change();
    }
}