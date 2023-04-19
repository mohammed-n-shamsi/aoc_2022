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

fn filter_tiny_trees(trees: Vec<u32>, min_tree_height: u32) -> bool {
    // check inverse, look for all trees greater than equal to current tree,
    // if any exist, then my current tree is not visible,
    // if none exist, then my current tree is visible

    // this loops through, and if any tree is greater than or equal to the min tree height
    // its added to the list
    let trees: Vec<_> = trees
        .iter()
        .filter(|iter_height| *iter_height >= &min_tree_height)
        .collect();
    // If the length of trees filter is greater than 0, then this tree is NOT visible
    // if the length is 0, then this is not visible
    trees.len() == 0
}

fn is_tree_visible(tree_grid: &Vec<Vec<u32>>, tree_row: usize, tree_col: usize) -> bool {
    let tree_height = tree_grid[tree_row][tree_col];
    let tree_row_len = tree_grid.first().unwrap().len();

    let print = false; // tree_row == 2 && tree_col == 2;

    // if edge return true
    if tree_row == 0
        || tree_col == 0
        || tree_row == tree_grid.len() - 1
        || tree_col == tree_row_len - 1
    {
        return true;
    }

    // check left
    let trees = tree_grid[tree_row][0..tree_col].to_vec();
    if print {
        println!("{:?}", trees);
    }
    // if tree is visible (all trees found in list are less than tree_height)
    if filter_tiny_trees(trees, tree_height) {
        return true;
    }

    // check right
    let trees = tree_grid[tree_row][tree_col + 1..tree_row_len].to_vec();
    if print {
        println!("{:?}", trees);
    }
    // if tree is visible (all trees found in list are less than tree_height)
    if filter_tiny_trees(trees, tree_height) {
        return true;
    }

    // check up
    let column: Vec<u32> = tree_grid
        .clone()
        .into_iter()
        .map(|s| s.into_iter().nth(tree_col).unwrap())
        .collect();
    let trees = column[0..tree_row].to_vec();
    if print {
        println!("{:?}", trees);
    }
    // if tree is visible (all trees found in list are less than tree_height)
    if filter_tiny_trees(trees, tree_height) {
        return true;
    }

    // check down
    let trees = column[tree_row + 1..tree_grid.len()].to_vec();
    if print {
        println!("{:?}", trees);
    }
    // if tree is visible (all trees found in list are less than tree_height)
    if filter_tiny_trees(trees, tree_height) {
        return true;
    }

    if print {
        println!("not visible");
    }

    false
}

fn tree_scenic_score(tree_grid: &Vec<Vec<u32>>, tree_row: usize, tree_col: usize) -> u32 {
    let tree_row_len = tree_grid.first().unwrap().len();
    let tree_height = tree_grid[tree_row][tree_col];

    let print = false; //tree_row == 3 && tree_col == 2;

    let mut left_score = 0;
    // count left
    if tree_col == 0 {
        left_score = 0;
    } else {
        for tree_idx in (0..tree_col).rev() {
            left_score += 1;
            if tree_grid[tree_row][tree_idx] >= tree_height {
                break;
            }
        }
    }

    if print {
        println!("Left score: {left_score}");
    }

    let mut right_score = 0;
    // count right
    if tree_col == tree_row_len - 1 {
        right_score = 0;
    } else {
        for tree_idx in tree_col + 1..tree_row_len {
            right_score += 1;
            if tree_grid[tree_row][tree_idx] >= tree_height {
                break;
            }
        }
    }
    if print {
        println!("right score: {right_score}");
    }

    let mut up_score = 0;
    // count up
    if tree_row == 0 {
        up_score = 0;
    } else {
        for tree_idx in (0..tree_row).rev() {
            up_score += 1;
            if tree_grid[tree_idx][tree_col] >= tree_height {
                break;
            }
        }
    }
    if print {
        println!("up score: {up_score}");
    }

    let mut down_score = 0;
    // count down
    if tree_row == tree_grid.len() - 1 {
        down_score = 0;
    } else {
        for tree_idx in tree_row + 1..tree_grid.len() {
            down_score += 1;
            if tree_grid[tree_idx][tree_col] >= tree_height {
                break;
            }
        }
    }
    if print {
        println!("down score: {down_score}");
    }

    left_score * right_score * up_score * down_score
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        const BASE_10: u32 = 10;
        let mut tree_grid: Vec<Vec<u32>> = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        for wrapped_line in lines {
            if let Ok(line) = wrapped_line {
                let tree_row: Vec<u32> = line
                    .chars()
                    .map(|char_val| char_val.to_digit(BASE_10).unwrap())
                    .collect();
                tree_grid.push(tree_row)
            }
        }
        let mut visible_count = 0;
        for tree_row in 0..tree_grid.len() {
            for tree_col in 0..tree_grid.first().unwrap().len() {
                if is_tree_visible(&tree_grid, tree_row, tree_col) {
                    // if (tree_row != 0
                    //     && tree_row != tree_grid.len() - 1
                    //     && tree_col != 0
                    //     && tree_col != tree_grid.first().unwrap().len() - 1)
                    // {
                    //     println!("Found at: {tree_row}, {tree_col}");
                    // }
                    visible_count += 1;
                }
            }
        }

        println!("Found {visible_count} visible trees!");

        let mut scenic_scores: Vec<u32> = Vec::new();

        for tree_row in 0..tree_grid.len() {
            for tree_col in 0..tree_grid.first().unwrap().len() {
                scenic_scores.push(tree_scenic_score(&tree_grid, tree_row, tree_col))
            }
        }
        println!("Max Scenic score: {:}", scenic_scores.iter().max().unwrap())
    }
}
