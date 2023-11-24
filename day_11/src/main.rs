use sscanf::sscanf;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const PARSE_INPUT_LINES: [&str; 6] = [
    "Monkey {i32}",
    "  Starting items: {Vec<i32>}",
    "  Operation: new = old {String} {i32}",
    "  Test: divisible by {i32}",
    "    If true: throw to monkey {i32}",
    "    If false: throw to monkey {i32}",
];

#[derive(Debug)]
enum Operation {
    None,
    Add(i32),
    Mult(i32),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    op: Operation,
    div_test: i32,
    true_monkey_idx: usize,
    false_monkey_idx: usize,
}

fn parse_item_list(str_item_list_unclean: String, item_list: &mut Vec<i32>) {
    let str_item_list = str_item_list_unclean
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let str_item_list_split = str_item_list.split(',');

    for str_item in str_item_list_split {
        let int_item: i32 = str_item.parse().unwrap();
        item_list.push(int_item)
    }
}

impl Monkey {
    fn new(monkey_data_full: String) -> Self {
        let monkey_data = monkey_data_full.split("\n");

        let mut item_list: Vec<i32> = Vec::new();
        let mut operation: Operation = Operation::None;
        let mut div_value: i32 = 0;
        let mut true_monkey_throw: usize = 0;
        let mut false_monkey_throw: usize = 0;

        for monkey_line in monkey_data {
            let parsed = sscanf::sscanf!(monkey_line, "  Starting items: {String}");
            if let Ok(str_item_list_unclean) = parsed {
                parse_item_list(str_item_list_unclean, &mut item_list);
                continue;
            }

            let parsed = sscanf::sscanf!(monkey_line, "  Operation: new = old {char} {i32}");
            if let Ok((op, count)) = parsed {
                operation = match op {
                    '*' => Operation::Mult(count),
                    '+' => Operation::Add(count),
                    _ => Operation::None,
                };
                continue;
            }

            let parsed = sscanf::sscanf!(monkey_line, "  Test: divisible by {i32}");
            if let Ok(div_test_val) = parsed {
                div_value = div_test_val;
                continue;
            }

            let parsed = sscanf::sscanf!(monkey_line, "    If {bool}: throw to monkey {usize}");
            if let Ok((case, monkey_to)) = parsed {
                if case {
                    true_monkey_throw = monkey_to
                } else {
                    false_monkey_throw = monkey_to
                }
                continue;
            }
        }
        Monkey {
            items: item_list,
            op: operation,
            div_test: div_value,
            true_monkey_idx: true_monkey_throw,
            false_monkey_idx: false_monkey_throw,
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
        for monkey in monkeys {
            println!("{:?}", monkey)
        }
    }
}
