pub struct SectionAssignmentPair {
    pub left: SectionAssignment,
    pub right: SectionAssignment,
}

impl SectionAssignmentPair {
    pub fn new(section_assignment_pair: String) -> SectionAssignmentPair {
        let line_split: Vec<&str> = section_assignment_pair.split(",").collect();

        SectionAssignmentPair {
            left: SectionAssignment::new(line_split[0]),
            right: SectionAssignment::new(line_split[1]),
        }
    }
    pub fn overlaps(&self) -> bool {
        if self.right.start > self.left.start {
            self.right.start - self.left.end <= 0
        } else {
            self.left.start - self.right.end <= 0
        }
    }

    pub fn fully_overlaps(&self) -> bool {
        (self.left.start <= self.right.start && self.left.end >= self.right.end)
            || (self.right.start <= self.left.start && self.right.end >= self.left.end)
    }
}

pub struct SectionAssignment {
    pub start: i32,
    pub end: i32,
}

impl SectionAssignment {
    pub fn new(range_string: &str) -> SectionAssignment {
        let rng: Vec<&str> = range_string.split("-").collect();
        let start = rng[0].parse::<i32>().unwrap();
        let end = rng[1].parse::<i32>().unwrap();

        SectionAssignment { start, end }
    }
}
