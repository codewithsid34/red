use std::io::{stdin, Result};
use std::fs::File;
use std::io::Write;

const WRITE_MODE: u8 = 1 << 0;

struct State {
    lines: Vec<String>,
    line: usize,
    flags: u8
}

fn parse_cmd(state: &mut State, cmd: &str) {
    if state.flags & WRITE_MODE == 1 {
        if cmd == "." {
            state.flags &= !WRITE_MODE;
            return;
        }
        state.lines.insert(state.line, cmd.to_string());
        state.line += 1;
        return;
    }

    match cmd {
        "a" => {
            state.flags |= WRITE_MODE;
        },
        "w" => {
            let mut file = match File::create("foo.txt") {
                Ok(f) => f,
                Err(_e) => {
                    println!("?");
                    return;
                }
            };

            for line in &state.lines {
                if let Err(_e) = file.write(format!("{}\n", line).as_bytes()) {
                    println!("?");
                    return;
                }
            }
        },
        ",l" => {
            println!("{:?}", state.lines);
        },
        _ => {
            println!("?");
        }
    }
}

fn main() -> Result<()> {
    let mut state = State {
        lines : Vec::new(),
        line  : 0,
        flags : 0
    };
    let mut cmd = String::new();

    stdin().read_line(&mut cmd)?;
    cmd = cmd.as_str().trim_end().to_string();

    while cmd != "q" {
        parse_cmd(&mut state, &cmd.as_str());

        cmd = "".to_string();
        stdin().read_line(&mut cmd)?;
        cmd = cmd.as_str().trim_end().to_string();
    }

    Ok(())
}
