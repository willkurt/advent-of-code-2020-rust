use std::collections::HashSet;
use std::fmt;

pub fn part_2() {
    let mut program: Vec<Command> = Vec::new();
    let mut line_var: String = String::from("");

    with_read_lines!(
        "./data/day_8.txt",
        line_var,
        {
            program.push(to_command(line_var));
        },
        {
            let mut state = ProgState {
                codeln: 0,
                accum: 0,
            };
            println!("{}", term_in_n(&state, &program, 10));
            let prog_term = program.len() as i32;
            let mut fixed: bool = false;
            while state.codeln < prog_term {
                if fixed {
                    state = exec_cmd(&program[state.codeln as usize], state);
                } else {
                    let pair =
                        exec_cmd_correcting(&program[state.codeln as usize], state, &program);
                    state = pair.0;
                    fixed = pair.1;
                }
            }
            println!("{}", state);
        }
    );
}

pub fn part_1() {
    let mut program: Vec<Command> = Vec::new();
    let mut line_var: String = String::from("");

    with_read_lines!(
        "./data/day_8.txt",
        line_var,
        {
            program.push(to_command(line_var));
        },
        {
            let mut lines_seen: HashSet<i32> = HashSet::new();
            let mut state = ProgState {
                codeln: 0,
                accum: 0,
            };
            while !lines_seen.contains(&state.codeln) {
                lines_seen.insert(state.codeln);
                state = exec_cmd(&program[state.codeln as usize], state);
            }
            println!("{}", state);
        }
    );
}

// don't love this... but I do
fn term_in_n(state: &ProgState, program: &Vec<Command>, n: usize) -> bool {
    let mut intern_state = state.clone();
    let mut prgm_ctr: usize = 0;
    while prgm_ctr < n {
        if intern_state.codeln as usize == program.len() {
            println!("{} {}", intern_state.codeln, program.len());
            return true;
        }
        intern_state = exec_cmd(&program[intern_state.codeln as usize], intern_state);
        prgm_ctr += 1;
    }
    intern_state.codeln as usize == program.len()
}

fn exec_cmd_correcting(
    cmd: &Command,
    state: ProgState,
    program: &Vec<Command>,
) -> (ProgState, bool) {
    match cmd.op {
        Op::ACC => (exec_cmd(cmd, state), false),
        _ => {
            let alt_cmd = if cmd.op == Op::NOP {
                Command {
                    op: Op::JMP,
                    val: cmd.val,
                }
            } else {
                Command {
                    op: Op::NOP,
                    val: cmd.val,
                }
            };
            let alt_state = exec_cmd(&alt_cmd, state.clone());

            if term_in_n(&alt_state, program, 100) {
                println!("fixed it!");
                (exec_cmd(&alt_cmd, state), true)
            } else {
                (exec_cmd(cmd, state), false)
            }
        }
    }
}

fn exec_cmd(cmd: &Command, state: ProgState) -> ProgState {
    match cmd.op {
        Op::NOP => ProgState {
            codeln: state.codeln + 1,
            accum: state.accum,
        },
        Op::JMP => ProgState {
            codeln: state.codeln + cmd.val,
            accum: state.accum,
        },
        Op::ACC => ProgState {
            codeln: state.codeln + 1,
            accum: state.accum + cmd.val,
        },
    }
}

fn to_command(line: String) -> Command {
    let parts = line.split(' ').collect::<Vec<&str>>();
    match parts[0] {
        "acc" => Command {
            op: Op::ACC,
            val: parts[1].parse::<i32>().unwrap(),
        },
        "nop" => Command {
            op: Op::NOP,
            val: parts[1].parse::<i32>().unwrap(),
        },
        "jmp" => Command {
            op: Op::JMP,
            val: parts[1].parse::<i32>().unwrap(),
        },
        _ => Command {
            op: Op::JMP,
            val: 0,
        },
    }
}

#[derive(Debug, Copy, Clone)]
struct ProgState {
    codeln: i32,
    accum: i32,
}

impl fmt::Display for ProgState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "l:{} acc:{}", self.codeln, self.accum)
    }
}

#[derive(PartialEq)]
enum Op {
    NOP,
    ACC,
    JMP,
}

struct Command {
    op: Op,
    val: i32,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.op {
            Op::NOP => {
                write!(f, "NOP {}", self.val.to_string())
            }
            Op::ACC => {
                write!(f, "ACC {}", self.val.to_string())
            }
            Op::JMP => {
                write!(f, "JMP {}", self.val.to_string())
            }
        }
    }
}
