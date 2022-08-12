pub(crate) mod token;

use logos::{Logos, SpannedIter};
use std::{iter::Peekable, ops::Range};

use self::token::{Span, Token, TokenType};

pub struct Lexer<'a> {
    pub logos_iter: Peekable<SpannedIter<'a, TokenType>>,
    pub input: &'a str,
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.start..span.end
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            logos_iter: TokenType::lexer(&input).spanned().peekable(),
            input: &input,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.logos_iter.next() {
            Some((token_type, span)) => Some(Token {
                token_type,
                slice: &self.input[span.start..span.end],
                span: span.into(),
            }),
            _ => None,
        }
    }
}
