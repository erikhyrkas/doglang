use crate::lex::group::Group;

#[derive(Debug)]
pub struct AnyChar {
}

impl Group for AnyChar {
    fn match_with(&self, text: &[char], offset: usize) -> Option<u32> {
        if text.len() > offset {
            return Some(1);
        }
        return None;
    }

    fn contains_any(&self) -> bool {
        return true;
    }

    fn name(&self) -> String {
        return "AnyChar".to_string();
    }
    fn min_matches(&self) -> usize {
        return 1;
    }

    fn render(&self) -> String {
        return ".".to_string();
    }
}
