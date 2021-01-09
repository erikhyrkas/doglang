use std::sync::Once;

use crate::lex::and_group::AndGroup;
use crate::lex::any_char::AnyChar;
use crate::lex::character_range::CharacterRange;
use crate::lex::group::Group;
use crate::lex::match_repeats::{GroupRepeats, MatchRepeats};
use crate::lex::or_group::OrGroup;
use crate::lex::pattern::Pattern;
use crate::lex::text_pattern::TextPattern;

static mut PATTERNS: Vec<Pattern> = Vec::new();
static PATTERNS_INIT: Once = Once::new();

pub fn get_patterns() -> &'static Vec<Pattern> {
    unsafe {
        PATTERNS_INIT.call_once(|| {
            build_patterns();
        });
        return &PATTERNS;
    }
}

unsafe fn build_patterns() {
    // these patterns are in order of precedence from highest to lowest

    PATTERNS.push(build_keyword("_log", "log"));

    // assignment
    PATTERNS.push(build_keyword("_let", "let"));

    // flow control
    PATTERNS.push(build_keyword("_return", "return"));
    PATTERNS.push(build_keyword("_if", "if"));
    PATTERNS.push(build_keyword("_else", "else"));
    PATTERNS.push(build_keyword("_switch", "switch"));
    PATTERNS.push(build_keyword("_case", "case"));
    PATTERNS.push(build_keyword("_default", "default"));
    PATTERNS.push(build_keyword("_fail", "fail"));
    PATTERNS.push(build_keyword("_otherwise", "otherwise"));
    PATTERNS.push(build_keyword("_for", "for"));
    PATTERNS.push(build_keyword("_in", "in"));
    PATTERNS.push(build_keyword("_while", "while"));
    PATTERNS.push(build_keyword("_break", "break"));
    PATTERNS.push(build_keyword("_continue", "continue"));
    PATTERNS.push(build_keyword("_with", "with")); // syntactical sugar to acquire/release resources
    PATTERNS.push(build_keyword("_isa", "isa"));

    // entry points
    PATTERNS.push(build_keyword("_app", "app"));
    PATTERNS.push(build_keyword("_lib", "lib"));
    PATTERNS.push(build_keyword("_ui", "ui")); // reserving for now, even though we may not implement
    PATTERNS.push(build_keyword("_service", "service"));
    PATTERNS.push(build_keyword("_test", "test"));

    // blocks
    PATTERNS.push(build_keyword("_config", "config"));
    PATTERNS.push(build_keyword("_function", "fn"));
    PATTERNS.push(build_keyword("_struct", "struct"));
    PATTERNS.push(build_keyword("_enum", "enum"));
    PATTERNS.push(build_keyword("_trait", "trait"));
    PATTERNS.push(build_keyword("_impl", "impl"));
    PATTERNS.push(build_keyword("_attribute", "attr")); // java annotation/rust attribute

    // qualifiers
    PATTERNS.push(build_keyword("_self", "self"));
    PATTERNS.push(build_keyword("_public", "pub"));
    PATTERNS.push(build_keyword("_mutable", "mut"));
    PATTERNS.push(build_keyword("_constant", "const"));
    PATTERNS.push(build_keyword("_once", "once"));
    //PATTERNS.push(build_keyword("_reference", "ref"));
    PATTERNS.push(build_keyword("_unsafe", "unsafe"));

    // scope
    PATTERNS.push(build_keyword("_use", "use")); // use [external library name::]<module name>::<submodule name>[::func] as name
    PATTERNS.push(build_keyword("_as", "as")); // cast variable || use [external library name::]<module name>::<submodule name>::global_var as name
    PATTERNS.push(build_keyword("_module", "mod")); // mod [[module name]::[submodulename]] [(os: "windows", arch: "x86")]

    // types
    PATTERNS.push(build_keyword("_unsigned_integer", "uint"));
    PATTERNS.push(build_keyword("_integer", "int"));
    PATTERNS.push(build_keyword("_float", "float"));
    PATTERNS.push(build_keyword("_boolean", "bool"));
    PATTERNS.push(build_keyword("_character", "char"));
    PATTERNS.push(build_keyword("_void", "void"));

    PATTERNS.push(build_keyword("_false", "false"));
    PATTERNS.push(build_keyword("_true", "true"));
    PATTERNS.push(build_keyword("_null", "null"));

    PATTERNS.push(build_keyword("_f32", "f32"));
    PATTERNS.push(build_keyword("_f64", "f64"));
    PATTERNS.push(build_keyword("_i8", "i8"));
    PATTERNS.push(build_keyword("_i16", "i16"));
    PATTERNS.push(build_keyword("_i32", "i32"));
    PATTERNS.push(build_keyword("_i64", "i64"));
    PATTERNS.push(build_keyword("_u8", "u8"));
    PATTERNS.push(build_keyword("_u16", "u16"));
    PATTERNS.push(build_keyword("_u32", "u32"));
    PATTERNS.push(build_keyword("_u64", "u64"));

    // building block patterns
    PATTERNS.push(build_comment());
    PATTERNS.push(build_sql_string());
    PATTERNS.push(build_quoted_string());
    PATTERNS.push(build_number());
    PATTERNS.push(build_word());

    // symbols
    PATTERNS.push(build_named_character("_open_curly", '{'));
    PATTERNS.push(build_named_character("_close_curly", '}'));
    PATTERNS.push(build_named_character("_comma", ','));
    PATTERNS.push(build_named_character("_equal", '='));
    PATTERNS.push(build_named_character("_greater", '>'));
    PATTERNS.push(build_named_character("_less", '<'));
    PATTERNS.push(build_named_character("_plus", '+'));
    PATTERNS.push(build_named_character("_minus", '-'));
    PATTERNS.push(build_named_character("_star", '*'));
    PATTERNS.push(build_named_character("_period", '.'));
    PATTERNS.push(build_named_character("_slash", '/'));
    PATTERNS.push(build_named_character("_hash", '#'));
    PATTERNS.push(build_named_character("_open_paren", '('));
    PATTERNS.push(build_named_character("_close_paren", ')'));
    PATTERNS.push(build_named_character("_open_bracket", '['));
    PATTERNS.push(build_named_character("_close_bracket", ']'));
    PATTERNS.push(build_named_character("_exclamation", '!'));
    PATTERNS.push(build_named_character("_question_mark", '?'));
    PATTERNS.push(build_named_character("_colon", ':'));
    PATTERNS.push(build_named_character("_pipe", '|'));

    // whitespace
    PATTERNS.push(newline());
    PATTERNS.push(whitespace());
}

fn build_keyword(name: &str, text: &str) -> Pattern {
    return Pattern {
        label: name.to_string(),
        pattern_group: Box::new(TextPattern {
            match_text: text.to_string()
        }),
        skip: false,
    };
}

fn build_comment() -> Pattern {
    return Pattern {
        label: "_comment".to_string(),
        pattern_group: Box::new(AndGroup {
            groups: vec![
                Box::new(CharacterRange {
                    match_start_char: '/',
                    match_end_char: '/',
                }),
                Box::new(CharacterRange {
                    match_start_char: '/',
                    match_end_char: '/',
                }),
                Box::new(GroupRepeats {
                    match_repeats: MatchRepeats::ZeroOrMore,
                    group: Box::new(AnyChar {}),
                }),
                Box::new(CharacterRange {
                    match_start_char: '\n',
                    match_end_char: '\n',
                }),
            ]
        }),
        skip: true,
    };
}


fn build_sql_string() -> Pattern {
    return Pattern {
        label: "_sql".to_string(),
        pattern_group:
        Box::new(AndGroup {
            groups: vec![
                Box::new(CharacterRange {
                    match_start_char: '`',
                    match_end_char: '`',
                }),
                Box::new(GroupRepeats {
                    match_repeats: MatchRepeats::ZeroOrMore,
                    group: Box::new(OrGroup {
                        groups: vec![
                            Box::new(AnyChar {}),
                            Box::new(TextPattern {
                                match_text: "\\`".to_string()
                            })
                        ]
                    }),
                }),
                Box::new(CharacterRange {
                    match_start_char: '`',
                    match_end_char: '`',
                }),
            ]
        }),
        skip: false,
    };
}

fn build_quoted_string() -> Pattern {
    return Pattern {
        label: "_string_literal".to_string(),
        pattern_group:
        Box::new(OrGroup {
            groups: vec![
                Box::new(AndGroup {
                    groups: vec![
                        Box::new(CharacterRange {
                            match_start_char: '"',
                            match_end_char: '"',
                        }),
                        Box::new(GroupRepeats {
                            match_repeats: MatchRepeats::ZeroOrMore,
                            group: Box::new(OrGroup {
                                groups: vec![
                                    Box::new(AnyChar {}),
                                    Box::new(TextPattern {
                                        match_text: "\\\"".to_string()
                                    })
                                ]
                            }),
                        }),
                        Box::new(CharacterRange {
                            match_start_char: '"',
                            match_end_char: '"',
                        }),
                    ]
                }),
                Box::new(AndGroup {
                    groups: vec![
                        Box::new(CharacterRange {
                            match_start_char: '\'',
                            match_end_char: '\'',
                        }),
                        Box::new(GroupRepeats {
                            match_repeats: MatchRepeats::ZeroOrMore,
                            group: Box::new(OrGroup {
                                groups: vec![
                                    Box::new(AnyChar {}),
                                    Box::new(TextPattern {
                                        match_text: "\\\'".to_string()
                                    })
                                ]
                            }),
                        }),
                        Box::new(CharacterRange {
                            match_start_char: '\'',
                            match_end_char: '\'',
                        }),
                    ]
                }),
            ]
        }),
        skip: false,
    };
}

fn build_number() -> Pattern {
    let mut hex_patterns: Vec<Box<dyn Group>> = vec![];
    hex_patterns.push(Box::new(CharacterRange {
        match_start_char: '0',
        match_end_char: '9',
    }));
    hex_patterns.push(Box::new(CharacterRange {
        match_start_char: 'a',
        match_end_char: 'f',
    }));
    hex_patterns.push(Box::new(CharacterRange {
        match_start_char: 'A',
        match_end_char: 'F',
    }));

    return Pattern {
        label: "_number_literal".to_string(),
        pattern_group: Box::new(OrGroup {
            groups: vec![
                Box::new(AndGroup {
                    groups: vec![
                        Box::new(TextPattern {
                            match_text: "0x".to_string()
                        }),
                        Box::new(GroupRepeats {
                            match_repeats: MatchRepeats::OneOrMore,
                            group: Box::new(OrGroup {
                                groups: hex_patterns
                            }),
                        })
                    ]
                }),
                Box::new(AndGroup {
                    groups: vec![
                        Box::new(GroupRepeats {
                            match_repeats: MatchRepeats::OneOrMore,
                            group: Box::new(CharacterRange {
                                match_start_char: '0',
                                match_end_char: '9',
                            }),
                        }),
                        Box::new(CharacterRange {
                            match_start_char: '.',
                            match_end_char: '.',
                        }),
                        Box::new(GroupRepeats {
                            match_repeats: MatchRepeats::OneOrMore,
                            group: Box::new(CharacterRange {
                                match_start_char: '0',
                                match_end_char: '9',
                            }),
                        })
                    ]
                }),
                Box::new(AndGroup {
                    groups: vec![
                        Box::new(GroupRepeats {
                            match_repeats: MatchRepeats::OneOrMore,
                            group: Box::new(CharacterRange {
                                match_start_char: '0',
                                match_end_char: '9',
                            }),
                        })
                    ]
                }),
            ]
        }),
        skip: false,
    };
}

fn build_word() -> Pattern {
    let mut first_character_pattern: Vec<Box<dyn Group>> = vec![];

    first_character_pattern.push(Box::new(CharacterRange {
        match_start_char: 'a',
        match_end_char: 'z',
    }));
    first_character_pattern.push(Box::new(CharacterRange {
        match_start_char: 'A',
        match_end_char: 'Z',
    }));

    let mut body_patterns: Vec<Box<dyn Group>> = vec![];
    body_patterns.push(Box::new(CharacterRange {
        match_start_char: 'a',
        match_end_char: 'z',
    }));
    body_patterns.push(Box::new(CharacterRange {
        match_start_char: 'A',
        match_end_char: 'Z',
    }));
    body_patterns.push(Box::new(CharacterRange {
        match_start_char: '0',
        match_end_char: '9',
    }));
    body_patterns.push(Box::new(CharacterRange {
        match_start_char: '_',
        match_end_char: '_',
    }));

    return build_pattern(first_character_pattern, body_patterns);
}

pub fn build_pattern(first_character_pattern: Vec<Box<dyn Group>>, body_patterns: Vec<Box<dyn Group>>) -> Pattern {
    return Pattern {
        label: "_word".to_string(),
        pattern_group: Box::new(AndGroup {
            groups: vec![
                Box::new(OrGroup {
                    groups: first_character_pattern,
                }),
                Box::new(GroupRepeats {
                    match_repeats: MatchRepeats::ZeroOrMore,
                    group: Box::new(OrGroup {
                        groups: body_patterns,
                    }),
                }),
            ]
        }),
        skip: false,
    };
}


fn build_named_character(label: &str, c: char) -> Pattern {
    return Pattern {
        label: label.to_string(),
        pattern_group: Box::new(TextPattern {
            match_text: c.to_string(),
        }),
        skip: false,
    };
}

fn newline() -> Pattern {
    return Pattern {
        label: "_end_of_line".to_string(),
        pattern_group: Box::new(CharacterRange {
            match_start_char: '\n',
            match_end_char: '\n',
        }),
        skip: true,
    };
}

fn whitespace() -> Pattern {
    return Pattern {
        label: "_whitespace".to_string(),
        pattern_group: Box::new(GroupRepeats {
            match_repeats: MatchRepeats::OneOrMore,
            group: Box::new(OrGroup {
                groups: vec![
                    Box::new(GroupRepeats {
                        match_repeats: MatchRepeats::OneOrMore,
                        group: Box::new(TextPattern {
                            match_text: " ".to_string(),
                        }),
                    }),
                    Box::new(GroupRepeats {
                        match_repeats: MatchRepeats::OneOrMore,
                        group: Box::new(CharacterRange {
                            match_start_char: '\t',
                            match_end_char: '\t',
                        }),
                    }),
                    Box::new(GroupRepeats {
                        match_repeats: MatchRepeats::OneOrMore,
                        group: Box::new(CharacterRange {
                            match_start_char: '\r',
                            match_end_char: '\r',
                        }),
                    }),
                ],
            }),
        }),
        skip: true,
    };
}