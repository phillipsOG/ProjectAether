#[derive(Copy, Clone)]
pub struct Status {
    pub health: i32,
    pub str: i32,
    pub def: i32,
}

impl Status {
    pub(crate) fn new() -> Self {
        Status {
            health: 100,
            str: 3,
            def: 1,
        }
    }

    pub(crate) fn new_monster(health: i32, str: i32, def: i32) -> Self {
        Status {
            health,
            str,
            def,
        }
    }

    fn print_status(&mut self) {
        println!("HP: {}", self.health);
        println!("STR: {}", self.str);
        println!("DEF: {}", self.def);
    }

    pub(crate) fn get_status(&mut self) -> [String; 3] {
        [
            format!("HP: {}", self.health),
            format!("STR: {}", self.str),
            format!("DEF: {}", self.def),
        ]
    }
}
