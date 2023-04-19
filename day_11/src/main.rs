use sscanf::sscanf;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Operation {
    Add(i32),
    Double,
    Mult(i32),
    Square,
}

struct Monkey {
    items: Vec<i32>,
    op: Operation,
    div_test: i32,
    true_monkey_idx: usize,
    false_monkey_idx: usize,
}

impl Monkey {
    fn new(monkey_data: String) -> Self {
        println!("{monkey_data}");
        let parse_string: String = r#"Monkey {i32}:\n
        Starting items: {String}\n
        Operation: new = old {String} {String}\n
        Test: divisible by {i32}\n
          If true: throw to monkey {i32}\n
          If false: throw to monkey {i32}"#
            .to_string();

        println!("{parse_string}");
        let parsed = sscanf::sscanf!(
            monkey_data,
            "Monkey {i32}:\n
   Starting items: {String}\n
  Operation: new = old {String} {String}\n
  Test: divisible by {i32}\n
    If true: throw to monkey {i32}\n
    If false: throw to monkey {i32}"
        );
        println!("{:?}", parsed);
        Monkey {
            items: Vec::new(),
            op: Operation::Double,
            div_test: 0,
            true_monkey_idx: 0,
            false_monkey_idx: 0,
        }
    }
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
        let mut monkeys: Vec<Monkey> = Vec::new();

        let mut monkey_lines: Vec<String> = Vec::new();
        let mut monkey_data: String = String::new();
        let mut counter = 0;
        // Consumes the iterator, returns an (Optional) String
        for wrapped_line in lines {
            if let Ok(line) = wrapped_line {
                if counter < 5 {
                    monkey_data += &(line + "\n");
                } else if counter == 5 {
                    monkey_data += &line;
                }
                counter += 1;
                if counter == 7 {
                    monkey_lines.push(monkey_data.clone());
                    monkey_data = String::new();
                    counter = 0;
                }
            }
        }
        for monkey in monkey_lines {
            monkeys.push(Monkey::new(monkey))
        }
    }
}
