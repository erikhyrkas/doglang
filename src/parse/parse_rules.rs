use std::collections::HashMap;
use std::sync::Once;

use crate::lex::token_stream::TokenStream;
use crate::parse::parse_model::ParseModel;

pub enum RuleType {
    And,
    Or,
    Match,
}

#[allow(dead_code)]
pub enum RuleRepeats {
    Once,
    ZeroOrMore,
    OneOrMore,
    ZeroOrOne,
}

pub struct RuleStruct {
    rule_type: RuleType,
    repeat: RuleRepeats,
    children: Vec<&'static str>,
    match_labels: Vec<&'static str>,
}

static mut RULES: Option<HashMap<&'static str, Box<RuleStruct>>> = None;

static RULES_INIT: Once = Once::new();

pub fn match_document(token_stream: &mut TokenStream) -> Option<ParseModel> {
    unsafe {
        RULES_INIT.call_once(|| {
            RULES = build_rules();
        });

        return match_rule_by_name("document", token_stream);
    }
}

unsafe fn match_rule_by_name(label: &str, token_stream: &mut TokenStream) -> Option<ParseModel> {
    if let Some(rules) = &RULES {
        if let Some(rule) = rules.get(label) {
            return match_with(label, rule, token_stream);
        } else {
            panic!("!!! Rule not found: {} !!!", label);
        }
    }

    return None;
}

unsafe fn match_with(label: &str, rule_struct: &RuleStruct, token_stream: &mut TokenStream) -> Option<ParseModel> {
    return match rule_struct.repeat {
        RuleRepeats::Once => {
            // return
            single_match_with(label, rule_struct, token_stream)
        }
        RuleRepeats::ZeroOrMore => {
            let mut children = vec![];
            loop {
                if let Some(match_option) = single_match_with(label, rule_struct, token_stream) {
                    children.push(Box::new(match_option));
                } else {
                    break;
                }
            }

            // return
            Some(ParseModel {
                label: label.to_string(),
                tokens: vec![],
                children,
            })
        }
        RuleRepeats::OneOrMore => {
            let mut children = vec![];
            loop {
                if let Some(match_option) = single_match_with(label, rule_struct, token_stream) {
                    children.push(Box::new(match_option));
                } else {
                    break;
                }
            }
            if children.is_empty() {
                return None;
            }

            // return
            Some(ParseModel {
                label: label.to_string(),
                tokens: vec![],
                children,
            })
        }
        RuleRepeats::ZeroOrOne => {
            let mut children = vec![];
            if let Some(match_option) = single_match_with(label, rule_struct, token_stream) {
                children.push(Box::new(match_option));
            } else {
                return None;
            }

            // return
            Some(ParseModel {
                label: label.to_string(),
                tokens: vec![],
                children,
            })
        }
    };
}

unsafe fn single_match_with(label: &str, rule_struct: &RuleStruct, token_stream: &mut TokenStream) -> Option<ParseModel> {
    return match rule_struct.rule_type {
        RuleType::And => {
            // return
            and_match_with(label, rule_struct, token_stream)
        }
        RuleType::Or => {
            // return
            or_match_with(label, rule_struct, token_stream)
        }
        RuleType::Match => {
            // return
            label_match_with(label, rule_struct, token_stream)
        }
    };
}

unsafe fn label_match_with(label: &str, rule_struct: &RuleStruct, token_stream: &mut TokenStream) -> Option<ParseModel> {
    println!("Trying `token` rule: {}", label);

    if !token_stream.has_next() {
        println!("Missed `token` rule: {}", label);
        return None;
    }

    let mut result = Vec::new();
    let offset = token_stream.offset;
    for token in &rule_struct.match_labels {
        if let Some(next) = token_stream.next() {
            println!("Testing target [{}] vs input [{} : {}] {}", token, &next.label, &next.value, &next.skip);
            if token.to_string() == next.label {
                println!("label_match_with found a match for: {} {}", token, next.value);
                // I'm not thrilled about doing a clone here, but my rust expertise
                // is not enough to figure out how to return this data without a clone.
                // I suspect there is a way to do it with a 'lifetime' specifier, but
                // as I'm learning rust and I don't understand it well, this is what we are
                // doing for now.
                result.push(Box::new((*next).clone()));
                continue;
            }
        }
        token_stream.reset(offset);
        println!("Missed `token` rule: {}", label);
        return None;
    }

    println!("Hit `token` rule: {}", label);
    return Some(ParseModel {
        label: label.to_string(),
        tokens: result,
        children: vec![],
    });
}

unsafe fn or_match_with(label: &str, rule_struct: &RuleStruct, token_stream: &mut TokenStream) -> Option<ParseModel> {
    println!("Trying `or` rule: {}", label);

    if !token_stream.has_next() {
        println!("Missed `or` rule: {}", label);
        return None;
    }

    let offset = token_stream.offset;
    for rule in &rule_struct.children {
        if let Some(next) = match_rule_by_name(rule, token_stream) {
            println!("Hit `or` rule: {}", label);
            return Some(next);
        }
    }

    token_stream.reset(offset);
    println!("Missed `or` rule: {}", label);
    return None;
}

unsafe fn and_match_with(label: &str, rule_struct: &RuleStruct, token_stream: &mut TokenStream) -> Option<ParseModel> {
    println!("Trying `and` rule: {}", label);

    if !token_stream.has_next() {
        println!("Token Stream Empty: Missed `and` rule: {}", label);
        return None;
    }
    let mut children: Vec<Box<ParseModel>> = Vec::new();
    let offset = token_stream.offset;
    for rule in &rule_struct.children {
        if let Some(next) = match_rule_by_name(rule, token_stream) {
            children.push(Box::new(next));
        } else {
            token_stream.reset(offset);
            println!("Missed `and` rule: {}", label);
            return None;
        }
    }

    println!("Hit `and` rule: {}", label);
    return Some(ParseModel {
        label: label.to_string(),
        tokens: vec![],
        children,
    });
}

fn build_rules() -> Option<HashMap<&'static str, Box<RuleStruct>>> {
    let mut result: HashMap<&'static str, Box<RuleStruct>> = HashMap::new();

    result.insert("function", create_label_match(vec!["_function"]));
    result.insert("open_curly", create_label_match(vec!["_open_curly"]));
    result.insert("close_curly", create_label_match(vec!["_close_curly"]));
    result.insert("open_paren", create_label_match(vec!["_open_paren"]));
    result.insert("close_paren", create_label_match(vec!["_close_paren"]));
    result.insert("period", create_label_match(vec!["_period"]));
    result.insert("comma", create_label_match(vec!["_comma"]));

    result.insert("config_word", create_label_match(vec!["_config"]));
    result.insert("app", create_label_match(vec!["_app"]));
    result.insert("identifier", create_label_match(vec!["_word"]));
    result.insert("string_literal", create_label_match(vec!["_string_literal"]));

    result.insert("expression", create_and_rule_once( vec!["string_literal"])); // todo

    result.insert("param", create_and_rule_once( vec!["expression", "next_param"]));
    result.insert("next_param", create_and_rule(RuleRepeats::ZeroOrMore, vec!["comma", "expression"]));
    result.insert("optional_params", create_and_rule(RuleRepeats::ZeroOrOne, vec!["param"]));

    result.insert("simple_identifier", create_or_rule_once(vec!["identifier", "config_word"])); // todo: primitive types here?
    result.insert("next_identifier", create_and_rule(RuleRepeats::ZeroOrMore, vec!["period", "identifier"])); // todo: maybe exclamation too?
    result.insert("qualified_identifier", create_and_rule_once( vec!["simple_identifier", "next_identifier"]));

    result.insert("function_invocation", create_and_rule_once( vec!["qualified_identifier", "open_paren", "optional_params", "close_paren"]));
    result.insert("simple_statement", create_and_rule_once( vec!["function_invocation"]));
    result.insert("statement", create_or_rule_once(vec!["block", "simple_statement"]));
    result.insert("statements", create_and_rule(RuleRepeats::ZeroOrMore, vec!["statement"]));
    result.insert("block", create_and_rule_once( vec!["open_curly", "statements", "close_curly"]));
    result.insert("app_decl", create_and_rule_once(vec!["app", "function", "identifier", "open_paren", "close_paren", "block"]));
    result.insert("document", create_or_rule_once(vec!["app_decl"]));

    return Some(result);
}

fn create_and_rule_once( children: Vec<&'static str>) -> Box<RuleStruct> {
    Box::new(RuleStruct {
        rule_type: RuleType::And,
        repeat: RuleRepeats::Once,
        children,
        match_labels: vec![],
    })
}

fn create_and_rule(repeat: RuleRepeats, children: Vec<&'static str>) -> Box<RuleStruct> {
    Box::new(RuleStruct {
        rule_type: RuleType::And,
        repeat,
        children,
        match_labels: vec![],
    })
}

#[allow(dead_code)]
fn create_or_rule(repeat: RuleRepeats, children: Vec<&'static str>) -> Box<RuleStruct> {
    Box::new(
        RuleStruct {
            rule_type: RuleType::Or,
            repeat,
            children,
            match_labels: vec![],
        }
    )
}

fn create_or_rule_once(children: Vec<&'static str>) -> Box<RuleStruct> {
    Box::new(
        RuleStruct {
            rule_type: RuleType::Or,
            repeat: RuleRepeats::Once,
            children,
            match_labels: vec![],
        }
    )
}

fn create_label_match(match_labels: Vec<&'static str>) -> Box<RuleStruct> {
    Box::new(
        RuleStruct {
            rule_type: RuleType::Match,
            repeat: RuleRepeats::Once,
            children: vec![],
            match_labels,
        }
    )
}

