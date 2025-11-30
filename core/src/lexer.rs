use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace();
            if self.position >= self.input.len() {
                break;
            }

            let ch = self.current_char();

            let token = match ch {
                '{' => {
                    self.advance();
                    Token::LeftBrace
                }
                '}' => {
                    self.advance();
                    Token::RightBrace
                }
                '[' => {
                    self.advance();
                    Token::LeftBracket
                }
                ']' => {
                    self.advance();
                    Token::RightBracket
                }
                ':' => {
                    self.advance();
                    Token::Colon
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                '"' => self.read_string(),
                _ if ch.is_ascii_alphabetic() || ch == '_' => self.read_identifier(),
                _ if ch.is_ascii_digit() => self.read_number(),
                _ => {
                    self.advance();
                    continue;
                }
            };

            tokens.push(token);
        }

        tokens.push(Token::Eof);
        tokens
    }

    fn current_char(&self) -> char {
        self.input[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // skip "
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != '"' {
            self.advance();
        }
        let value: String = self.input[start..self.position].iter().collect();
        self.advance(); // skip closing "
        Token::String(value)
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.current_char().is_alphanumeric() {
            self.advance();
        }
        let value: String = self.input[start..self.position].iter().collect();
        self.match_keyword(&value)
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.current_char().is_ascii_digit() {
            self.advance();
        }
        let value: String = self.input[start..self.position].iter().collect();
        Token::Number(value.parse().unwrap())
    }

    fn match_keyword(&self, value: &str) -> Token {
        match value {
            "app" => Token::App,
            "page" => Token::Page,
            "component" => Token::Component,
            "import" => Token::Import,
            "start" => Token::Start,
            "layout" => Token::Layout,
            "Column" => Token::Column,
            "Row" => Token::Row,
            "Stack" => Token::Stack,
            "Container" => Token::Container,
            "Text" => Token::Text,
            "Image" => Token::Image,
            "Button" => Token::Button,
            "fetch" => Token::Fetch,
            "state" => Token::State,
            "style" => Token::Style,
            "font" => Token::Font,
            "color" => Token::Color,
            "asset" => Token::Asset,
            "export" => Token::Export,
            "function" => Token::Function,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            _ => Token::Identifier(value.to_string()),
        }
    }
}