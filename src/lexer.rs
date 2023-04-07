#[derive(Debug)]
pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }
}

impl<'a> Lexer<'a> {
    fn take(&mut self, n: usize) -> &'a [char] {
        let result = &self.content[..n];
        self.content = &self.content[n..];
        result
    }

    fn take_while<P>(&mut self, predicate: P) -> &'a [char]
    where
        P: Fn(&char) -> bool,
    {
        let mut i = 0;
        while i < self.content.len() && predicate(&self.content[i]) {
            i += 1;
        }
        self.take(i)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }

        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_alphanumeric() {
            return Some(self.take_while(|c| c.is_alphanumeric()));
        } else if self.content[0].is_numeric() {
            return Some(self.take_while(|c| c.is_numeric()));
        }

        Some(self.take(1))
    }
}
