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
            '(' => self.add_token(TokenType::LParen),
            ')' => self.add_token(TokenType::RParen),
            '{' => self.add_token(TokenType::LBrace),
            '}' => self.add_token(TokenType::RBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    self.single_line_comment();
                } else if self.match_char('*') {
                    self.multi_line_comment();
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            '\n' => self.line += 1, // Handle line breaks
            '"' => self.string(),
            'o' => {
                if self.match_char('r') {
                    self.add_token(TokenType::Or);
                }
            }
            _ => {
                if Scanner::is_digit(c) {
                    self.number();
                } else if Scanner::is_alpha(c) {
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

    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(c: char) -> bool {
        return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_';
    }

    fn is_alpha_numeric(c: char) -> bool {
        return Scanner::is_alpha(c) || Scanner::is_digit(c);
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
        self.add_token_with_value(TokenType::String, Some(value));
    }
    
    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }
    
        // Look for fractional part
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance(); // Consume '.'
    
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }
    
        let value = self.source[self.start..self.current].to_string();
        self.add_token_with_value(TokenType::Number, Some(value));
    }
    
    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) || self.peek() == '_' {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let token_type = self.keywords(&text);
        self.add_token(token_type);
    }

    fn keywords(&self, text: &str) -> TokenType {
        match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "contract" => TokenType::Contract,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "func" => TokenType:: Func,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "Self" => TokenType::SelfUpper,
            "self" => TokenType::SelfLower,
            "true" => TokenType::True,
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "while" => TokenType::While,
            "import" => TokenType::Import,
            "from" => TokenType::From,
            _ => TokenType::Identifier,
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
