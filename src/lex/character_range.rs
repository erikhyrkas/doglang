use crate::lex::group::Group;
use crate::lex::render_char;

#[derive(Debug)]
pub struct CharacterRange {
    pub match_start_char: char,
    pub match_end_char: char,
}

impl Group for CharacterRange {
    fn match_with(&self, text: &[char], offset: usize) -> Option<u32> {
        let target_char = text[offset];
        if target_char >= self.match_start_char && target_char <= self.match_end_char {
            return Some(1);
        }
        return None;
    }

    fn contains_any(&self) -> bool {
        return false;
    }

    fn name(&self) -> String {
        return "CharacterRange".to_string();
    }

    fn min_matches(&self) -> usize {
        return 1;
    }

    fn render(&self) -> String {
        let mut result: String = String::from("[");
        if self.match_start_char == self.match_end_char {
            result += &*render_char(self.match_start_char);
        } else {
            result += &*render_char(self.match_start_char);
            result += "-";
            result += &*render_char(self.match_end_char);
        }
        result += "]";
        return result;
    }
}

