use std::iter::Peekable;
use std::str::Chars;

pub enum TokenKind {
    Number{ value: f64 },
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}
pub struct Span {
    start: usize,
    end: usize,
    literal: String,
}

impl Span {
    pub fn new(start: usize, end: usize, literal: String) -> Span {
        Self { start, end, literal }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    // input: Peekable<Chars<'a>>
    input: &'a str,
    current_pos: usize,
}

impl <'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Self { input, current_pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let c = self.input.peek()?;
        let span = self.span();
        let token = match c {
            '0'..='9' => self.lex_number(),
            '+' => {
                self.input.next();
                Token::new(TokenKind::Plus, span)
            },
            '-' => {
                self.input.next();
                Token::new(TokenKind::Minus, span)
            },
            '*' => {
                self.input.next();
                Token::new(TokenKind::Star, span)
            },
            '/' => {
                self.input.next();
                Token::new(TokenKind::Slash, span)
            },
            '(' => {
                self.input.next();
                Token::new(TokenKind::LParen, span)
            },
            ')' => {
                self.input.next();
                Token::new(TokenKind::RParen, span)
            },
            _ => {
                self.input.next();
                Token::new(TokenKind::Plus, span)
            }
        };
        Some(token)
    }

    fn is_number_start(c: &char) -> bool {
        c.peek().map_or(false, |c| c.is_digit(10))
    }

    fn peek (&mut self) -> Option<char> {
        self.input.peek().cloned()
    }
}