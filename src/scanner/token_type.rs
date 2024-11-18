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

    EOF
}
