use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
        let mut calorie_per_elf: Vec<i64> = Vec::new();

        let mut elf_calorie: i64 = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calorie) = line {
                if let Ok(food_calorie) = calorie.parse::<i64>() {
                    elf_calorie += food_calorie;
                } else {
                    calorie_per_elf.push(elf_calorie);
                    elf_calorie = 0;
                }
            }
        }

        calorie_per_elf.sort();
        calorie_per_elf.reverse();

        let print_range = 3;

        let mut total_in_print_range: i64 = 0;
        for idx in 0..print_range {
            println!("Rank {:}, calorie is {:}", idx, calorie_per_elf[idx]);
            total_in_print_range += calorie_per_elf[idx];
        }

        println!(
            "Total in {:} range is: {:}",
            print_range, total_in_print_range
        )
    }
}
