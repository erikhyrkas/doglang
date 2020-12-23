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

    // assignment
    PATTERNS.push(build_keyword("let", "let"));

    // flow control
    PATTERNS.push(build_keyword("return", "return"));
    PATTERNS.push(build_keyword("if", "if"));
    PATTERNS.push(build_keyword("else", "else"));
    PATTERNS.push(build_keyword("switch", "switch"));
    PATTERNS.push(build_keyword("case", "case"));
    PATTERNS.push(build_keyword("default", "default"));
    PATTERNS.push(build_keyword("fail", "fail"));
    PATTERNS.push(build_keyword("otherwise", "otherwise"));
    PATTERNS.push(build_keyword("for", "for"));
    PATTERNS.push(build_keyword("in", "in"));
    PATTERNS.push(build_keyword("while", "while"));
    PATTERNS.push(build_keyword("break", "break"));
    PATTERNS.push(build_keyword("continue", "continue"));
    PATTERNS.push(build_keyword("with", "with")); // syntactical sugar to acquire/release resources

    // entry points
    PATTERNS.push(build_keyword("app", "app"));
    PATTERNS.push(build_keyword("lib", "lib"));
    PATTERNS.push(build_keyword("ui", "ui")); // reserving for now, even though we may not implement
    PATTERNS.push(build_keyword("service", "service"));
    PATTERNS.push(build_keyword("test", "test"));

    // blocks
    PATTERNS.push(build_keyword("config", "config"));
    PATTERNS.push(build_keyword("function", "fn"));
    PATTERNS.push(build_keyword("struct", "struct"));
    PATTERNS.push(build_keyword("enum", "enum"));
    PATTERNS.push(build_keyword("trait", "trait"));
    PATTERNS.push(build_keyword("impl", "impl"));
    PATTERNS.push(build_keyword("attribute", "attr")); // java annotation/rust attribute

    // qualifiers
    PATTERNS.push(build_keyword("self", "self"));
    PATTERNS.push(build_keyword("public", "pub"));
    PATTERNS.push(build_keyword("mutable", "mut"));
    PATTERNS.push(build_keyword("constant", "const"));
    //PATTERNS.push(build_keyword("reference", "ref"));
    PATTERNS.push(build_keyword("unsafe", "unsafe"));

    // scope
    PATTERNS.push(build_keyword("use", "use")); // use [external library name::]<module name>::<submodule name>[::func] as name
    PATTERNS.push(build_keyword("as", "as")); // use [external library name::]<module name>::<submodule name>::global_var as name
    PATTERNS.push(build_keyword("module", "mod")); // mod [[module name]::[submodulename]] [(os: "windows", arch: "x86")]

    // types
    //PATTERNS.push(build_keyword("string", "str")); // we'll use a struct/trait for this
    PATTERNS.push(build_keyword("unsigned integer", "uint"));
    PATTERNS.push(build_keyword("integer", "int"));
    PATTERNS.push(build_keyword("float", "float"));
    PATTERNS.push(build_keyword("boolean", "bool"));
    PATTERNS.push(build_keyword("character", "char"));

    PATTERNS.push(build_keyword("false", "false"));
    PATTERNS.push(build_keyword("true", "true"));
    PATTERNS.push(build_keyword("null", "null"));

    PATTERNS.push(build_keyword("f32", "f32"));
    PATTERNS.push(build_keyword("f64", "f64"));
    PATTERNS.push(build_keyword("i8", "i8"));
    PATTERNS.push(build_keyword("i16", "i16"));
    PATTERNS.push(build_keyword("i32", "i32"));
    PATTERNS.push(build_keyword("i64", "i64"));
    PATTERNS.push(build_keyword("u8", "u8"));
    PATTERNS.push(build_keyword("u16", "u16"));
    PATTERNS.push(build_keyword("u32", "u32"));
    PATTERNS.push(build_keyword("u64", "u64"));

    // building block patterns
    PATTERNS.push(build_comment());
    PATTERNS.push(build_quoted_string());
    PATTERNS.push(build_quoted_char());
    PATTERNS.push(build_number());
    PATTERNS.push(build_word());

    // symbols
    PATTERNS.push(build_named_character("openCurly", '{'));
    PATTERNS.push(build_named_character("closeCurly", '}'));
    PATTERNS.push(build_named_character("comma", ','));
    PATTERNS.push(build_named_character("equal", '='));
    PATTERNS.push(build_named_character("greater", '>'));
    PATTERNS.push(build_named_character("less", '<'));
    PATTERNS.push(build_named_character("plus", '+'));
    PATTERNS.push(build_named_character("minus", '-'));
    PATTERNS.push(build_named_character("star", '*'));
    PATTERNS.push(build_named_character("period", '.'));
    PATTERNS.push(build_named_character("slash", '/'));
    PATTERNS.push(build_named_character("hash", '#'));
    PATTERNS.push(build_named_character("openParen", '('));
    PATTERNS.push(build_named_character("closeParen", ')'));
    PATTERNS.push(build_named_character("openBracket", '['));
    PATTERNS.push(build_named_character("closeBracket", ']'));
    PATTERNS.push(build_named_character("exclamation", '!'));
    PATTERNS.push(build_named_character("questionMark", '?'));
    PATTERNS.push(build_named_character("colon", ':'));
    PATTERNS.push(build_named_character("pipe", '|'));

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
        label: "comment".to_string(),
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

fn build_quoted_string() -> Pattern {
    return Pattern {
        label: "string".to_string(),
        pattern_group: Box::new(AndGroup {
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
        skip: false,
    };
}

fn build_quoted_char() -> Pattern {
    return Pattern {
        label: "char".to_string(),
        pattern_group: Box::new(AndGroup {
            groups: vec![
                Box::new(CharacterRange {
                    match_start_char: '\'',
                    match_end_char: '\'',
                }),
                Box::new(GroupRepeats {
                    match_repeats: MatchRepeats::ZeroOrOne,
                    group: Box::new(CharacterRange {
                        match_start_char: '\\',
                        match_end_char: '\\',
                    }),
                }),
                Box::new(AnyChar {}),
                Box::new(CharacterRange {
                    match_start_char: '\'',
                    match_end_char: '\'',
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
        label: "number".to_string(),
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

    return build_pattern(first_character_pattern, body_patterns);
}

pub fn build_pattern(first_character_pattern: Vec<Box<dyn Group>>, body_patterns: Vec<Box<dyn Group>>) -> Pattern {
    return Pattern {
        label: "word".to_string(),
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
        label: "end_of_line".to_string(),
        pattern_group: Box::new(CharacterRange {
            match_start_char: '\n',
            match_end_char: '\n',
        }),
        skip: true,
    };
}

fn whitespace() -> Pattern {
    return Pattern {
        label: "whitespace".to_string(),
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