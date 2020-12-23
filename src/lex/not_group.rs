use crate::lex::group::Group;

pub struct NotGroup {
    pub group: Box<dyn Group>,
    pub length: u32,
}

impl Group for NotGroup {
    fn match_with(&self, text: &[char], offset: usize) -> Option<u32> {
        if self.group.match_with(text, offset).is_some() {
            return None;
        }
        return Some(self.length);
    }

    fn contains_any(&self) -> bool {
        // i think this is the correct behavior
        // contains any is used to help the AndGroup decide if it needs to look ahead
        return self.group.contains_any();
    }

    fn name(&self) -> String {
        return "NotGroup".to_string();
    }

    fn min_matches(&self) -> usize {
        // this is used to determine how many characters this will take up if it matches
        // i think using 'length' here is correct.
        return self.length as usize;
    }

    fn render(&self) -> String {
        let mut result: String = String::from("!");
        result += &*self.group.render();
        return result;
    }
}