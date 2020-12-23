use crate::lex::group::Group;

#[derive(PartialEq, Eq, Debug)]
pub enum MatchRepeats {
    Once,
    ZeroOrMore,
    OneOrMore,
    ZeroOrOne,
}

pub fn min_matches(match_repeats: &MatchRepeats) -> i32 {
    let result;
    match match_repeats {
        MatchRepeats::Once => { result = 1 }
        MatchRepeats::ZeroOrMore => { result = 0 }
        MatchRepeats::OneOrMore => { result = 1 }
        MatchRepeats::ZeroOrOne => { result = 0 }
    }
    return result;
}


pub struct GroupRepeats {
    pub match_repeats: MatchRepeats,
    pub group: Box<dyn Group>,
}

impl Group for GroupRepeats {
    fn match_with(&self, text: &[char], offset: usize) -> Option<u32> {
        let first_match = self.group.match_with(text, offset);
        if self.match_repeats == MatchRepeats::Once {
            return first_match;
        }
        if first_match.is_none() {
            if self.match_repeats == MatchRepeats::ZeroOrMore
                || self.match_repeats == MatchRepeats::ZeroOrOne {
                return Some(0);
            }
            return None;
        }
        if self.match_repeats == MatchRepeats::ZeroOrOne {
            return first_match;
        }
        let mut total_length = first_match.unwrap();
        if total_length > 0 {
            loop {
                if let Some(next_match) = self.group.match_with(text, offset + total_length as usize) {
                    if next_match == 0 {
                        break;
                    }
                    total_length += next_match;
                } else {
                    break;
                }
            }
        }
        return Some(total_length);
    }

    fn contains_any(&self) -> bool {
        // i think this is the correct behavior
        // contains any is used to help the AndGroup decide if it needs to look ahead
        return self.group.contains_any();
    }

    fn name(&self) -> String {
        return "MatchRepeats".to_string();
    }

    fn min_matches(&self) -> usize {
        let minimum_matches = min_matches(&self.match_repeats);
        if minimum_matches == 0 {
            return 0;
        }
        return minimum_matches as usize * self.group.min_matches();
    }

    fn render(&self) -> String {
        let mut result: String = String::from(&*self.group.render());
        match self.match_repeats {
            MatchRepeats::Once => {}
            MatchRepeats::ZeroOrMore => {
                result += "*";
            }
            MatchRepeats::OneOrMore => {
                result += "+";
            }
            MatchRepeats::ZeroOrOne => {
                result += "?";
            }
        }
        return result;
    }
}