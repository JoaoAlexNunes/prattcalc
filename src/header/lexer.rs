#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Number,
    Subtraction,
    Addition,
    Multiplication,
    Div,
    EOF, // end of file
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes(); // 0 - 255
    let mut cursor = 0;

    while cursor < bytes.len() {
        let ch = bytes[cursor];

        // 123455
        //       ^
        match ch {
            b'0'..=b'9' => {
                let start = cursor;

                let mut ch = ch;
                while ch.is_ascii_digit() {
                    cursor += 1;
                    if cursor >= bytes.len() {
                        break;
                    }
                    ch = bytes[cursor];
                }

                let end = cursor;
                // start..end

                let value = source[start..end].to_string(); // &str -> String
                tokens.push(Token {
                    kind: TokenKind::Number,
                    value,
                });
            }
            b'+' => {
                tokens.push(Token {
                    kind: TokenKind::Addition,
                    value: "+".to_string(),
                });

                cursor += 1;
            }
            b'-' => {
                tokens.push(Token {
                    kind: TokenKind::Subtraction,
                    value: "-".to_string(),
                });

                cursor += 1;
            }
            b'*' => {
                tokens.push(Token {
                    kind: TokenKind::Multiplication,
                    value: "*".to_string(),
                });

                cursor += 1;
            }
            b'/' => {
                tokens.push(Token {
                    kind: TokenKind::Div,
                    value: "/".to_string(),
                });

                cursor += 1;
            }
            c if c.is_ascii_whitespace() => {
                cursor += 1;
            }
            _ => {
                return vec![Token {
                    kind: TokenKind::EOF,
                    value: "".to_string(),
                }];
            }
        }
    }
    tokens.push(Token {
        kind: TokenKind::EOF,
        value: "".to_string(),
    });
    tokens
}
