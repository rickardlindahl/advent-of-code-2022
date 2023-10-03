pub struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    pub fn new() -> Self {
        Elf {
            calories: Vec::new(),
        }
    }

    pub fn add_calories(&mut self, calories: u32) {
        self.calories.push(calories);
    }

    pub fn total_calories(&self) -> u32 {
        self.calories
            .clone()
            .into_iter()
            .reduce(|acc, e| acc + e)
            .unwrap_or(0_u32)
    }
}
