use std::io::{
    stdin,
    BufReader,
    BufRead,
    Result,
    Write
};
use std::fs::File;

const WRITE_MODE: u8 = 1 << 0;

struct State {
    file_path: String,
    bytes: usize,
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
        state.bytes += cmd.len() + 1;
        return;
    }

    match cmd {
        "a" => {
            state.flags |= WRITE_MODE;
        },
        "w" => {
            let mut file = match File::create(state.file_path.as_str()) {
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
            println!("{}", state.bytes);
        },
        "l" => {
            println!("{}$", state.lines[state.line]);
        },
        ",l" => {
            for line in &state.lines {
                println!("{}$", line);
            }
        },
        _ => {
            println!("?");
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        eprintln!("Error: File missing.");
        std::process::exit(1)
    }

    let mut state = State {
        file_path: args[1].clone(),
        lines:     Vec::new(),
        line :     0,
        bytes:     0,
        flags:     0
    };
    let mut cmd = String::new();

    let file = File::open(state.file_path.as_str())?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line?;
        state.lines.push(l.as_str().to_string());
        state.bytes += l.len() + 1;
    }

    println!("{}", state.bytes);

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
