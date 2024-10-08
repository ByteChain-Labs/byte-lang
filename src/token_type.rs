pub enum TokenType {
    // Keywords
    STRUCT, CONTRACT,
    PUBLIC, PRIVATE, PROTECTED,
    MUT, LET, CONST,
    FUNC, PRINT,
    IMPORT, RETURN,
    IF, ELSE, WHILE, FOR,
    TRUE, FALSE,

    // Literals
    INT_LITERAL(i64),
    UINT_LITERAL(u64),
    FLOAT_LITERAL(f64),
    STRING_LITERAL(String),
    BOOL_LITERAL(bool),

    // Identifiers
    IDENTIFIER(String),

    // Operators
    PLUS, MINUS,
    MULTIPLY, DIVIDE,
    EQUAL, DOUBLE_EQUAL,
    NOT_EQUAL, MODULUS,
    GREATER_THAN, LESS_THAN,
    GREATER_EQUAL, LESS_EQUAL,
    AND, OR, NOT,

    // Delimiters
    OPEN_PAREN, CLOSE_PAREN,
    OPEN_BRACE, CLOSE_BRACE,
    OPEN_BRACKET, CLOSE_BRACKET,
    SEMICOLON, COLON, COMMA,

    // Types
    INT(i64),
    UINT(u64),
    FLOAT(f64),
    STRING(String),
    BOOL(bool),

    // Comments
    LINE_COMMENT(String),
    BLOCK_COMMENT(String),

    // Whitespace
    WHITESPACE,
    NEWLINE,
}
