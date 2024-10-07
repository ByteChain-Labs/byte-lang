#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Box<dyn std::any::Any>>,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<Box<dyn std::any::Any>>, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal.as_ref().map(|l| l.type_id()))
    }
}
