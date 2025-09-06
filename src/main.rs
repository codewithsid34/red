mod lexer;

use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write, stdin};

use lexer::*;

const WRITE_MODE: u8 = 1 << 0;

struct State {
    file_path: Option<String>,
    bytes: usize,
    lines: Vec<String>,
    line: usize,
    flags: u8,
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
        // Search
        if token == TokenTypes::Slash {
            token = lex.next();
            if token != TokenTypes::Word {
                println!("?");
                return;
            }

            let query = lex.str_data.clone();
            let exp = format!(".*{}.*", query.as_str());

            let re = Regex::new(exp.as_str()).unwrap();
            let mut found = false;

            for i in state.line + 1..=state.lines.len() - 1 {
                if re.is_match(state.lines[i].as_str()) {
                    state.line = i;
                    found = true;
                    break;
                }
            }

            if !found {
                for i in 0..=state.line {
                    if re.is_match(state.lines[i].as_str()) {
                        state.line = i;
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                println!("?");
            }
        }
        // Direct command (eg: l)
        else if token == TokenTypes::Word {
            let c = lex.str_data.clone();
            match c.as_str() {
                "a" => {
                    state.flags |= WRITE_MODE;
                    if state.line < state.lines.len() {
                        state.line += 1;
                    }
                }
                "s" => {
                    println!("subs");
                    token = lex.next();

                    if token != TokenTypes::Slash {
                        println!("? First slash");
                        return;
                    }

                    token = lex.next();
                    if token != TokenTypes::Word {
                        println!("? Word");
                        return;
                    }

                    let rexp = lex.str_data.clone();

                    println!("rexp = {}", rexp);

                    token = lex.next();
                    if token != TokenTypes::Slash {
                        println!("? Second slash");
                        return;
                    }

                    token = lex.next();
                    if token != TokenTypes::Word {
                        println!("? Word2");
                        return;
                    }

                    let value = lex.str_data.clone();

                    println!("value = {}", value);

                    let exp = format!(".*{}.*", rexp);
                    let re = Regex::new(exp.as_str()).unwrap();
                    let mut found = false;

                    for i in 0..=state.lines.len() {
                        if re.is_match(state.lines[i].as_str()) {
                            println!("{}", state.lines[i]);
                            state.lines[i] = re.replace(exp.as_str(), value.as_str()).to_string();
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        println!("? Not found");
                    }
                }
                "w" => {
                    if let Some(fp) = &state.file_path {
                        let mut file = match File::create(fp.as_str()) {
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
                    } else {
                        println!("?");
                    }
                }
                "l" => {
                    println!("{}$", state.lines[state.line]);
                }
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
                    for line in &state.lines[0..second] {
                        println!("{}$", line);
                    }
                }
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
                // Jump to line number
                if first <= state.lines.len() {
                    state.line = first - 1;
                } else {
                    println!("?");
                }
                return;
            }

            token = lex.next();
            if token != TokenTypes::Number {
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
                    for line in &state.lines[first - 1..second] {
                        println!("{}$", line);
                    }
                }
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

    let mut state = State {
        file_path: None,
        lines: Vec::new(),
        line: 0,
        bytes: 0,
        flags: 0,
    };
    let mut cmd = String::new();

    if args.len() > 1 {
        state.file_path = Some(args[1].clone());
        let file = File::open(args[1].as_str())?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = line?;
            state.lines.push(l.as_str().to_string());
            state.bytes += l.len() + 1;
        }
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
