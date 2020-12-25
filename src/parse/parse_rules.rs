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


    result.insert("open_curly", create_label_match(vec!["open_curly"]));
    result.insert("close_curly", create_label_match(vec!["close_curly"]));
    result.insert("open_paren", create_label_match(vec!["open_paren"]));
    result.insert("close_paren", create_label_match(vec!["close_paren"]));
    result.insert("period", create_label_match(vec!["period"]));
    result.insert("comma", create_label_match(vec!["comma"]));

    result.insert("config_word", create_label_match(vec!["config"]));
    result.insert("app", create_label_match(vec!["app"]));
    result.insert("identifier", create_label_match(vec!["word"]));
    result.insert("string_literal", create_label_match(vec!["string"]));

    result.insert("expression", create_and_rule(RuleRepeats::Once, vec!["string_literal"])); // todo

    result.insert("param", create_and_rule(RuleRepeats::Once, vec!["expression", "next_param"]));
    result.insert("next_param", create_and_rule(RuleRepeats::ZeroOrMore, vec!["comma", "expression"]));
    result.insert("optional_params", create_and_rule(RuleRepeats::ZeroOrOne, vec!["param"]));

    result.insert("simple_identifier", create_or_rule(vec!["identifier", "config_word"])); // todo: primitive types here?
    result.insert("next_identifier", create_and_rule(RuleRepeats::ZeroOrMore, vec!["period", "identifier"])); // todo: maybe exclamation too?
    result.insert("qualified_identifier", create_and_rule(RuleRepeats::Once, vec!["simple_identifier", "next_identifier"]));

    result.insert("function_invocation", create_and_rule(RuleRepeats::Once, vec!["qualified_identifier", "open_paren", "optional_params", "close_paren"]));
    result.insert("simple_statement", create_and_rule(RuleRepeats::Once, vec!["function_invocation"]));
    result.insert("statement", create_or_rule(vec!["block", "simple_statement"]));
    result.insert("statements", create_and_rule(RuleRepeats::ZeroOrMore, vec!["statement"]));
    result.insert("block", create_and_rule(RuleRepeats::Once, vec!["open_curly", "statements", "close_curly"]));
    result.insert("app_decl", create_and_rule(RuleRepeats::Once, vec!["app", "identifier", "open_paren", "close_paren", "block"]));
    result.insert("document", create_or_rule(vec!["app_decl"]));

    return Some(result);
}

fn create_and_rule(repeat: RuleRepeats, children: Vec<&'static str>) -> Box<RuleStruct> {
    Box::new(RuleStruct {
        rule_type: RuleType::And,
        repeat,
        children,
        match_labels: vec![],
    })
}

fn create_or_rule(children: Vec<&'static str>) -> Box<RuleStruct> {
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


/*
struct PrimeRule {
    rules: HashMap<&'static str, &'static Box<RuleStruct>>,
}

impl PrimeRule {
    pub fn get_prime_rule(&self) -> Option<&'static Box<dyn Rule>> {
        return RULES.get("document");
    }

    pub fn get_rule(&self, name: &str) -> Option<&'static Box<dyn Rule>> {
        return RULES.get(name);
    }

    fn build_rules() -> HashMap<&'static str, &'static Box<dyn Rule>> {
        let block_rules: Vec<Rc<RefCell<dyn Rule>>> = vec![];
        let block = Rc::new(RefCell::new(AndRule {
            label: "block".to_string(),
            rules: block_rules,
        }));

        let open_paren = create_token_rule("open_paren");
        let close_paren = create_token_rule("close_paren");
        let open_curly = create_token_rule("open_curly");
        let close_curly = create_token_rule("close_curly");

        let application_word = create_token_rule("app");

        let identifier = create_token_rule_with_tokens("identifier", vec!["word".to_string()]);


        let mut block_ref = &block;

        let statement: Rc<RefCell<dyn Rule>> = Rc::new(RefCell::new(AndRule {
            label: "statement".to_string(),
            rules: vec![
                Rc::new(RefCell::new(OrRule {
                    rules: vec![
                        block.clone()
                    ]
                }))
            ],
        }));

        block_ref.borrow().rules.push(open_curly);
        block_ref.borrow().rules.push(
            Rc::new(RefCell::new(RepeatRule {
                label: "statements".to_string(),
                rule: statement,
                minimum: 0,
            })));
        block_ref.borrow().rules.push(close_curly);


        let application_rule = Rc::new(RefCell::new(AndRule {
            label: "app".to_string(),
            rules: vec![application_word, identifier, open_paren, close_paren, block],
        }));

        let prime_rule = Some(Rc::new(RefCell::new(AndRule {
            label: "document".to_string(),
            rules: vec![
                Rc::new(RefCell::new(OrRule {
                    rules: vec![
                        application_rule
                    ]
                }))
            ],
        })));
        println!("Prime Rule created");
        return prime_rule;
    }

    fn create_token_rule(label: &str) -> Rc<RefCell<TokenRule>> {
        return Rc::new(RefCell::new(TokenRule {
            label: label.to_string(),
            token_matcher: Box::new(LabelTokenMatcher {
                tokens: vec![label.to_string()]
            }),
        }));
    }

    fn create_token_rule_with_tokens(label: &str, tokens: Vec<String>) -> Rc<RefCell<TokenRule>> {
        return Rc::new(RefCell::new(TokenRule {
            label: label.to_string(),
            token_matcher: Box::new(LabelTokenMatcher {
                tokens
            }),
        }));
    }
}
*/