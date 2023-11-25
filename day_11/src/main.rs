// use sscanf::sscanf;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
enum Operation {
    None,
    Add(u128),
    Mult(u128),
    Double,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u128>,
    op: Operation,
    div_test: u128,
    true_monkey_idx: usize,
    false_monkey_idx: usize,
    inspection_count: usize,
}

fn parse_item_list(str_item_list_unclean: String, item_list: &mut Vec<u128>) {
    let str_item_list = str_item_list_unclean
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let str_item_list_split = str_item_list.split(',');

    for str_item in str_item_list_split {
        let int_item: u128 = str_item.parse().unwrap();
        item_list.push(int_item)
    }
}

impl Monkey {
    fn new(monkey_data_full: String) -> Self {
        let monkey_data = monkey_data_full.split("\n");

        let mut item_list: Vec<u128> = Vec::new();
        let mut operation: Operation = Operation::None;
        let mut div_value: u128 = 0;
        let mut true_monkey_throw: usize = 0;
        let mut false_monkey_throw: usize = 0;

        for monkey_line in monkey_data {
            let parsed = sscanf::sscanf!(monkey_line, "  Starting items: {String}");
            if let Ok(str_item_list_unclean) = parsed {
                parse_item_list(str_item_list_unclean, &mut item_list);
                continue;
            }

            let parsed = sscanf::sscanf!(monkey_line, "  Operation: new = old {char} {u128}");
            if let Ok((op, count)) = parsed {
                operation = match op {
                    '*' => Operation::Mult(count),
                    '+' => Operation::Add(count),
                    _ => Operation::None,
                };
                continue;
            } else {
                let parsed = sscanf::scanf!(monkey_line, "  Operation: new = old {char} {String}");
                if let Ok((_op, _count)) = parsed {
                    operation = Operation::Double;
                }
            }

            let parsed = sscanf::sscanf!(monkey_line, "  Test: divisible by {u128}");
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
            inspection_count: 0,
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
                if line.is_empty() {
                    continue;
                }

                if counter < 5 {
                    monkey_data += &(line + "\n");
                } else if counter == 5 {
                    monkey_data += &line;
                }
                counter += 1;
                if counter == 6 {
                    monkey_lines.push(monkey_data.clone());
                    monkey_data = String::new();
                    counter = 0;
                }
            }
        }
        for monkey in monkey_lines {
            monkeys.push(Monkey::new(monkey))
        }

        // Will define max worry level as the product over all divisible tests
        // such that it will be a number that is divisibly by all
        let mut worry_level_max = 1;

        for monkey in &monkeys {
            worry_level_max *= monkey.div_test;
            println!("{:?}", monkey)
        }

        let rounds = 10000;
        for _i in 0..rounds {
            for monkey_id in 0..monkeys.len() {
                let monkey = monkeys[monkey_id].clone();
                monkeys[monkey_id].inspection_count += monkeys[monkey_id].items.len();
                monkeys[monkey_id].items.clear();

                for item in monkey.items {
                    let mut worry_level: u128 = item;
                    match monkey.op {
                        Operation::Add(modifier) => worry_level += modifier,
                        Operation::Mult(modifier) => worry_level *= modifier,
                        Operation::Double => worry_level *= worry_level,
                        Operation::None => {
                            println!("Handling a no-op")
                        }
                    };

                    // No longer for part2
                    // worry_level /= 3;
                    worry_level %= worry_level_max;

                    let insert_idx;
                    if worry_level % monkey.div_test == 0 {
                        insert_idx = monkey.true_monkey_idx;
                    } else {
                        insert_idx = monkey.false_monkey_idx;
                    }
                    monkeys[insert_idx].items.push(worry_level);
                }
            }
        }

        println!("------------------------- {rounds} rounds");
        let mut inspection_counts: Vec<u128> = Vec::new();
        for monkey in &monkeys {
            println!("{:?}", monkey)
        }

        for monkey in &monkeys {
            inspection_counts.push(monkey.inspection_count.try_into().unwrap());
        }

        inspection_counts.sort();
        inspection_counts.reverse();

        if inspection_counts.len() >= 2 {
            println!(
                "Monkey business is: {}",
                (inspection_counts[0] * inspection_counts[1])
            );
        }
    }
}
