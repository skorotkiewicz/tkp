use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords (Toki Pona)
    Pali,     // function (pali = make/do)
    Pana,     // return (pana = give)
    Ijo,      // variable (ijo = thing)
    Awen,     // constant (awen = stay/keep)
    La,       // if (la = conditional)
    Ante,     // else (ante = different)
    Sin,      // for loop (sin = again/repeat)
    Lon,      // while (lon = exist/during)
    Pini,     // break (pini = end/stop)
    Tawa,     // continue (tawa = go/move)
    Kin,      // true (kin = indeed)
    Ala,      // false (ala = no/not)
    Weka,     // null/void (weka = absent)
    Toki,     // print (toki = speak)
    Kute,     // input (kute = listen)
    Kulupu,   // struct (kulupu = group)
    Lukin,    // try (lukin = try/look)
    Alasa,    // catch (alasa = hunt/catch)
    Jo,       // import (jo = have/contain)
    Sama,     // match (sama = same)
    Nanpa,    // enum (nanpa = number/enumeration)
    Insa,     // in (insa = inside)
    DotDot,   // ..
    FatArrow, // =>
    Ken,      // impl (ken = can/ability)
    // Type keywords
    NanpaTy,   // integer type
    KipisiTy,  // float type
    SitelenTy, // string type
    LawaTy,    // bool type
    #[allow(dead_code)]
    WekaTy, // void type
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    // Identifiers
    Identifier(String),
    // Arithmetic operators
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    // Comparison operators
    EqEq,   // ==
    BangEq, // !=
    Lt,     // <
    Gt,     // >
    LtEq,   // <=
    GtEq,   // >=
    // Logical operators
    AmpAmp,   // &&
    PipePipe, // ||
    Bang,     // !
    // Assignment operators
    Eq,      // =
    PlusEq,  // +=
    MinusEq, // -=
    StarEq,  // *=
    SlashEq, // /=
    // Arrow
    Arrow, // ->
    // Delimiters
    Colon,      // :
    ColonColon, // ::
    Comma,      // ,
    Semicolon,  // ;
    // Brackets
    LBrace,   // {
    RBrace,   // }
    LParen,   // (
    RParen,   // )
    LBracket, // [
    RBracket, // ]
    Dot,      // .
    // Special
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct TokenWithPos {
    pub token: Token,
    pub line: usize,
    pub col: usize,
}

impl TokenWithPos {
    pub fn new(token: Token, line: usize, col: usize) -> Self {
        Self { token, line, col }
    }
}

pub fn get_keyword_map() -> HashMap<String, Token> {
    let mut map = HashMap::new();
    // Keywords
    map.insert("pali".to_string(), Token::Pali);
    map.insert("pana".to_string(), Token::Pana);
    map.insert("ijo".to_string(), Token::Ijo);
    map.insert("awen".to_string(), Token::Awen);
    map.insert("la".to_string(), Token::La);
    map.insert("ante".to_string(), Token::Ante);
    map.insert("sin".to_string(), Token::Sin);
    map.insert("lon".to_string(), Token::Lon);
    map.insert("pini".to_string(), Token::Pini);
    map.insert("tawa".to_string(), Token::Tawa);
    map.insert("kin".to_string(), Token::Kin);
    map.insert("ala".to_string(), Token::Ala);
    map.insert("weka".to_string(), Token::Weka);
    map.insert("toki".to_string(), Token::Toki);
    map.insert("kute".to_string(), Token::Kute);
    map.insert("kulupu".to_string(), Token::Kulupu);
    map.insert("lukin".to_string(), Token::Lukin);
    map.insert("alasa".to_string(), Token::Alasa);
    map.insert("jo".to_string(), Token::Jo);
    map.insert("sama".to_string(), Token::Sama);
    map.insert("ken".to_string(), Token::Ken);
    map.insert("nanpa".to_string(), Token::Nanpa);
    map.insert("insa".to_string(), Token::Insa);
    // Type keywords
    map.insert("nanpa_kind".to_string(), Token::NanpaTy);
    map.insert("kipisi".to_string(), Token::KipisiTy);
    map.insert("sitelen".to_string(), Token::SitelenTy);
    map.insert("lawa".to_string(), Token::LawaTy);
    map
}

fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 0,
            keywords: get_keyword_map(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' || c == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn read_string(&mut self) -> Result<String, String> {
        let mut s = String::new();
        loop {
            match self.advance() {
                None => return Err("Unterminated string literal".to_string()),
                Some('"') => break,
                Some('\\') => match self.advance() {
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('\\') => s.push('\\'),
                    Some('"') => s.push('"'),
                    Some(c) => {
                        s.push('\\');
                        s.push(c);
                    }
                    None => return Err("Unterminated escape".to_string()),
                },
                Some(c) => s.push(c),
            }
        }
        Ok(s)
    }

    fn read_number(&mut self, first: char) -> Token {
        let mut num = String::new();
        num.push(first);
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num.push(c);
                self.advance();
            } else if c == '.' && !is_float && self.peek_next().is_some_and(|n| n.is_ascii_digit())
            {
                is_float = true;
                num.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::FloatLiteral(num.parse().unwrap_or(0.0))
        } else {
            Token::IntLiteral(num.parse().unwrap_or(0))
        }
    }

    fn read_identifier(&mut self, first: char) -> Token {
        let mut ident = String::new();
        ident.push(first);
        while let Some(c) = self.peek() {
            if is_identifier_continue(c) {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.keywords
            .get(&ident)
            .cloned()
            .unwrap_or(Token::Identifier(ident))
    }

    fn next_token(&mut self) -> Option<TokenWithPos> {
        loop {
            self.skip_whitespace();

            if self.peek() == Some('/') && self.peek_next() == Some('/') {
                self.advance();
                self.advance();
                self.skip_line_comment();
                continue;
            }

            break;
        }

        let line = self.line;
        let col = self.col;

        let c = self.advance()?;

        let token = match c {
            '\n' => Token::Newline,

            '"' => match self.read_string() {
                Ok(s) => Token::StringLiteral(s),
                Err(_) => return None,
            },

            '0'..='9' => self.read_number(c),

            '+' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::PlusEq
                } else {
                    Token::Plus
                }
            }
            '-' => {
                if self.peek() == Some('>') {
                    self.advance();
                    Token::Arrow
                } else if self.peek() == Some('=') {
                    self.advance();
                    Token::MinusEq
                } else {
                    Token::Minus
                }
            }
            '*' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::StarEq
                } else {
                    Token::Star
                }
            }
            '/' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::SlashEq
                } else {
                    Token::Slash
                }
            }
            '%' => Token::Percent,
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::EqEq
                } else if self.peek() == Some('>') {
                    self.advance();
                    Token::FatArrow
                } else {
                    Token::Eq
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::BangEq
                } else {
                    Token::Bang
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
            '&' => {
                if self.peek() == Some('&') {
                    self.advance();
                    Token::AmpAmp
                } else {
                    return None;
                }
            }
            '|' => {
                if self.peek() == Some('|') {
                    self.advance();
                    Token::PipePipe
                } else {
                    return None;
                }
            }
            ':' => {
                if self.peek() == Some(':') {
                    self.advance();
                    Token::ColonColon
                } else {
                    Token::Colon
                }
            }
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '.' => {
                if self.peek() == Some('.') {
                    self.advance();
                    Token::DotDot
                } else {
                    Token::Dot
                }
            }

            c if is_identifier_start(c) => self.read_identifier(c),

            _ => return None,
        };

        Some(TokenWithPos::new(token, line, col))
    }
}

pub fn tokenize(source: &str) -> Vec<TokenWithPos> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    while let Some(tok) = lexer.next_token() {
        tokens.push(tok);
    }
    tokens.push(TokenWithPos::new(Token::Eof, lexer.line, lexer.col));
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(src: &str) -> Vec<Token> {
        tokenize(src)
            .into_iter()
            .filter(|t| !matches!(t.token, Token::Newline | Token::Eof))
            .map(|t| t.token)
            .collect()
    }

    #[test]
    fn test_keyword_map() {
        let map = get_keyword_map();
        assert_eq!(map.get("pali"), Some(&Token::Pali));
        assert_eq!(map.get("pana"), Some(&Token::Pana));
        assert_eq!(map.get("la"), Some(&Token::La));
        assert_eq!(map.get("nanpa_kind"), Some(&Token::NanpaTy));
    }

    #[test]
    fn test_token_with_pos() {
        let t = TokenWithPos::new(Token::Pali, 1, 0);
        assert_eq!(t.line, 1);
        assert_eq!(t.col, 0);
        assert!(matches!(t.token, Token::Pali));
    }

    #[test]
    fn test_keyword() {
        let toks = tokens("pali");
        assert_eq!(toks, vec![Token::Pali]);
    }

    #[test]
    fn test_identifier() {
        let toks = tokens("my_var");
        assert_eq!(toks, vec![Token::Identifier("my_var".to_string())]);
    }

    #[test]
    fn test_integer_literal() {
        let toks = tokens("42");
        assert_eq!(toks, vec![Token::IntLiteral(42)]);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_float_literal() {
        let toks = tokens("3.14");
        assert_eq!(toks, vec![Token::FloatLiteral(3.14)]);
    }

    #[test]
    fn test_string_literal() {
        let toks = tokens("\"toki\"");
        assert_eq!(toks, vec![Token::StringLiteral("toki".to_string())]);
    }

    #[test]
    fn test_operators() {
        let toks = tokens("+ == -> += !=");
        assert_eq!(
            toks,
            vec![
                Token::Plus,
                Token::EqEq,
                Token::Arrow,
                Token::PlusEq,
                Token::BangEq,
            ]
        );
    }

    #[test]
    fn test_full_function() {
        let src = "pali add(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind { pana a + b }";
        let toks = tokens(src);
        assert_eq!(
            toks,
            vec![
                Token::Pali,
                Token::Identifier("add".to_string()),
                Token::LParen,
                Token::Identifier("a".to_string()),
                Token::Colon,
                Token::NanpaTy,
                Token::Comma,
                Token::Identifier("b".to_string()),
                Token::Colon,
                Token::NanpaTy,
                Token::RParen,
                Token::Arrow,
                Token::NanpaTy,
                Token::LBrace,
                Token::Pana,
                Token::Identifier("a".to_string()),
                Token::Plus,
                Token::Identifier("b".to_string()),
                Token::RBrace,
            ]
        );
    }

    #[test]
    fn test_line_comment_skipped() {
        let toks = tokens("42 // ni li toki\n99");
        assert_eq!(toks, vec![Token::IntLiteral(42), Token::IntLiteral(99)]);
    }

    #[test]
    fn test_position_tracking() {
        let result = tokenize("pali");
        assert_eq!(result[0].line, 1);
        assert_eq!(result[0].col, 0);
    }
}
