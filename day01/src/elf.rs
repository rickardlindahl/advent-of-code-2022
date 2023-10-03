pub struct Elf {
    pub calories: u32,
}

impl Elf {
    pub fn new() -> Self {
        Elf {
            calories: 0,
        }
    }

    pub fn add_calories(&mut self, calories: u32) {
        self.calories += calories;
    }
}
