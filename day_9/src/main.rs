use sscanf::sscanf;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const UNIT: i32 = 1;

#[derive(sscanf::FromScanf, Copy, Clone, Debug)]
enum MoveDir {
    #[sscanf("U")]
    Up,
    #[sscanf("D")]
    Down,
    #[sscanf("L")]
    Left,
    #[sscanf("R")]
    Right,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Rope {
    head: Position,
    tail: Position,
    tail_store: HashSet<Position>,
}

struct RopeOfRopes {
    ropes: Vec<Rope>,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn move_loc(&mut self, dir: &MoveDir) {
        match dir {
            MoveDir::Up => self.y += UNIT,
            MoveDir::Down => self.y -= UNIT,
            MoveDir::Left => self.x -= UNIT,
            MoveDir::Right => self.x += UNIT,
        };
    }

    fn on_axis(&self, other: &Position) -> bool {
        self.x == other.x || self.y == other.y
    }

    fn touching(&self, other: &Position) -> bool {
        if self.on_axis(other) {
            ((self.y - other.y) + (self.x - other.x)).abs() <= 1
        } else {
            (self.y - other.y).abs() == 1 && (self.x - other.x).abs() == 1
        }
    }

    fn follow_direction(&self, other: &Position) -> Vec<MoveDir> {
        if self.on_axis(other) {
            if self.x == other.x {
                if other.y > self.y {
                    vec![MoveDir::Up]
                } else {
                    vec![MoveDir::Down]
                }
            } else {
                if other.x > self.x {
                    vec![MoveDir::Right]
                } else {
                    vec![MoveDir::Left]
                }
            }
        } else {
            let mut dirs = Vec::new();
            if other.y > self.y {
                dirs.push(MoveDir::Up)
            } else {
                dirs.push(MoveDir::Down)
            }

            if other.x > self.x {
                dirs.push(MoveDir::Right)
            } else {
                dirs.push(MoveDir::Left)
            }
            dirs
        }
    }
}

impl Rope {
    fn new() -> Self {
        let mut new_store = HashSet::new();
        new_store.insert(Position::new());
        Rope {
            head: Position::new(),
            tail: Position::new(),
            tail_store: new_store,
        }
    }

    fn move_loc(&mut self, dirs: &Vec<MoveDir>, count: i32) {
        for _ in 0..count {
            for dir in dirs {
                self.head.move_loc(&dir);
            }
            if !self.head.touching(&self.tail) {
                for follow_dir in self.tail.follow_direction(&self.head) {
                    self.tail.move_loc(&follow_dir);
                }
                self.add_loc();
            }
        }
    }

    fn add_loc(&mut self) {
        self.tail_store.insert(self.tail.clone());
    }

    fn unique_tail_locs(&self) -> usize {
        self.tail_store.len()
    }
}

impl RopeOfRopes {
    fn new(count: usize) -> Self {
        RopeOfRopes {
            ropes: vec![Rope::new(); count],
        }
    }

    fn move_loc(&mut self, dir: MoveDir, count: i32) {
        for _ in 0..count {
            let mut move_dirs = vec![dir];
            let mut last_tail_pos = Position::new();
            let mut last_head_pos = Position::new();
            let mut first_time = true;
            let mut print = false;
            for (idx, curr_rope) in self.ropes.iter_mut().enumerate() {
                if idx == 9 {
                    // print = true;
                }
                if first_time {
                    first_time = false;
                    curr_rope.move_loc(&move_dirs, UNIT);
                    if print {
                        println!("-------------------------------");
                        // println!("move_dirs -> {:?}", move_dirs);
                    }
                    if print {
                        println!("adding: {:?}", curr_rope.tail);
                    }
                } else {
                    move_dirs = curr_rope.head.follow_direction(&last_tail_pos);
                    if print {
                        println!("-------------------------------");
                        // println!("move_dirs -> {:?}", move_dirs);
                    }
                    if !curr_rope.head.touching(&last_head_pos) {
                        curr_rope.move_loc(&move_dirs, UNIT);
                        if print {
                            println!("adding: {:?}", curr_rope.tail);
                        }
                    }
                }

                if print {
                    // println!("Head loc: {:?}", curr_rope.head);
                    // println!("Last tail loc:{:?}", last_tail_pos);
                    // println!("Tail loc: {:?}", curr_rope.tail);
                }
                print = false;

                last_tail_pos = curr_rope.tail.clone();
                last_head_pos = curr_rope.head.clone();
            }
        }
    }

    fn unique_tail_locs(&self) -> usize {
        self.ropes[self.ropes.len() - 2].unique_tail_locs()
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
        let mut rope = RopeOfRopes::new(10);

        // Consumes the iterator, returns an (Optional) String
        for wrapped_line in lines {
            if let Ok(line) = wrapped_line {
                let parsed = sscanf!(line, "{MoveDir} {i32}").unwrap();
                let dir = parsed.0;
                let count = parsed.1;
                rope.move_loc(dir, count)
            }
        }

        println!("Unique locs: {:}", rope.unique_tail_locs())
    }
}
