use crate::lexer::tokens::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<String>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    
        // Add EOF token with None as the literal
        self.tokens.push(Token::new(TokenType::EOF, String::new(), self.line, None));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LPAREN),
            ')' => self.add_token(TokenType::RPAREN),
            '{' => self.add_token(TokenType::LBRACE),
            '}' => self.add_token(TokenType::RBRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANGEQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUALEQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESSEQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATEREQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '/' => {
                if self.match_char('/') {
                    self.single_line_comment();
                } else if self.match_char('*') {
                    self.multi_line_comment();
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            '\n' => self.line += 1, // Handle line breaks
            '"' => self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    self.error(&format!("Unexpected character: {}", c));
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn add_token_with_value(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, self.line, literal));
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_value(token_type, None);
    }
    
    fn error(&mut self, message: &str) {
        self.errors.push(format!("Line {}: {}", self.line, message));
    }
    
    fn single_line_comment(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn multi_line_comment(&mut self) {
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                break;
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("Unterminated multi-line comment.");
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
    
        if self.is_at_end() {
            eprintln!("Unterminated string.");
            return;
        }
    
        self.advance(); // Consume closing "
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_value(TokenType::STRING, Some(value));
    }
    
    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
    
        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance(); // Consume '.'
    
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
    
        let value = self.source[self.start..self.current].to_string();
        self.add_token_with_value(TokenType::NUMBER, Some(value));
    }
    
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let token_type = self.keyword_or_identifier(&text);
        self.add_token(token_type);
    }

    fn keyword_or_identifier(&self, text: &str) -> TokenType {
        match text {
            "and" => TokenType::AND,
            "class" => TokenType::CLASS,
            "contract" => TokenType::CONTRACT,
            "else" => TokenType::ELSE,
            "false" => TokenType::FALSE,
            "func" => TokenType::FUNC,
            "for" => TokenType::FOR,
            "if" => TokenType::IF,
            "nil" => TokenType::NIL,
            "or" => TokenType::OR,
            "print" => TokenType::PRINT,
            "return" => TokenType::RETURN,
            "super" => TokenType::SUPER,
            "self" => TokenType::SELF,
            "true" => TokenType::TRUE,
            "let" => TokenType::LET,
            "const" => TokenType::CONST,
            "while" => TokenType::WHILE,
            _ => TokenType::IDENTIFIER,
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }
}
