use core::option::Option;

use crate::lex::group::Group;

pub struct OrGroup {
    pub groups: Vec<Box<dyn Group>>,
}

impl Group for OrGroup {
    fn match_with(&self, text: &[char], offset: usize) -> Option<u32> {
        if self.groups.is_empty() {
            panic!("At least one group is required in an or-group.");
        }

        // This is stupid that I need two variables to detect whether we found a match.
        // However, when I try to use one, it gets complicated. More research needed.
        let mut no_matches = true;
        let mut longest_match_len: u32 = 0;
        for group in &self.groups {
            if let Some(next_match) = group.match_with(text, offset) {
                if no_matches || next_match > longest_match_len {
                    longest_match_len = next_match;
                    no_matches = false;
                }
            }
        }
        if no_matches {
            return None;
        }

        return Some(longest_match_len);
    }

    fn contains_any(&self) -> bool {
        for group in &self.groups {
            if group.contains_any() {
                return true;
            }
        }
        return false;
    }

    fn name(&self) -> String {
        return "OrGroup".to_string();
    }

    fn min_matches(&self) -> usize {
        let mut max_match: usize = 0;
        for group in &self.groups {
            let next_match_amount = group.min_matches();
            if next_match_amount > max_match {
                max_match = next_match_amount;
            }
        }
        return max_match;
    }

    fn render(&self) -> String {
        if self.groups.len() == 1 {
            return self.groups[0].render();
        }
        let mut result: String = String::from("(");
        let mut delim = "";
        for group in &self.groups {
            result += &*String::from(delim);
            delim = " || ";
            result += &*group.render();
        }
        result += ")";
        return result;
    }
}
