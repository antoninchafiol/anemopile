use std::{char, error::Error, str::Chars};


pub enum Operator{ 
    Equal,
    NEqual,
    Less,
    LEqual,
    Greater,
    GEqual,
    Add,
    Sub,
    Mul,
    Div,
    SemiColumn
}

#[derive(PartialEq, Eq)]
pub enum Command {
    Let, 
    Print,
    End,
    Error
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Let => String::from("LET"),
            Command::Print => String::from("PRINT"),
            Command::End => String::from("END"),
            Command::Error => String::from("Error"),
        }
    }
}

impl Command { 
    fn from_string(s: String) -> self::Command {
        match s.to_ascii_lowercase() {
            x if x == self::Command::Let.to_string().to_ascii_lowercase()   => self::Command::Let,
            x if x == self::Command::Print.to_string().to_ascii_lowercase() => self::Command::Print,
            x if x == self::Command::End.to_string().to_ascii_lowercase()   => self::Command::End, 
            _ => self::Command::Error
        }
    }
} 

pub enum Token {
    // Line 
    Line(u16),

    // Commands
    Commands(self::Command),
    
    Variable(String),
    Operator(self::Operator),

    // Values
    Int(i32),
    Float(f32),
    String(String)

}

pub struct Lexer<'a> {
    pub source: &'a str,
}

impl<'a> Lexer<'a> {
    fn line_number(current_line_chars: &mut Chars,  output: &mut Vec<Token>) -> Option<bool> {
        let mut buffer: String =  String::new();
        let length = current_line_chars.clone().count();
        for char_index in 0..length { 
            let c = current_line_chars.next().unwrap();

            // Code Line
            match c {
                '0'..='9' => {
                    buffer.push(c);
                },
                ' ' => {
                    buffer.parse::<u16>();
                    output.push(
                        Token::Line(
                            buffer
                            .parse::<u16>()
                            .unwrap()
                        )
                    );
                    buffer.clear();
                }, 
                _ => {
                    return Some(false);
                }
            }

        }
        return Some(true);
    }

    fn check_to_token(s: &String) -> Result<Token, Box<dyn Error>> {
    
        match s.to_owned() {
            x if x.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) => {
                Ok(Token::String(*s.clone().to_string()))
            },
            x if x.chars().all(|c| c.is_alphabetic()) => {
                let commands = vec![
                    Command::Let,
                    Command::Print,
                    Command::End,
                ];
                let s_command:Command = Command::from_string(*s.clone().to_string());
                if commands.contains(&s_command) {
                    Ok(Token::Commands(s_command))
                } else {
                    Ok(Token::Variable(*s))
                }
            },
            x if x.chars().all(|c| c.is_numeric()) => {
                Ok(Token::Int(s.parse::<i32>().unwrap()))
            },
            _ => {
                return Err("Buffer cannot be checked".into());
            }
        }
    }

    pub fn tokenize(buf: Vec<String>) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut output: Vec<Token> = Vec::new();
        let mut buffer: String = String::new();

        for (line_index, line) in buf.iter().enumerate() {

            let mut chars: Chars = line.chars();

            // Handling Line number
            if Self::line_number(&mut chars.clone(), &mut output) == Some(false){
                return Err("Error when getting line number".into());
            }
            
            // Handling rest of line
            let length = chars.clone().count();
            while let Some(c) = chars.next() {
                let is_in_string: bool = false;
                match c {
                    x if x.is_ascii_digit() => {
                        // Add
                        buffer.push(c);
                    },
                    x if x.to_ascii_lowercase().is_ascii_alphabetic() => {
                        // Add
                        buffer.push(c);
                    }, 
                    // Breakers
                    x if x.is_ascii_whitespace() => {
                        if is_in_string {
                            // Add
                            buffer.push(c);
                        } else {
                            // Raise Token
                            output.push(Self::check_to_token(&buffer));
                        }
                    },
                    x if x == ';' => {
                        // Raise Token
                        output.push(Self::check_to_token(&buffer));
                        output.push(Token::Operator(Operator::SemiColumn));
                    },
                    x if x == '"' => {
                        // Changing to String
                        is_in_string = !is_in_string;
                        if !is_in_string {
                            // Raise Token
                            output.push(Self::check_to_token(&buffer));
                        }
                    }
                    _ => {
                        return Err("Issue when parsing".into());
                    }
                }
            } 
        }
        return Ok(output);
    }
}
pub fn test() {
    println!("test");
}





