use sscanf::sscanf;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn generate_init_vec() -> Vec<Vec<char>> {
    vec![
        vec!['b', 'g', 's', 'c'],
        vec!['t', 'm', 'w', 'h', 'j', 'n', 'v', 'g'],
        vec!['m', 'q', 's'],
        vec!['b', 's', 'l', 't', 'w', 'n', 'm'],
        vec!['j', 'z', 'f', 't', 'v', 'g', 'w', 'p'],
        vec!['c', 't', 'b', 'g', 'q', 'h', 's'],
        vec!['t', 'j', 'p', 'b', 'w'],
        vec!['g', 'd', 'c', 'z', 'f', 't', 'q', 'm'],
        vec!['n', 's', 'h', 'b', 'p', 'f'],
    ]
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
        let mut crates = generate_init_vec();
        // Consumes the iterator, returns an (Optional) String
        for wrapped_line in lines {
            if let Ok(line) = wrapped_line {
                let parsed = sscanf!(line, "move {usize} from {usize} to {usize}").unwrap();
                let count = parsed.0;
                let from_idx = parsed.1 - 1;
                let to_idx = parsed.2 - 1;

                let mut popped_data: Vec<char> = Vec::new();

                for _idx in 0..count {
                    let some_val = crates[from_idx].pop();
                    if let Some(val) = some_val {
                        popped_data.push(val);
                    }
                }

                for idx in (0..count).rev() {
                    crates[to_idx].push(popped_data[idx])
                }
            }
        }
        for idx in 0..crates.len() {
            print!("{:}", crates[idx].last().unwrap())
        }
    }
}
