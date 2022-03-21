/*
Jay lexer
Copyright (C) 2022  Loris Cuntreri
*/

use {
    super::token::{Span, Token, TokenType},
    logos::{Logos, SpannedIter},
    std::{iter::Peekable, ops::Range},
};

pub struct Lexer<'inp> {
    pub logos_iter: Peekable<SpannedIter<'inp, TokenType>>,
    pub input: &'inp str,
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

impl<'inp> Lexer<'inp> {
    pub fn new(input: &'inp str) -> Self {
        Self {
            logos_iter: TokenType::lexer(&input).spanned().peekable(),
            input: &input,
        }
    }
}

impl<'inp> Iterator for Lexer<'inp> {
    type Item = Token<'inp>;

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
