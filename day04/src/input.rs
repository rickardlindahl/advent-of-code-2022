use crate::section_assignment::SectionAssignmentPair;
use shared::{open_file, read_lines};

pub fn get_section_assignments_from_input(
    input_file_path: &str,
    section_assignment_pairs: &mut Vec<SectionAssignmentPair>,
) {
    let buf_reader = open_file(input_file_path);

    read_lines(buf_reader, |line| {
        add_new_section_assignment(section_assignment_pairs, line);
    });
}

fn add_new_section_assignment(
    section_assignment_pairs: &mut Vec<SectionAssignmentPair>,
    line: String,
) {
    section_assignment_pairs.push(SectionAssignmentPair::new(line));
}
