use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq, Logos, ToPrimitive)]
pub(crate) enum SyntaxKind {
    BinaryExpr,

    #[regex("#.*")]
    Comment,

    #[token("=")]
    Equals,

    #[error]
    Error,

    #[token("fn")]
    FnKw,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    #[token("{")]
    LBrace,

    #[token("(")]
    LParen,

    #[token("let")]
    LetKw,

    #[token("-")]
    Minus,

    #[regex("[0-9]+")]
    Number,

    #[token("+")]
    Plus,

    PrefixExpr,

    #[token("}")]
    RBrace,

    #[token(")")]
    RParen,

    Root,

    #[token("/")]
    Slash,

    #[token("*")]
    Star,

    #[regex("[ \n]+")]
    Whitespace,
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some(Self::Item { kind, text })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Lexeme<'a> {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some(Lexeme { kind, text: input }));
    }

    #[test]
    fn lex_spaces() {
        check("   ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_fn_keyword() {
        check("fn", SyntaxKind::FnKw);
    }

    #[test]
    fn lex_let_keyword() {
        check("let", SyntaxKind::LetKw);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", SyntaxKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", SyntaxKind::Ident);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", SyntaxKind::Ident);
    }

    #[test]
    fn lex_number() {
        check("123456", SyntaxKind::Number);
    }

    #[test]
    fn lex_plus() {
        check("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", SyntaxKind::Star);
    }

    #[test]
    fn lex_slash() {
        check("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", SyntaxKind::Equals);
    }

    #[test]
    fn lex_left_brace() {
        check("{", SyntaxKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", SyntaxKind::RBrace);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("x", SyntaxKind::Ident);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", SyntaxKind::RParen);
    }

    #[test]
    fn lex_comment() {
        check("# foo", SyntaxKind::Comment);
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check("  \n ", SyntaxKind::Whitespace);
    }
}

impl SyntaxKind {
    pub(crate) fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}
