use crate::token_type::TokenType;

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string_impl(&self) -> String {
        format!(
            "type: {}, lexeme: {}, literal: {}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
