#[derive(Debug, PartialEq)]
pub enum TokenTypes {
    End,
    Word,
    Number,
    Comma,
    Slash
}

pub struct Lexer {
    code: String,
    position: usize,
    pub str_data: String,
    pub num_data: usize
}

impl Lexer {
    pub fn new(c: &str) -> Lexer {
        Lexer {
            code: c.to_string(),
            position: 0,
            str_data: String::new(),
            num_data: 0
        }
    }

    pub fn next(&mut self) -> TokenTypes {
        self.str_data.clear();
        self.num_data = 0;

        if self.position >= self.code.len() {
            return TokenTypes::End;
        }

        if let Some(c) = self.code.chars().nth(self.position) {
            if c == ',' {
                self.position += 1;
                return TokenTypes::Comma;
            }
            else if c == '/' {
                self.position += 1;
                return TokenTypes::Slash;
            }
            else if c.is_ascii_digit() {
                self.position += 1;
                self.num_data = c as usize - '0' as usize;

                loop {
                    let Some(n) = self.code.chars().nth(self.position) else {
                        break;
                    };
                    if n.is_ascii_digit() {
                        self.num_data *= 10;
                        self.num_data += n as usize - '0' as usize;
                        self.position += 1;
                    } else {
                        break;
                    }
                }
                return TokenTypes::Number;
            } else {
                self.position += 1;
                self.str_data.push(c);

                loop {
                    let Some(n) = self.code.chars().nth(self.position) else {
                        break;
                    };
                    if !n.is_ascii_digit() {
                        self.str_data.push(n);
                        self.position += 1;
                    } else {
                        break;
                    }
                }

                return TokenTypes::Word;
            }
        }

        return TokenTypes::End;
    }
}
