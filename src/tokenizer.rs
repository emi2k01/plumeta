#[derive(Debug)]
pub(crate) enum TokenKind {
    Ident,
    LeftBrace,
    RightBrace,
}

#[derive(Debug)]
pub(crate) struct Token<'input> {
    span: &'input str,
    offset: usize,
    kind: TokenKind,
}

pub(crate) struct Tokenizer<'input> {
    input: &'input str,
    offset: usize,
    tokens: Vec<Token<'input>>,
}

impl<'input> Tokenizer<'input> {
    pub(crate) fn new(input: &'input str) -> Self {
        Self {
            input,
            offset: 0,
            tokens: vec![],
        }
    }

    pub(crate) fn tokenize(mut self) -> Vec<Token<'input>> {
        loop {
            let ch = self.input.chars().next();
            let Some(ch) = ch else { break };

            let (token_kind, len) = match ch {
                '{' => (TokenKind::LeftBrace, 1),
                '}' => (TokenKind::RightBrace, 1),
                'a'..='z' | 'A'..='Z' => (TokenKind::Ident, ident_len(self.input)),
                _ => todo!(),
            };

            self.tokens.push(Token {
                span: &self.input[..len],
                offset: self.offset,
                kind: token_kind,
            });
            self.offset += len;
            self.input = &self.input[len..];
        }

        self.tokens
    }
}

fn ident_len(input: &str) -> usize {
    let ident_len = input
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric())
        .fold(0, |old, cur| old + cur.len_utf8());

    assert!(ident_len > 0, "identifier length can't be zero");

    ident_len
}

fn tokenize(input: &str) -> Vec<Token<'_>> {
    let tokenizer = Tokenizer::new(&input);
    tokenizer.tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_delimiters() {
        let input = "{}";
        k9::snapshot!(
            tokenize(input),
            r#"
[
    Token {
        span: "{",
        offset: 0,
        kind: LeftBrace,
    },
    Token {
        span: "}",
        offset: 1,
        kind: RightBrace,
    },
]
"#
        );
    }

    #[test]
    fn test_tokenize_ident() {
        let input = "Dropdown";
        k9::snapshot!(
            tokenize(input),
            r#"
[
    Token {
        span: "Dropdown",
        offset: 0,
        kind: Ident,
    },
]
"#
        );
    }

    #[test]
    fn test_tokenize_ident_block() {
        let input = "Dropdown { }";
        k9::snapshot!(
            tokenize(input),
            r#"
[
    Token {
        span: "Dropdown",
        offset: 0,
        kind: Ident,
    },
]
"#
        );
    }
}
