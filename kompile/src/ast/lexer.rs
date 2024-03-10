use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum TokenKind {
    Number{ value: i64 },
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Bad,
    EOF,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
        if self.current_pos > self.input.len() {
            return None;
        }
        let eof_char = '\0';
        if self.current_pos == self.input.len() {
            self.current_pos += 1;
            return Some(
                Token::new(
                    TokenKind::EOF,
                    Span::new(0, 0, eof_char.to_string())
                )
            );
        }

        let start = self.current_pos;
        let c = self.current_char();
        let mut kind = TokenKind::Bad;
        if Self::is_number_start(&c) {
            let number = self.consume_number();
            kind = TokenKind::Number{ value: number };
        }
        let end = self.current_pos;
        let literal = self.input[start..end].to_string();
        let span = Span::new(start, end, literal);
        Some(Token::new(kind, span))
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    // fn peek (&mut self) -> Option<char> {
    //     self.input.peek()
    // }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.current_pos).unwrap()
    }

    fn consume(&mut self) -> Option<char>{
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;
        Some(c)
    }

    fn consume_number(&mut self) -> i64 {
        let mut number = 0;
        while let Some(c) = self.consume() {
            if c.is_digit(10) {
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }
}