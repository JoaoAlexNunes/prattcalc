use super::ast;
use super::lexer;

use ast::{Expr, OperatorKind};
use lexer::{Token, TokenKind};

fn prefix_bp(kind: &TokenKind) -> ((), u8) {
    match kind {
        TokenKind::Subtraction => ((), 5),
        _ => panic!("Unknown token {:?}", kind),
    }
}

// fn postfix_bp(kind: TokenKind) -> (u8, ()) {

// }

fn infix_bp(kind: &TokenKind) -> (/*LBP*/ u8, /*RBP*/ u8) {
    match kind {
        TokenKind::Addition | TokenKind::Subtraction => (1, 2),
        TokenKind::Multiplication | TokenKind::Div => (3, 4),
        _ => panic!("Unknown token {:?}", kind),
    }
}

// Expr := PrimaryExpr | BinaryExpr
// BinaryExpr := PrimaryExpr Operator Expr

fn parse_primary(tokens: &[Token], cursor: &mut usize) -> Expr {
    let current_token = &tokens[*cursor];
    *cursor += 1;

    match current_token.kind {
        TokenKind::Number => Expr::Number(current_token.value.parse().unwrap()),
        _ => panic!(
            "Primary Expression Expected but found {:?}",
            current_token.kind
        ),
    }
}

fn parse_operator(tokens: &[Token], cursor: &mut usize) -> OperatorKind {
    let current_token = &tokens[*cursor];
    *cursor += 1;

    match current_token.kind {
        TokenKind::Addition => OperatorKind::Add,
        TokenKind::Subtraction => OperatorKind::Sub,
        TokenKind::Multiplication => OperatorKind::Mul,
        TokenKind::Div => OperatorKind::Div,
        _ => panic!(
            "Operator Expression Expected but found {:?}",
            current_token.kind
        ),
    }
}

// recursive descent parser
pub fn parse(tokens: &[Token], cursor: &mut usize, min_bp: u8) -> Expr {
    let mut left = match tokens[*cursor].kind {
        TokenKind::Number => parse_primary(tokens, cursor),
        TokenKind::Subtraction => {
            let ((), r_bp) = prefix_bp(&TokenKind::Subtraction);
            let kind = parse_operator(tokens, cursor);
            let right = parse(&tokens, cursor, r_bp);

            Expr::Unary {
                kind,
                operand: Box::new(right),
            }
        }
        _ => panic!("Unexpected token {:?}", tokens[*cursor].kind),
    };

    loop {
        // iterate over all the tokens...
        let op = &tokens[*cursor];

        if op.kind == TokenKind::EOF {
            break;
        }

        // 1 + 2 * 3 - 4

        let (l_bp, r_bp) = infix_bp(&op.kind);
        if min_bp > l_bp {
            break;
        }

        let kind = parse_operator(tokens, cursor);

        let right = parse(&tokens, cursor, r_bp);

        left = Expr::Binary {
            kind,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    left
}
