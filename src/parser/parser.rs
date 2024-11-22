use crate::lexer::tokens::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn error(&self, message: &str) -> String {
        format!("Error at {}: {}", self.peek().line, message)
    }
}

impl Parser {
    fn parse(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            self.declaration()?; // Start parsing declarations
        }
        Ok(())
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::EQUALEQUAL, TokenType::BANGEQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_token(&[
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::STAR, TokenType::SLASH]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::Literal {
                value: self.previous().lexeme.clone(),
            });
        }
        if self.match_token(&[TokenType::TRUE]) {
            return Ok(Expr::Literal {
                value: "true".to_string(),
            });
        }
        if self.match_token(&[TokenType::FALSE]) {
            return Ok(Expr::Literal {
                value: "false".to_string(),
            });
        }
        if self.match_token(&[TokenType::NIL]) {
            return Ok(Expr::Literal {
                value: "nil".to_string(),
            });
        }
        if self.match_token(&[TokenType::LPAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RPAREN, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(self.error("Expect expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(self.error(message))
    }
}
