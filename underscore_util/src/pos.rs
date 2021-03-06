//! Source Code locations

use std::cmp;
use std::fmt::{self, Display};
use std::str::Chars;

/// Represents a Span in the source file along with its value
#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub span: Span,
    pub value: T,
}

/// A span between two locations in a source file
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

pub const EMPTYSPAN: Span = Span {
    start: Position {
        line: 1,
        column: 0,
        absolute: 1,
    },
    end: Position {
        line: 1,
        column: 1,
        absolute: 1,
    },
};

/// Represents a postion within a specifc source file
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Position {
    /// A 0 offset line the source code
    pub line: i32,
    /// A 0 offset col
    pub column: i32,
    pub absolute: usize,
}

/// An iterator over all the charaters and there positions within a file
#[derive(Debug, Clone)]
pub struct CharPosition<'a> {
    pub pos: Position,
    pub chars: Chars<'a>,
}

impl<'a> CharPosition<'a> {
    pub fn new(input: &'a str) -> Self {
        CharPosition {
            pos: Position {
                line: 1,
                column: 1,
                absolute: 0,
            },
            chars: input.chars(),
        }
    }
}

impl<'a> Iterator for CharPosition<'a> {
    type Item = (Position, char);

    fn next(&mut self) -> Option<(Position, char)> {
        self.chars.next().map(|ch| {
            let pos = self.pos;
            self.pos = self.pos.shift(ch);
            (pos, ch)
        })
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {},column {}", self.line, self.column)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl Span {
    pub fn to(self, other: Span) -> Self {
        Span {
            start: cmp::min(self.start, other.end),
            end: cmp::max(self.end, other.end),
        }
    }
}

impl<T> Spanned<T> {
    pub fn get_span(&self) -> Span {
        self.span
    }
}
impl Position {
    pub fn shift(mut self, ch: char) -> Self {
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else if ch == '\t' {
            self.column += 4;
        } else {
            self.column += 1;
        }

        self.absolute += ch.len_utf8();
        self
    }
}
