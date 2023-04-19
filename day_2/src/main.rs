use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn create_win_loss_map() -> HashMap<String, i32> {
    let mut win_loss_map = HashMap::new();

    // win & loss w.r.t to second players option

    // Rock & Loss (scissor)
    win_loss_map.insert(String::from("A X"), 0 + 3);
    // Rock & Draw (rock)
    win_loss_map.insert(String::from("A Y"), 3 + 1);
    // Rock & Win (paper)
    win_loss_map.insert(String::from("A Z"), 6 + 2);

    // Paper & Loss (rock)
    win_loss_map.insert(String::from("B X"), 0 + 1);
    // Paper & Draw (paper)
    win_loss_map.insert(String::from("B Y"), 3 + 2);
    // Paper & Win (Scissor)
    win_loss_map.insert(String::from("B Z"), 6 + 3);

    // Scissor & Loss (paper)
    win_loss_map.insert(String::from("C X"), 0 + 2);
    // Scissor & Draw (scissor)
    win_loss_map.insert(String::from("C Y"), 3 + 3);
    // Scissor & Win (rock)
    win_loss_map.insert(String::from("C Z"), 6 + 1);

    return win_loss_map;
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
        let win_loss_map = create_win_loss_map();

        let mut points = 0;

        for line in lines {
            if let Ok(round_suggestion) = line {
                points += win_loss_map[&round_suggestion]
            }
        }

        println!("Total points {points}")
    }
}
