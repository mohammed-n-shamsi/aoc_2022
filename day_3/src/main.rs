use maplit;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn generate_map() -> HashMap<char, i32> {
    maplit::hashmap! {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 26 + 1,
        'B' => 26 + 2,
        'C' => 26 + 3,
        'D' => 26 + 4,
        'E' => 26 + 5,
        'F' => 26 + 6,
        'G' => 26 + 7,
        'H' => 26 + 8,
        'I' => 26 + 9,
        'J' => 26 + 10,
        'K' => 26 + 11,
        'L' => 26 + 12,
        'M' => 26 + 13,
        'N' => 26 + 14,
        'O' => 26 + 15,
        'P' => 26 + 16,
        'Q' => 26 + 17,
        'R' => 26 + 18,
        'S' => 26 + 19,
        'T' => 26 + 20,
        'U' => 26 + 21,
        'V' => 26 + 22,
        'W' => 26 + 23,
        'X' => 26 + 24,
        'Y' => 26 + 25,
        'Z' => 26 + 26,
    }
}

fn find_matching_item(
    list_one: &Vec<char>,
    list_two: &Vec<char>,
    list_three: &Vec<char>,
) -> Option<i32> {
    let prio_map = generate_map();
    for item_one in list_one {
        for item_two in list_two {
            for item_three in list_three {
                if prio_map[item_one] == prio_map[item_two]
                    && prio_map[item_one] == prio_map[item_three]
                    && prio_map[item_two] == prio_map[item_three]
                {
                    return Some(prio_map[item_one]);
                }
            }
        }
    }
    return None;
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
        let mut total_prio: i32 = 0;
        let mut counter: i32 = 0;

        let mut elf_one = Vec::new();
        let mut elf_two = Vec::new();
        let mut elf_three = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(sack_contents) = line {
                let sack_chars: Vec<char> = sack_contents.chars().collect();
                if counter % 3 == 0 {
                    elf_one = sack_chars;
                } else if counter % 3 == 1 {
                    elf_two = sack_chars;
                } else {
                    elf_three = sack_chars;
                }

                if counter % 3 == 2 {
                    let dup_item = find_matching_item(&elf_one, &elf_two, &elf_three);
                    if let Some(found_item) = dup_item {
                        total_prio += found_item;
                    }
                }
                counter += 1;
            }
        }

        println!("Summed prio is {total_prio}");
    }
}

