use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const SIZE_CHECK: usize = 14;

fn check_all_elements(char_q: &VecDeque<char>) -> bool {
    let mut elements = HashSet::new();

    for char in char_q {
        elements.insert(*char);
    }

    return elements.len() == SIZE_CHECK;
}

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
        // Consumes the iterator, returns an (Optional) String
        for wrapped_line in lines {
            if let Ok(line) = wrapped_line {
                let char_vec: Vec<char> = line.chars().collect();
                let mut char_q: VecDeque<char> = VecDeque::new();
                for (idx, char) in char_vec.iter().enumerate() {
                    if char_q.len() == SIZE_CHECK {
                        _ = char_q.pop_front();
                        char_q.push_back(*char);

                        if check_all_elements(&char_q) {
                            println!("{:?}", char_q);

                            println!("Found at iter: {:}", idx + 1);
                            return;
                        }
                    } else {
                        char_q.push_back(*char);
                    }
                }
            }
        }
    }
}
