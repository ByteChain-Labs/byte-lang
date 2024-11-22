#[derive(Debug, PartialEq, Eq, Hash, Clone)]

pub enum TokenType {
    // Single-character tokens.
    LPAREN, RPAREN, LBRACE, RBRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANGEQUAL,
    EQUAL, EQUALEQUAL,
    GREATER, GREATEREQUAL,
    LESS, LESSEQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, CONTRACT, ELSE, FALSE, FUNC, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, SELF, TRUE, LET, CONST, WHILE,

    EOF //End Of File
}


#[derive(Debug, Clone)]

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, literal: Option<String>) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            literal,
        }
    }
}
