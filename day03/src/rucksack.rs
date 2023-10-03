pub struct Rucksack {
    pub items: String,
    pub left_compartment: String,
    pub right_compartment: String,
}

impl Rucksack {
    pub fn new(items: String) -> Rucksack {
        let split_index = items.len() / 2;

        let left_compartment = items[..split_index].to_string();
        let right_compartment = items[split_index..].to_string();

        Rucksack {
            items,
            left_compartment,
            right_compartment,
        }
    }

    pub fn find_duplicate_item(&self) -> Option<char> {
        for c1 in self.left_compartment.chars() {
            if self.right_compartment.contains(c1) {
                return Some(c1);
            }
        }
        None
    }
}

pub fn find_badge(
    rucksack1: &Rucksack,
    rucksack2: &Rucksack,
    rucksack3: &Rucksack,
) -> Option<char> {
    for c1 in rucksack1.items.chars() {
        if rucksack2.items.contains(c1) && rucksack3.items.contains(c1) {
            return Some(c1);
        }
    }
    None
}
