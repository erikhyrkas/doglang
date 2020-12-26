//lexer
// While there are lexer-parser generators which work with well-formed syntax, the error
// handling is not useful for helping users understand where they went wrong.
// This lexer is simple, but we'll improve on it over time.
//
// The goal of a lexer is to create a list of tokens that can be passed to a parser, which will
// utilize groups of tokens to understand the text's intent.

use std::cmp::min;
use std::iter::FromIterator;

use pattern::PatternMatcher;
use token_stream::TokenStream;

use crate::lex::pattern_init::get_patterns;
use crate::lex::token_stream::Match;
use crate::UNKNOWN;

pub(crate) mod token_stream;
mod pattern;
mod or_group;
mod text_pattern;
mod match_repeats;
mod pattern_init;
mod and_group;
mod not_group;
mod character_range;
mod any_char;
mod group;

// todo: right now, this is only using the dog grammar, but we need to add a function or enum
// that supports the SQL grammar.
pub fn lex(dog_code: &str, file_name: Option<&str>, file_path: Option<&str>) -> Option<Box<TokenStream>> {
    let real_file_name = String::from(file_name.unwrap_or(UNKNOWN));
    let real_file_path = String::from(file_path.unwrap_or(UNKNOWN));
    println!("Lexing: {} ({})", real_file_name, real_file_path);

    let text: Vec<char> = dog_code.chars().collect();

    let mut line_number: usize = 1;
    let mut line_offset: usize = 1;
    let mut matches = vec![];
    let mut offset = 0;
    while offset < text.len() {
        //println!("{} {}", offset, text[offset]);
        //println!("Text is: [{}]", render_string(String::from_iter(text[offset..min(text.len(), offset + 5)].iter())));
        if let Some(mut longest_match) = find_longest_match(&text, offset) {
            longest_match.line_number = line_number;
            longest_match.line_offset = line_offset;
            longest_match.file_path = Some(real_file_path.clone());
            longest_match.file_name = Some(real_file_name.clone());
            //println!("Found pattern: {:?}", &longest_match);
            //println!();

            if longest_match.value.contains("\n") {
                line_number += 1;
                line_offset = 1;
            } else {
                line_offset += longest_match.length;
            }
            offset += longest_match.length;
            if !longest_match.skip {
                matches.push(longest_match);
            }
        } else {
            // I'm about 99% sure there must be a more elegant way of doing this.
            println!("Text is: [{}]", render_string(String::from_iter(text[offset..min(text.len(), offset + 5)].iter())));
            println!("Currently at: {} out of {} which has char [{}]", offset, text.len(), text[offset] as u16);
            let mut val = &text[offset..];
            let mut val_as_string = String::from_iter(val.iter());
            let i = val_as_string.find(' ').unwrap_or(val.len() - 1);
            let j = val_as_string.find('\n').unwrap_or(val.len() - 1);

            if i < j {
                val = &val[..i];
            } else {
                val = &val[..j];
            }

            val_as_string = String::from_iter(val.iter());
            println!("Unable to match text at [line: {}: character: {}]: [{}] [{}]", line_number, line_offset, val[0], val_as_string);
            return None;
        }
    }

    return Some(Box::new(TokenStream {
        matches,
        offset: 0,
        fresh: true,
        last_consumed_offset: None,
    }));
}

fn find_longest_match(text: &Vec<char>, offset: usize) -> Option<Box<Match>> {
    // This is stupid that I need two variables to detect whether we found a match.
    // However, when I try to use one, it gets complicated. More research needed.
    let mut longest_match: Option<Match> = None;
    let mut longest_match_len: usize = 0;
    for pattern in get_patterns() {
        let next = pattern.match_with(text, offset);
        if let Some(next) = next {
            if longest_match.is_none() || next.length > longest_match_len {
                longest_match_len = next.length;
                longest_match = Some(next);
            }
        }
    }

    if longest_match.is_none() {
        return None;
    }
    return Some(Box::new(longest_match.unwrap()));
}

fn render_string(s: String) -> String {
    let mut result = s.replace("\n", "\\n");
    result = result.replace("\r", "\\r");
    result = result.replace("\t", "\\t");
    return result;
}

fn render_char(c: char) -> String {
    if c.is_whitespace() {
        if c == '\n' {
            return "\\n".to_string();
        }
        if c == '\r' {
            return "\\r".to_string();
        }
        if c == '\t' {
            return "\\t".to_string();
        }
        return "\\s".to_string();
    }
    return c.to_string();
}


#[cfg(test)]
mod lex_tests {
    use crate::lex::and_group::AndGroup;
    use crate::lex::any_char::AnyChar;
    use crate::lex::group::Group;
    use crate::lex::match_repeats::{GroupRepeats, MatchRepeats};
    use crate::lex::or_group::OrGroup;
    use crate::lex::pattern::Pattern;
    use crate::lex::pattern_init::build_pattern;
    use crate::lex::text_pattern::TextPattern;

    use super::*;

    #[test]
    fn lex_simple() {
        let text: Vec<char> = "app main() {}".chars().collect();

        let option = find_longest_match(&text, 0);
        assert_eq!(true, option.is_some());
        let match_obj = option.unwrap();
        let match_value = match_obj.value;
        assert_eq!("word", match_obj.label);
        assert_eq!("app", match_value);
    }

    #[test]
    fn text_match_with_and() {
        let mut alpha_patterns: Vec<Box<dyn Group>> = vec![];
        let mut body_patterns: Vec<Box<dyn Group>> = vec![];
        alpha_patterns.push(Box::new(TextPattern {
            match_text: 'a'.to_string(),
        }));
        body_patterns.push(Box::new(TextPattern {
            match_text: 'p'.to_string(),
        }));

        let pat = build_pattern(alpha_patterns, body_patterns);

        let text: Vec<char> = "app main() {}".chars().collect();
        let option = pat.match_with(&text, 0);
        assert_eq!(true, option.is_some());
        let match_obj = option.unwrap();
        let match_value = match_obj.value;
        assert_eq!("word", match_obj.label);
        assert_eq!("app", match_value);
    }

    #[test]
    fn text_match_with_and_any() {
        let mut alpha_patterns: Vec<Box<dyn Group>> = vec![];
        let mut body_patterns: Vec<Box<dyn Group>> = vec![];
        let mut tail_patterns: Vec<Box<dyn Group>> = vec![];
        alpha_patterns.push(Box::new(TextPattern {
            match_text: 'a'.to_string(),
        }));
        body_patterns.push(Box::new(AnyChar {}));
        tail_patterns.push(Box::new(TextPattern {
            match_text: 't'.to_string(),
        }));

        let pat = Pattern {
            label: "word".to_string(),
            pattern_group: Box::new(AndGroup {
                groups: vec![
                    Box::new(OrGroup {
                        groups: alpha_patterns,
                    }),
                    Box::new(GroupRepeats {
                        match_repeats: MatchRepeats::ZeroOrMore,
                        group: Box::new(OrGroup {
                            groups: body_patterns,
                        }),
                    }),
                    Box::new(GroupRepeats {
                        match_repeats: MatchRepeats::OneOrMore,
                        group: Box::new(OrGroup {
                            groups: tail_patterns,
                        }),
                    }),
                ]
            }),
            skip: false,
        };

        let text: Vec<char> = "apt words".chars().collect();
        let option = pat.match_with(&text, 0);
        assert_eq!(true, option.is_some());
        let match_obj = option.unwrap();
        let match_value = match_obj.value;
        assert_eq!("word", match_obj.label);
        assert_eq!("apt", match_value);
    }

    #[test]
    fn text_match_simple_or() {
        let mut alpha_patterns: Vec<Box<dyn Group>> = vec![];
        alpha_patterns.push(Box::new(TextPattern {
            match_text: 'a'.to_string(),
        }));
        alpha_patterns.push(Box::new(TextPattern {
            match_text: 'p'.to_string(),
        }));

        let pat = test_word(alpha_patterns);

        let text: Vec<char> = "app main() {}".chars().collect();
        let option = pat.match_with(&text, 0);
        assert_eq!(true, option.is_some());
        let match_obj = option.unwrap();
        let match_value = match_obj.value;
        assert_eq!("word", match_obj.label);
        assert_eq!("app", match_value);
    }

    fn test_word(alpha_patterns: Vec<Box<dyn Group>>) -> Pattern {
        return Pattern {
            label: "word".to_string(),
            pattern_group: Box::new(GroupRepeats {
                match_repeats: MatchRepeats::OneOrMore,
                group: Box::new(OrGroup {
                    groups: alpha_patterns,
                }),
            }),
            skip: false,
        };
    }

    #[test]
    fn text_match_simple_and() {
        let mut alpha_patterns: Vec<Box<dyn Group>> = vec![];
        alpha_patterns.push(Box::new(TextPattern {
            match_text: 'a'.to_string(),
        }));
        alpha_patterns.push(Box::new(AnyChar {}));
        alpha_patterns.push(Box::new(TextPattern {
            match_text: 'p'.to_string(),
        }));

        let pat = Pattern {
            label: "word".to_string(),
            pattern_group: Box::new(GroupRepeats {
                match_repeats: MatchRepeats::OneOrMore,
                group: Box::new(AndGroup {
                    groups: alpha_patterns
                }),
            }),
            skip: false,
        };

        let text: Vec<char> = "app main() {}".chars().collect();
        let option = pat.match_with(&text, 0);
        assert_eq!(true, option.is_some());
        let match_obj = option.unwrap();
        let match_value = match_obj.value;
        assert_eq!("word", match_obj.label);
        assert_eq!("app", match_value);
    }

    #[test]
    fn text_match_any_or() {
        let mut alpha_patterns: Vec<Box<dyn Group>> = vec![];
        alpha_patterns.push(Box::new(TextPattern {
            match_text: 'a'.to_string(),
        }));
        alpha_patterns.push(Box::new(AnyChar {}));
        alpha_patterns.push(Box::new(TextPattern {
            match_text: 't'.to_string(),
        }));


        let pat = Pattern {
            label: "word".to_string(),
            pattern_group: Box::new(GroupRepeats {
                match_repeats: MatchRepeats::OneOrMore,
                group: Box::new(OrGroup {
                    groups: alpha_patterns,
                }),
            }),
            skip: false,
        };

        let text: Vec<char> = "apt words".chars().collect();
        let option = pat.match_with(&text, 0);
        assert_eq!(true, option.is_some());
        let match_obj = option.unwrap();
        let match_value = match_obj.value;
        assert_eq!("word", match_obj.label);
        assert_eq!("apt words", match_value);
    }
}