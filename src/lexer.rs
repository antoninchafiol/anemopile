use std::{char, error::Error, path::Path, str::Chars};
use std::path::PathBuf;
use std::string::String;


#[derive(Debug)]
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

impl Operator {
    fn from(s: &str) -> Option<Self> {
         match s {
             "+" => Some(Self::Add),
             "-" => Some(Self::Sub),
             "=" => Some(Self::Equal),
             _ => None,
         }
    }
}

#[derive(PartialEq, Eq, Debug)]
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

#[derive(Debug)]
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

pub struct Lexer {
    pub source: PathBuf,
}

impl Lexer {
    pub fn new(path: &str) -> Self { 
        Self { 
            source: PathBuf::from(&path),     
        }
    }
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
                    output.push(
                        Token::Line(
                            buffer
                            .parse::<u16>()
                            .unwrap()
                        )
                    );
                    buffer.clear();
                    return Some(true)
                }, 
                _ => {
                    return Some(false);
                }
            }

        }
        return Some(true);
    }

    fn check_to_token(s: &str, is_string: bool) -> Result<Token, Box<dyn Error>> {

        
        match s.to_owned() {
            x if x.chars().all(|c| c.is_alphabetic()) => {
                let commands = vec![
                    Command::Let,
                    Command::Print,
                    Command::End,
                ];
                let s_command:Command = Command::from_string(s.to_string());
                if commands.contains(&s_command) {
                    Ok(Token::Commands(s_command))
                } else {
                    Ok(Token::Variable(s.to_string()))
                }
            },
            x if x.chars().all(|c| c.is_numeric()) => {
                Ok(Token::Int(s.parse::<i32>().unwrap()))
            },
            x if Operator::from(s).is_some() => {
                Ok(Token::Operator(Operator::from(s).unwrap()))
            }
            _ => {
                Ok(Token::String(s.to_string()))
                // return Err("Buffer cannot be checked".into());
            }
        }
    }

    fn tokenize_line(line: String) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut output: Vec<Token> = Vec::new();
        let mut buffer: String = String::new();

        let mut chars: Chars = line.chars();

        // Handling Line number
        if Self::line_number(&mut chars,  &mut output) == Some(false){
            return Err("Error when getting line number".into());
        }

        // Handling rest of line
        // let length = chars.clone().count();
        let mut is_in_string: bool = false;
        while let Some(c) = chars.next() {
            match c {
                x if x.is_ascii_whitespace() => {
                    if is_in_string {
                        // Add
                        buffer.push(c);
                    } else {
                        // Raise Token
                        if buffer != "" {
                            output.push(Self::check_to_token(&buffer,is_in_string)?);
                        }
                        buffer.clear();
                    }
                },
                x if x == ';' => {
                    // Raise Token
                    // output.push(Self::check_to_token(&buffer, is_in_string)?);
                    output.push(Token::Operator(Operator::SemiColumn));
                    buffer.clear();
                },
                x if x == '=' => {
                    // Raise Token
                    output.push(Token::Operator(Operator::Equal));
                    buffer.clear();
                },
                x if x == '"' => {
                    // Changing to String
                    is_in_string = !is_in_string;
                    if !is_in_string {
                        // Raise Token
                        output.push(Self::check_to_token(&buffer, is_in_string)?);
                        buffer.clear();
                    }
                }
                _ => {
                    buffer.push(c);
                    // return Err(format!("Issue when parsing at line {} and character {}", line_index, c).into());
                }
            }
            if chars.clone().next() == None {
                output.push(Self::check_to_token(&buffer, is_in_string)?);
            }
        } 
        return Ok(output);
    }
    pub fn tokenize(buf: Vec<String>) -> Result<Vec<Vec<Token>>, Box<dyn Error>> {
        let mut output: Vec<Vec<Token>> = Vec::new();

        for (line_index, line) in buf.iter().enumerate() {
            output.push(Self::tokenize_line(line.clone().to_string())?);
        }
        return Ok(output);
    }
}
pub fn test() {
    println!("test");
}





