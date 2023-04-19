use sscanf::sscanf;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// fn generate_map() -> HashMap<char, i32> {}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        let mut any_overlap = 0;

        // Consumes the iterator, returns an (Optional) String
        for wrapped_line in lines {
            if let Ok(line) = wrapped_line {
                let parsed = sscanf!(line, "{i32}-{i32},{i32}-{i32}").unwrap();
                let left_elf_start = parsed.0;
                let left_elf_end = parsed.1;
                let right_elf_start = parsed.2;
                let right_elf_end = parsed.3;

                // left first, left range -> l_s <-> l_e,
                if right_elf_start >= left_elf_start && right_elf_start <= left_elf_end {
                    any_overlap += 1
                } else if right_elf_end >= left_elf_start && right_elf_end <= left_elf_end {
                    any_overlap += 1
                } else if left_elf_start >= right_elf_start && left_elf_start <= right_elf_end {
                    any_overlap += 1
                } else if left_elf_end >= right_elf_start && left_elf_end <= right_elf_end {
                    any_overlap += 1
                }
            }
        }

        println!("Total overlap: {any_overlap}");
    }
}
