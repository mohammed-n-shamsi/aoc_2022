use sscanf::sscanf;
use std::collections::{BTreeMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NOOP_DURATION: i32 = 1;
const ADDX_DURATION: i32 = 2;

#[derive(sscanf::FromScanf, Copy, Clone, Debug)]
enum Inst {
    #[sscanf("noop")]
    Noop,
    #[sscanf("addx")]
    Addx,
}

struct Job {
    cycle_at_expire: i32,
    job_content: i32,
}

struct CPU {
    exp_cycle: i32,
    cycle: i32,
    reg_x: i32,
    job_queue: VecDeque<Job>,
    snapshot_vals: BTreeMap<i32, i32>,
}

struct CRT {
    screen: Vec<Vec<char>>,
    sprite_row: Vec<char>,
}

impl CPU {
    fn new() -> Self {
        CPU {
            exp_cycle: 0,
            cycle: 0,
            reg_x: 1,
            job_queue: VecDeque::new(),
            snapshot_vals: BTreeMap::new(),
        }
    }

    fn add_job(&mut self, job_content: i32, inst: Inst) {
        let new_job = match inst {
            Inst::Noop => Job {
                cycle_at_expire: self.exp_cycle + NOOP_DURATION,
                job_content,
            },
            Inst::Addx => Job {
                cycle_at_expire: self.exp_cycle + ADDX_DURATION,
                job_content,
            },
        };
        self.exp_cycle = new_job.cycle_at_expire;
        self.job_queue.push_back(new_job)
    }

    fn handle_rising_edge(&mut self) {
        self.cycle += 1;

        self.snapshot_vals.insert(self.cycle, self.reg_x);
    }

    fn handle_falling_edge(&mut self) {
        if self.cycle >= self.job_queue.front().unwrap().cycle_at_expire {
            let job = self.job_queue.pop_front().unwrap();
            self.reg_x += job.job_content;
        }
        if !self.job_queue.is_empty() {
            if self.cycle >= self.job_queue.front().unwrap().cycle_at_expire {
                let job = self.job_queue.pop_front().unwrap();
                self.reg_x += job.job_content;
            }
        }
        if !self.job_queue.is_empty() {
            if self.cycle >= self.job_queue.front().unwrap().cycle_at_expire {
                let job = self.job_queue.pop_front().unwrap();
                self.reg_x += job.job_content;
            }
        }
    }
}

impl CRT {
    fn new() -> Self {
        let screen = vec![vec!['.'; 40]; 6];
        let sprite_row = vec!['.'; 40];
        CRT { screen, sprite_row }
    }
    fn render_sprite(&mut self, sprite_pos: i32) {
        self.sprite_row = vec!['.'; 40];
        if (0..40).contains(&sprite_pos) {
            let sprite_pos: usize = sprite_pos.try_into().unwrap();
            if sprite_pos == 0 {
                self.sprite_row[0] = '#';
                self.sprite_row[1] = '#';
            } else if sprite_pos == (self.sprite_row.len() - 1) as usize {
                self.sprite_row[sprite_pos] = '#';
                self.sprite_row[sprite_pos - 1] = '#';
            } else {
                for sprite_idx in sprite_pos - 1..=sprite_pos + 1 {
                    self.sprite_row[sprite_idx] = '#';
                }
            }
        }
    }
    fn render_screen(&mut self, cycle: usize) {
        let (row, cycle) = match cycle {
            1..=40 => (0, cycle),
            41..=80 => (1, cycle - 40),
            81..=120 => (2, cycle - 80),
            121..=160 => (3, cycle - 120),
            161..=200 => (4, cycle - 160),
            201..=240 => (5, cycle - 200),
            _ => (6, cycle - 240),
        };
        if row < 6 {
            self.screen[row][cycle - 1] = self.sprite_row[cycle - 1]
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
        let mut cpu = CPU::new();
        // Consumes the iterator, returns an (Optional) String
        for wrapped_line in lines {
            if let Ok(line) = wrapped_line {
                let parsed = sscanf!(line, "{Inst} {i32}");
                if let Ok((inst, val)) = parsed {
                    cpu.add_job(val, inst);
                } else {
                    cpu.add_job(0, Inst::Noop);
                }
            }
        }
        while !cpu.job_queue.is_empty() {
            cpu.handle_rising_edge();
            cpu.handle_falling_edge();
        }
        let mut crt = CRT::new();
        for (cycle, val) in cpu.snapshot_vals {
            crt.render_sprite(val);
            crt.render_screen(cycle as usize);
        }
        for row in 0..6 {
            println!("{:?}", crt.screen[row])
        }
    }
}
