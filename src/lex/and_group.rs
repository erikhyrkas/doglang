use core::option::Option;

use crate::lex::group::Group;

pub struct AndGroup {
    pub groups: Vec<Box<dyn Group>>,
}

impl Group for AndGroup {
    fn match_with(&self, text: &[char], offset: usize) -> Option<u32> {
        if self.groups.is_empty() {
            panic!("At least one group is required in an and-group.");
        }
        let mut count: u32 = 0;
        let last_group = self.groups.len() - 1;
        let mut index = 0;
        while index < self.groups.len() {
            let current_offset = offset + count as usize;
            let current_group = &self.groups[index];
            //println!("Matching. name: [{}] contains any: [{}] current offset: [{}] current group: [{}] last group: [{}]", current_group.name(), current_group.contains_any(), current_offset, index, last_group);
            if current_group.contains_any() && index < last_group {
                let mut next_group_match: Option<u32> = None;
                let mut next_group_offset: usize = 0;
                let next_group = &self.groups[index + 1];
                if next_group.contains_any() {
                    panic!("You can not have two match-any groups in a row!");
                }
                // starting at each of the remaining characters
                let future_start = current_offset + current_group.min_matches();
                //println!("Looking into the future starting at [{}].", future_start);
                for extra_index in future_start..text.len() {
                    //println!("scanning: [{:?}]", &text[extra_index..]);
                    next_group_match = next_group.match_with(&text, extra_index);
                    if next_group_match.is_some() {
                        next_group_offset = extra_index;
                        break;
                    }
                }
                if let Some(next_group_match) = next_group_match {
                    //println!("Found a match in the future starting at [{}].", next_group_match);
                    let in_between_text: &[char];
                    if next_group_offset == 0 {
                        in_between_text = &[];
                    } else {
                        in_between_text = &text[current_offset..next_group_offset];
                    }
                    let mut failed = true;
                    if let Some(match_amount) = current_group.match_with(in_between_text, 0) {
                        failed = current_offset + match_amount as usize > next_group_offset;
                        if !failed {
                            count += match_amount;
                            //println!("Any matched [{}] characters. (Successful match.)", match_amount);
                        //} else {
                            //println!("Any matched [{}] characters at [{}]. (Failed match, next group offset {} is less than {}.)", match_amount, current_offset, next_group_offset, current_offset + match_amount as usize);
                        }
                    }
                    if failed {
                        // the any match required more or less characters than we had available
                        //println!("Any matched 0 characters.");
                        return None;
                    }
                    index += 1;
                    count += next_group_match;
                    //println!("Any matched [{}] characters for next group. (Successful match.)", next_group_match);
                } else {
                    //println!("Any matched 0 characters for next group. (Failed match.)");
                    return None;
                }
            } else {
                if let Some(match_amount) = current_group.match_with(text, current_offset) {
                    //println!("Matched [{}] characters. (Successful match.)", match_amount);
                    count += match_amount;
                } else {
                    //println!("Matched 0 characters.");
                    return None;
                }
            }
            index += 1;
        }

        return Some(count);
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
        return "AndGroup".to_string();
    }

    fn min_matches(&self) -> usize {
        let mut total_match: usize = 0;
        for group in &self.groups {
            total_match += group.min_matches();
        }
        return total_match;
    }

    fn render(&self) -> String {
        if self.groups.len() == 1 {
            return self.groups[0].render();
        }
        let mut result: String = String::from("(");
        let mut delim = "";
        for group in &self.groups {
            result += delim;
            delim = " && ";
            result += &*group.render();
        }
        result += ")";
        return result;
    }
}