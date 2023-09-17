#[derive(Copy, Clone)]
pub struct Status {
    pub max_health: i32,
    pub current_health: i32,
    pub str: i32,
    pub def: i32,
}

impl Status {
    pub(crate) fn new() -> Self {
        Status {
            max_health: 100,
            current_health: 100,
            str: 3,
            def: 1,
        }
    }

    pub(crate) fn new_monster(health: i32, str: i32, def: i32) -> Self {
        Status { max_health: health, current_health: health, str, def }
    }

    fn print_status(&mut self) {
        println!("HP: {}", self.max_health);
        println!("STR: {}", self.str);
        println!("DEF: {}", self.def);
    }

    pub(crate) fn get_status(&mut self) -> [String; 3] {
        [
            format!("HP: {}", self.max_health),
            format!("STR: {}", self.str),
            format!("DEF: {}", self.def),
        ]
    }
}
