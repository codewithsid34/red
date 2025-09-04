mod lexer;

use std::io::{
    stdin,
    BufReader,
    BufRead,
    Result,
    Write
};
use std::fs::File;

use lexer::*;

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
            state.line -= 1;
            return;
        }
        state.lines.insert(state.line, cmd.to_string());
        state.line += 1;
        state.bytes += cmd.len() + 1;
        return;
    }

    let mut lex = Lexer::new(cmd);
    let mut token = lex.next();

    while token != TokenTypes::End {
        // Direct command (eg: l)
        if token == TokenTypes::Word {
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
                _ => {
                    println!("?");
                }
            }
        }

        // Half range (eg: ,l ,2l)
        else if token == TokenTypes::Comma {
            let mut second = state.lines.len();

            token = lex.next();
            if token == TokenTypes::Number {
                second = lex.num_data;
                token = lex.next();
            }

            if token != TokenTypes::Word {
                println!("?");
                return;
            }

            let c = lex.str_data.as_str();

            match c {
                "l" => {
                    for line in &state.lines[0.. second] {
                        println!("{}$", line);
                    }
                },
                _ => {
                    println!("?");
                }
            }
        }

        // Full range (eg: 1,3l)
        else if token == TokenTypes::Number {
            let first = lex.num_data;
            token = lex.next();

            if token != TokenTypes::Comma {
                println!("?");
                return;
            }

            token = lex.next();
            if token != TokenTypes::Number{
                println!("?");
                return;
            }

            let second = lex.num_data;

            token = lex.next();
            if token != TokenTypes::Word {
                println!("?");
                return;
            }

            let c = lex.str_data.as_str();

            match c {
                "l" => {
                    for line in &state.lines[first-1.. second] {
                        println!("{}$", line);
                    }
                },
                _ => {
                    println!("?");
                }
            }
        }

        token = lex.next();
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
