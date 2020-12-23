use crate::lex::group::Group;

pub struct TextPattern {
    pub match_text: String,
}

impl Group for TextPattern {
    fn match_with(&self, text: &[char], offset: usize) -> Option<u32> {
        let match_text_len = self.match_text.len();
        if offset + match_text_len > text.len() {
            return None;
        }

        for (index, c) in self.match_text.chars().enumerate() {
            if text[offset + index] != c {
                return None;
            }
        }

        return Some(match_text_len as u32);
    }

    fn contains_any(&self) -> bool {
        return false;
    }

    fn name(&self) -> String {
        return "TextPattern".to_string();
    }
    fn min_matches(&self) -> usize {
        return self.match_text.len();
    }

    fn render(&self) -> String {
        let mut result: String = String::from("'");
        result += &*self.match_text.replace("'", "\\'");
        result += "'";
        return result;
    }
}
