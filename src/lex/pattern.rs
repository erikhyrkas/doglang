// the pattern is the heart of defining a token

use crate::lex::token_stream::Match;
use crate::lex::group::Group;

pub struct Pattern {
    pub label: String,
    pub pattern_group: Box<dyn Group>,
    pub skip: bool,
}

pub trait PatternMatcher {
    fn match_with(&self, text: &[char], offset: usize) -> Option<Match>;
}

impl PatternMatcher for Pattern {
    fn match_with(&self, text: &[char], offset: usize) -> Option<Match> {
        //println!("Testing group: {} {} {}", self.label, self.pattern_group.name(), self.pattern_group.render());
        if let Some(len) = self.pattern_group.match_with(text, offset) {
            let end_offset = offset + (len as usize);
            let matched_text = text[offset..end_offset].iter().collect();
            return Some(Match {
                length: len as usize,
                label: self.label.clone(),
                value: matched_text,
                skip: self.skip,
                line_number: 0,
                line_offset: 0,
                file_name: None,
                file_path: None,
            });
        }
        return None;
    }
}



