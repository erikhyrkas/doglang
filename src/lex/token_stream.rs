
#[derive(Debug)]
pub struct TokenStream {
    pub matches: Vec<Box<Match>>,
    pub offset: usize,
    pub last_consumed_offset: Option<usize>,
}

#[derive(Debug)]
pub struct Match {
    pub length: usize,
    pub label: String,
    pub value: String,
    pub skip: bool,
    pub line_number: usize,
    pub line_offset: usize,
    pub file_name: Option<String>,
    pub file_path: Option<String>,
}

impl TokenStream {
    fn find_next_offset(&self) -> Option<usize> {
        if self.matches.is_empty() || self.offset == self.matches.len() {
            return None;
        }
        for next_offset in self.offset + 1..self.matches.len() - 1 {
            let next = &self.matches[next_offset];
            if !next.skip {
                return Some(next_offset);
            }
        }
        return None;
    }

    #[allow(dead_code)]
    pub fn has_next(&self) -> bool {
        return self.find_next_offset().is_some();
    }

    #[allow(dead_code)]
    pub fn next(&mut self) -> Option<&Match> {
        let next_offset = self.find_next_offset();
        if next_offset.is_none() {
            return None;
        }

        // it is so clumsy to work with optionals sometimes.
        self.offset = next_offset.unwrap();
        if let Some(last_consumed_offset) = self.last_consumed_offset {
            if self.offset > last_consumed_offset {
                self.last_consumed_offset = Some(self.offset);
            }
        } else {
            self.last_consumed_offset = Some(self.offset);
        }

        return Some(&self.matches[self.offset]);
    }

    #[allow(dead_code)]
    pub fn last_consumed(&self) -> Option<&Match> {
        if self.matches.is_empty() || self.last_consumed_offset.is_none() {
            return None;
        }
        if let Some(last_consumed_offset) = self.last_consumed_offset {
            return Some(&self.matches[last_consumed_offset]);
        }
        return None;
    }

    #[allow(dead_code)]
    pub fn reset(&mut self, offset: usize) {
        if self.matches.is_empty() || offset >= self.matches.len() {
            return;
        }
        self.offset = offset;
    }

    #[allow(dead_code)]
    pub fn peek(&self, offset: usize) -> Option<&Match> {
        if self.matches.is_empty() || offset >= self.matches.len() {
            return None;
        }

        return Some(&self.matches[offset]);
    }

    #[allow(dead_code)]
    pub fn look_ahead(&self, count: usize) -> Option<&Match> {
        let next = self.offset + count;
        return self.peek(next);
    }

    #[allow(dead_code)]
    pub fn matches(&self) -> Vec<&Match> {
        let mut result: Vec<&Match> = Vec::new();
        for next_match in self.matches.iter() {
            result.push(&next_match);
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        return self.matches.len();
    }

    #[allow(dead_code)]
    pub fn offset(&self) -> usize {
        return self.offset;
    }
}