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
            //print!("Once: ");
            // return
            single_match_with(label, rule_struct, token_stream)
        }
        RuleRepeats::ZeroOrMore => {
            //print!("ZeroOrMore: ");
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
            //print!("OneOrMore: ");
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
            //print!("ZeroOrOne: ");
            let mut children = vec![];
            if let Some(match_option) = single_match_with(label, rule_struct, token_stream) {
                children.push(Box::new(match_option));
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
    println!("Trying `token` rule: {} {}", label, token_stream.offset);

    if !token_stream.has_next() {
        //println!("Missed `token` rule: {}", label);
        return None;
    }

    let mut result = Vec::new();
    let offset = token_stream.offset;
    for token in &rule_struct.match_labels {
        if let Some(next) = token_stream.next() {
            //println!("Look for [{}] in input [{} : {}] {}", token, &next.label, &next.value, &next.skip);
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
        //println!("Missed `token` rule: {}", label);
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
    println!("Trying `or` rule: {} {}", label, token_stream.offset);

    if !token_stream.has_next() {
        //println!("Missed `or` rule: {}", label);
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
    //println!("Missed `or` rule: {}", label);
    return None;
}

unsafe fn and_match_with(label: &str, rule_struct: &RuleStruct, token_stream: &mut TokenStream) -> Option<ParseModel> {
    println!("Trying `and` rule: {} {}", label, token_stream.offset);

    if !token_stream.has_next() {
        //println!("Token Stream Empty: Missed `and` rule: {}", label);
        return None;
    }
    let mut children: Vec<Box<ParseModel>> = Vec::new();
    let offset = token_stream.offset;
    for rule in &rule_struct.children {
        if let Some(next) = match_rule_by_name(rule, token_stream) {
            children.push(Box::new(next));
        } else {
            token_stream.reset(offset);
            //println!("Missed `and` rule: {}", label);
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
    // Rust Parser Rules 0.0.1
    // open_curly: _open_curly
    result.insert("open_curly", create_label_match(vec!["_open_curly"]));
    // close_curly: _close_curly
    result.insert("close_curly", create_label_match(vec!["_close_curly"]));
    // comma: _comma
    result.insert("comma", create_label_match(vec!["_comma"]));
    // equal: _equal
    result.insert("equal", create_label_match(vec!["_equal"]));
    // greater: _greater
    result.insert("greater", create_label_match(vec!["_greater"]));
    // less: _less
    result.insert("less", create_label_match(vec!["_less"]));
    // plus: _plus
    result.insert("plus", create_label_match(vec!["_plus"]));
    // minus: _minus
    result.insert("minus", create_label_match(vec!["_minus"]));
    // star: _star
    result.insert("star", create_label_match(vec!["_star"]));
    // period: _period
    result.insert("period", create_label_match(vec!["_period"]));
    // slash: _slash
    result.insert("slash", create_label_match(vec!["_slash"]));
    // hash: _hash
    result.insert("hash", create_label_match(vec!["_hash"]));
    // open_paren: _open_paren
    result.insert("open_paren", create_label_match(vec!["_open_paren"]));
    // close_paren: _close_paren
    result.insert("close_paren", create_label_match(vec!["_close_paren"]));
    // open_bracket: _open_bracket
    result.insert("open_bracket", create_label_match(vec!["_open_bracket"]));
    // close_bracket: _close_bracket
    result.insert("close_bracket", create_label_match(vec!["_close_bracket"]));
    // exclamation: _exclamation
    result.insert("exclamation", create_label_match(vec!["_exclamation"]));
    // question_mark: _question_mark
    result.insert("question_mark", create_label_match(vec!["_question_mark"]));
    // colon: _colon
    result.insert("colon", create_label_match(vec!["_colon"]));
    // pipe: _pipe
    result.insert("pipe", create_label_match(vec!["_pipe"]));
    // let: _let
    result.insert("let", create_label_match(vec!["_let"]));
    // return: _return
    result.insert("return", create_label_match(vec!["_return"]));
    // if: _if
    result.insert("if", create_label_match(vec!["_if"]));
    // else: _else
    result.insert("else", create_label_match(vec!["_else"]));
    // switch: _switch
    result.insert("switch", create_label_match(vec!["_switch"]));
    // case: _case
    result.insert("case", create_label_match(vec!["_case"]));
    // default: _default
    result.insert("default", create_label_match(vec!["_default"]));
    // fail: _fail
    result.insert("fail", create_label_match(vec!["_fail"]));
    // otherwise: _otherwise
    result.insert("otherwise", create_label_match(vec!["_otherwise"]));
    // for: _for
    result.insert("for", create_label_match(vec!["_for"]));
    // in: _in
    result.insert("in", create_label_match(vec!["_in"]));
    // while: _while
    result.insert("while", create_label_match(vec!["_while"]));
    // break: _break
    result.insert("break", create_label_match(vec!["_break"]));
    // continue: _continue
    result.insert("continue", create_label_match(vec!["_continue"]));
    // with: _with
    result.insert("with", create_label_match(vec!["_with"]));
    // isa: _isa
    result.insert("isa", create_label_match(vec!["_isa"]));
    // app: _app
    result.insert("app", create_label_match(vec!["_app"]));
    // lib: _lib
    result.insert("lib", create_label_match(vec!["_lib"]));
    // ui: _ui
    result.insert("ui", create_label_match(vec!["_ui"]));
    // service: _service
    result.insert("service", create_label_match(vec!["_service"]));
    // test: _test
    result.insert("test", create_label_match(vec!["_test"]));
    // log: _log
    result.insert("log", create_label_match(vec!["_log"]));
    // config: _config
    result.insert("config", create_label_match(vec!["_config"]));
    // function: _function
    result.insert("function", create_label_match(vec!["_function"]));
    // struct: _struct
    result.insert("struct", create_label_match(vec!["_struct"]));
    // enum: _enum
    result.insert("enum", create_label_match(vec!["_enum"]));
    // trait: _trait
    result.insert("trait", create_label_match(vec!["_trait"]));
    // impl: _impl
    result.insert("impl", create_label_match(vec!["_impl"]));
    // attribute: _attribute
    result.insert("attribute", create_label_match(vec!["_attribute"]));
    // self: _self
    result.insert("self", create_label_match(vec!["_self"]));
    // public: _public
    result.insert("public", create_label_match(vec!["_public"]));
    // mutable: _mutable
    result.insert("mutable", create_label_match(vec!["_mutable"]));
    // constant: _constant
    result.insert("constant", create_label_match(vec!["_constant"]));
    // once: _once
    result.insert("once", create_label_match(vec!["_once"]));
    //reference: _reference
    // unsafe: _unsafe
    result.insert("unsafe", create_label_match(vec!["_unsafe"]));
    // use: _use
    result.insert("use", create_label_match(vec!["_use"]));
    // as: _as
    result.insert("as", create_label_match(vec!["_as"]));
    // module: _module
    result.insert("module", create_label_match(vec!["_module"]));
    // unsigned_integer: _unsigned_integer
    result.insert("unsigned_integer", create_label_match(vec!["_unsigned_integer"]));
    // integer: _integer
    result.insert("integer", create_label_match(vec!["_integer"]));
    // float: _float
    result.insert("float", create_label_match(vec!["_float"]));
    // boolean: _boolean
    result.insert("boolean", create_label_match(vec!["_boolean"]));
    // character: _character
    result.insert("character", create_label_match(vec!["_character"]));
    // void: _void
    result.insert("void", create_label_match(vec!["_void"]));
    // false: _false
    result.insert("false", create_label_match(vec!["_false"]));
    // true: _true
    result.insert("true", create_label_match(vec!["_true"]));
    // null: _null
    result.insert("null", create_label_match(vec!["_null"]));
    // f32: _f32
    result.insert("f32", create_label_match(vec!["_f32"]));
    // f64: _f64
    result.insert("f64", create_label_match(vec!["_f64"]));
    // i8: _i8
    result.insert("i8", create_label_match(vec!["_i8"]));
    // i16: _i16
    result.insert("i16", create_label_match(vec!["_i16"]));
    // i32: _i32
    result.insert("i32", create_label_match(vec!["_i32"]));
    // i64: _i64
    result.insert("i64", create_label_match(vec!["_i64"]));
    // u8: _u8
    result.insert("u8", create_label_match(vec!["_u8"]));
    // u16: _u16
    result.insert("u16", create_label_match(vec!["_u16"]));
    // u32: _u32
    result.insert("u32", create_label_match(vec!["_u32"]));
    // u64: _u64
    result.insert("u64", create_label_match(vec!["_u64"]));
    // sql: _sql
    result.insert("sql", create_label_match(vec!["_sql"]));
    // bool_literal: true || false
    result.insert("bool_literal", create_or_rule_once( vec!["true", "false"]));
    // string_literal: _string_literal
    result.insert("string_literal", create_label_match(vec!["_string_literal"]));
    // number_literal: _number_literal
    result.insert("number_literal", create_label_match(vec!["_number_literal"]));
    // identifier: _word
    result.insert("identifier", create_label_match(vec!["_word"]));
    // null: _null
    result.insert("null", create_label_match(vec!["_null"]));
    // literal: string_literal || number_literal || bool_literal || null
    result.insert("literal", create_or_rule_once( vec!["string_literal", "number_literal", "bool_literal", "null"]));
    // external_identifier_tail: (double_colon && identifier)*
    result.insert("external_identifier_tail", create_and_rule(RuleRepeats::ZeroOrMore, vec!["double_colon", "identifier"]));
    // external_identifier: identifier && external_identifier_tail
    result.insert("external_identifier", create_and_rule_once( vec!["identifier", "external_identifier_tail"]));
    // identifier_part: external_identifier || config || string_literal || number_literal || bool_literal
    result.insert("identifier_part", create_or_rule_once( vec!["external_identifier", "config", "string_literal", "number_literal", "bool_literal"]));
    // additional_identifier_part: (period && identifier)*
    result.insert("additional_identifier_part", create_and_rule(RuleRepeats::ZeroOrMore, vec!["period", "identifier"]));
    // qualified_identifier: identifier_part && additional_identifier_part
    result.insert("qualified_identifier", create_and_rule_once( vec!["identifier_part", "additional_identifier_part"]));
    // literal_or_identifier: literal || qualified_identifier
    result.insert("literal_or_identifier", create_or_rule_once( vec!["literal", "qualified_identifier"]));
    // optional_generic_of_decl: (colon && data_type)?
    result.insert("optional_generic_of_decl", create_and_rule(RuleRepeats::ZeroOrOne, vec!["colon", "data_type"]));
    // generics: (external_identifier && optional_generic_of_decl && optional_comma)+
    result.insert("generics", create_and_rule(RuleRepeats::OneOrMore, vec!["external_identifier", "optional_generic_of_decl", "optional_comma"]));
    // optional_generics: (less && generics && greater)?
    result.insert("optional_generics", create_and_rule(RuleRepeats::ZeroOrOne, vec!["less", "generics", "greater"]));
    // user_type_or_generic: external_identifier && optional_generics
    result.insert("user_type_or_generic", create_and_rule_once( vec!["external_identifier", "optional_generics"]));
    // base_data_type: integer || float || boolean || character || user_type_or_generic
    result.insert("base_data_type", create_or_rule_once( vec!["integer", "float", "boolean", "character", "user_type_or_generic"]));
    // array_type: open_bracket && data_type && close_bracket
    result.insert("array_type", create_and_rule_once( vec!["open_bracket", "data_type", "close_bracket"]));
    // data_type: base_data_type || array_type
    result.insert("data_type", create_or_rule_once( vec!["base_data_type", "array_type"]));
    // optional_data_type: (colon && data_type)?
    result.insert("optional_data_type", create_and_rule(RuleRepeats::ZeroOrOne, vec!["colon", "data_type"]));
    // alias: _word
    result.insert("alias", create_label_match(vec!["_word"]));
    // double_colon: colon && colon
    result.insert("double_colon", create_and_rule_once( vec!["colon", "colon"]));
    // optional_comma: comma?
    result.insert("optional_comma", create_and_rule(RuleRepeats::ZeroOrOne, vec!["comma"]));
    // optional_semicolon: semicolon*
    result.insert("optional_semicolon", create_and_rule(RuleRepeats::ZeroOrMore, vec!["semicolon"]));
    // boolean_equals: equal && equal
    result.insert("boolean_equals", create_and_rule_once( vec!["equal", "equal"]));
    // boolean_less: less
    result.insert("boolean_less", create_and_rule_once( vec!["less"]));
    // boolean_greater: greater
    result.insert("boolean_greater", create_and_rule_once( vec!["greater"]));
    // boolean_not_equal: exclamation && equal
    result.insert("boolean_not_equal", create_and_rule_once( vec!["exclamation", "equal"]));
    // boolean_greater_or_equal: greater && equal
    result.insert("boolean_greater_or_equal", create_and_rule_once( vec!["greater", "equal"]));
    // boolean_less_or_equal: less && equal
    result.insert("boolean_less_or_equal", create_and_rule_once( vec!["less", "equal"]));
    // comparison: boolean_equals || boolean_less || boolean_greater || boolean_not_equal || boolean_greater_or_equal ||boolean_less_or_equal
    result.insert("comparison", create_or_rule_once( vec!["boolean_equals", "boolean_less", "boolean_greater", "boolean_not_equal", "boolean_greater_or_equal", "boolean_less_or_equal"]));
    // multiply: star
    result.insert("multiply", create_and_rule_once( vec!["star"]));
    // divide: slash
    result.insert("divide", create_and_rule_once( vec!["slash"]));
    // dereference_instance_member: period
    result.insert("dereference_instance_member", create_and_rule_once( vec!["period"]));
    // dereference_const_member: double_colon
    result.insert("dereference_const_member", create_and_rule_once( vec!["double_colon"]));
    // binary_operator: plus || minus || multiply || divide || dereference_instance_member || dereference_const_member || comparison
    result.insert("binary_operator", create_or_rule_once( vec!["plus", "minus", "multiply", "divide", "dereference_instance_member", "dereference_const_member", "comparison"]));
    // not_operator: exclamation
    result.insert("not_operator", create_and_rule_once( vec!["exclamation"]));
    // minus_operator: minus
    result.insert("minus_operator", create_and_rule_once( vec!["minus"]));
    // unary_operator: exclamation || minus
    result.insert("unary_operator", create_or_rule_once( vec!["exclamation", "minus"]));
    // log_decl: log && open_paren && string_literal && close_paren && optional_semicolon
    result.insert("log_decl", create_and_rule_once( vec!["log", "open_paren", "string_literal", "close_paren", "optional_semicolon"]));
    // attr_metadata: identifier && colon && literal
    result.insert("attr_metadata", create_and_rule_once( vec!["identifier", "colon", "literal"]));
    // optional_attr_metadata_next: (optional_comma && attr_metadata)*
    result.insert("optional_attr_metadata_next", create_and_rule(RuleRepeats::ZeroOrMore, vec!["optional_comma", "attr_metadata"]));
    // optional_attr_metadata: (attr_metadata && optional_attr_metadata_next)?
    result.insert("optional_attr_metadata", create_and_rule(RuleRepeats::ZeroOrOne, vec!["attr_metadata", "optional_attr_metadata_next"]));
    // optional_attr_metadata_group: (open_curly && optional_attr_metadata && close_curly)*
    result.insert("optional_attr_metadata_group", create_and_rule(RuleRepeats::ZeroOrMore, vec!["open_curly", "optional_attr_metadata", "close_curly"]));
    // attr_tag: hash && external_identifier && optional_attr_metadata_group
    result.insert("attr_tag", create_and_rule_once( vec!["hash", "external_identifier", "optional_attr_metadata_group"]));
    // optional_attr_tags: (attr_tag)*
    result.insert("optional_attr_tags", create_and_rule(RuleRepeats::ZeroOrMore, vec!["attr_tag"]));
    // enum_member: identifier
    result.insert("enum_member", create_and_rule_once( vec!["identifier"]));
    // enum_members: (enum_member && optional_comma)*
    result.insert("enum_members", create_and_rule(RuleRepeats::ZeroOrMore, vec!["enum_member", "optional_comma"]));
    // enum_decl: optional_attr_tags && enum && open_curly && enum_members && close_culry
    result.insert("enum_decl", create_and_rule_once( vec!["optional_attr_tags", "enum", "open_curly", "enum_members", "close_culry"]));
    // impl_statement: function_decl
    result.insert("impl_statement", create_and_rule_once( vec!["function_decl"]));
    // impl_body: (optional_const && impl_statement)*
    result.insert("impl_body", create_and_rule(RuleRepeats::ZeroOrMore, vec!["optional_const", "impl_statement"]));
    // on_optional_trait: (on && identifier && optional_generics)?
    result.insert("on_optional_trait", create_and_rule(RuleRepeats::ZeroOrOne, vec!["on", "identifier", "optional_generics"]));
    // impl_decl: optional_attr_tags && impl && identifier && on_optional_trait && open_curly && impl_body && close_curly
    result.insert("impl_decl", create_and_rule_once( vec!["optional_attr_tags", "impl", "identifier", "on_optional_trait", "open_curly", "impl_body", "close_curly"]));
    // optional_const: const?
    result.insert("optional_const", create_and_rule(RuleRepeats::ZeroOrOne, vec!["const"]));
    // trait_statement: function_signature_decl || function_decl
    result.insert("trait_statement", create_or_rule_once( vec!["function_signature_decl", "function_decl"]));
    // trait_body: (optional_const && trait_statement)*
    result.insert("trait_body", create_and_rule(RuleRepeats::ZeroOrMore, vec!["optional_const", "trait_statement"]));
    // trait_decl: optional_attr_tags && identifier && optional_generics && open_curly && trait_body && close_curly
    result.insert("trait_decl", create_and_rule_once( vec!["optional_attr_tags", "identifier", "optional_generics", "open_curly", "trait_body", "close_curly"]));
    // struct_member: identifier && optional_data_type
    result.insert("struct_member", create_and_rule_once( vec!["identifier", "optional_data_type"]));
    // struct_body: (struct_member && optional_semicolon)*
    result.insert("struct_body", create_and_rule(RuleRepeats::ZeroOrMore, vec!["struct_member", "optional_semicolon"]));
    // struct_decl: optional_attr_tags && struct && identifier && optional_generics && open_curly && struct_body && close_curly
    result.insert("struct_decl", create_and_rule_once( vec!["optional_attr_tags", "struct", "identifier", "optional_generics", "open_curly", "struct_body", "close_curly"]));
    // optional_param_qualifier: (identifier && colon)?
    result.insert("optional_param_qualifier", create_and_rule(RuleRepeats::ZeroOrOne, vec!["identifier", "colon"]));
    // params: (optional_param_qualifier && expression && optional_comma)*
    result.insert("params", create_and_rule(RuleRepeats::ZeroOrMore, vec!["optional_param_qualifier", "expression", "optional_comma"]));
    // function_invocation: qualified_identifier && open_paren && params && close_paren
    result.insert("function_invocation", create_and_rule_once( vec!["qualified_identifier", "open_paren", "params", "close_paren"]));
    // fail_invocation: fail && open_paren && params && close_paren
    result.insert("fail_invocation", create_and_rule_once( vec!["fail", "open_paren", "params", "close_paren"]));
    // variable_literal_invocation: function_invocation || literal_or_identifier
    result.insert("variable_literal_invocation", create_or_rule_once( vec!["function_invocation", "literal_or_identifier"]));
    // struct_constructor_list_entry: literal_or_identifier && optional_comma
    result.insert("struct_constructor_list_entry", create_and_rule_once( vec!["literal_or_identifier", "optional_comma"]));
    // struct_constructor_list_entries: struct_constructor_list_entry*
    result.insert("struct_constructor_list_entries", create_and_rule(RuleRepeats::ZeroOrMore, vec!["struct_constructor_list_entry"]));
    // struct_constructor_list: open_bracket && struct_constructor_list_entries && close_bracket
    result.insert("struct_constructor_list", create_and_rule_once( vec!["open_bracket", "struct_constructor_list_entries", "close_bracket"]));
    // struct_constructor_map_entry: identifier && colon && literal_or_identifier && optional_comma
    result.insert("struct_constructor_map_entry", create_and_rule_once( vec!["identifier", "colon", "literal_or_identifier", "optional_comma"]));
    // struct_constructor_map_entries: struct_constructor_map_entry*
    result.insert("struct_constructor_map_entries", create_and_rule(RuleRepeats::ZeroOrMore, vec!["struct_constructor_map_entry"]));
    // struct_constructor_map: open_curly && struct_constructor_map_entries && close_curly
    result.insert("struct_constructor_map", create_and_rule_once( vec!["open_curly", "struct_constructor_map_entries", "close_curly"]));
    // struct_constructor: identifier && struct_constructor_map
    result.insert("struct_constructor", create_and_rule_once( vec!["identifier", "struct_constructor_map"]));
    // optional_config_extention: (colon && identifier)?
    result.insert("optional_config_extention", create_and_rule(RuleRepeats::ZeroOrOne, vec!["colon", "identifier"]));
    // config_decl:  config && identifier && optional_config_extention && config_map
    result.insert("config_decl", create_and_rule_once( vec!["config", "identifier", "optional_config_extention", "config_map"]));
    // config_document: config_decl*
    result.insert("config_document", create_and_rule(RuleRepeats::ZeroOrMore, vec!["config_decl"]));
    // optional_range_inclusive: equal?
    result.insert("optional_range_inclusive", create_and_rule(RuleRepeats::ZeroOrOne, vec!["equal"]));
    // range_expression: open_bracket && literal_or_identifier && period && period && optional_range_inclusive && literal_or_identifier && close_bracket
    result.insert("range_expression", create_and_rule_once( vec!["open_bracket", "literal_or_identifier", "period", "period", "optional_range_inclusive", "literal_or_identifier", "close_bracket"]));
    // binary_operation: variable_literal_invocation && binary_operator && expression
    result.insert("binary_operation", create_and_rule_once( vec!["variable_literal_invocation", "binary_operator", "expression"]));
    // unary_operation: unary_operator && expression
    result.insert("unary_operation", create_and_rule_once( vec!["unary_operator", "expression"]));
    // cast_operation: variable_literal_invocation && as && data_type
    result.insert("cast_operation", create_and_rule_once( vec!["variable_literal_invocation", "as", "data_type"]));
    // expression_group: open_paren && expression && close_paren
    result.insert("expression_group", create_and_rule_once( vec!["open_paren", "expression", "close_paren"]));
    // expression_part: function_invocation || struct_constructor || expression_group || binary_operation || unary_operation || variable_literal_invocation || range_expression
    result.insert("expression_part", create_or_rule_once( vec!["function_invocation", "struct_constructor", "expression_group", "binary_operation", "unary_operation", "variable_literal_invocation", "range_expression"]));
    // trailing_binary_expression_part: (binary_operator && expression)*
    result.insert("trailing_binary_expression_part", create_and_rule(RuleRepeats::ZeroOrMore, vec!["binary_operator", "expression"]));
    // expression: expression_part && trailing_binary_expression_part
    result.insert("expression", create_and_rule_once( vec!["expression_part", "trailing_binary_expression_part"]));
    // optional_expression: expression?
    result.insert("optional_expression", create_and_rule(RuleRepeats::ZeroOrOne, vec!["expression"]));
    // variable_declaration: let && identifier && optional_data_type
    result.insert("variable_declaration", create_and_rule_once( vec!["let", "identifier", "optional_data_type"]));
    // variable_declaration_statement: variable_declaration && optional_semicolon
    result.insert("variable_declaration_statement", create_and_rule_once( vec!["variable_declaration", "optional_semicolon"]));
    // variable_or_variable_declaration: qualified_identifier || variable_declaration
    result.insert("variable_or_variable_declaration", create_or_rule_once( vec!["qualified_identifier", "variable_declaration"]));
    // assignment: variable_or_variable_declaration && equal && expression && optional_semicolon
    result.insert("assignment", create_and_rule_once( vec!["variable_or_variable_declaration", "equal", "expression", "optional_semicolon"]));
    // simple_statement: assignment || expression || variable_declaration_statement
    result.insert("simple_statement", create_or_rule_once( vec!["assignment", "expression", "variable_declaration_statement"]));
    // while_loop_statement: while && optional_expression && block
    result.insert("while_loop_statement", create_and_rule_once( vec!["while", "optional_expression", "block"]));
    // for_loop_statement: for && identifier && optional_data_type && in && expression && block
    result.insert("for_loop_statement", create_and_rule_once( vec!["for", "identifier", "optional_data_type", "in", "expression", "block"]));
    // return_statement: return && expression && optional_semicolon
    result.insert("return_statement", create_and_rule_once( vec!["return", "expression", "optional_semicolon"]));
    // if_statement: if && expression && block
    result.insert("if_statement", create_and_rule_once( vec!["if", "expression", "block"]));
    // otherwise_action: (block || expression || fail_invocation)
    result.insert("otherwise_action", create_or_rule_once( vec!["block", "expression", "fail_invocation"]));
    // optional_otherwise: (otherwise && otherwise_action)?
    result.insert("optional_otherwise", create_and_rule(RuleRepeats::ZeroOrOne, vec!["otherwise", "otherwise_action"]));
    // any_statement: block || return_statement || for_loop_statement || while_loop_statement || simple_statement || if_statement || fail_invocation
    result.insert("any_statement", create_or_rule_once( vec!["block", "return_statement", "for_loop_statement", "while_loop_statement", "simple_statement", "if_statement", "fail_invocation"]));
    // statements: (any_statement && optional_otherwise)*
    result.insert("statements", create_and_rule(RuleRepeats::ZeroOrMore, vec!["any_statement", "optional_otherwise"]));
    // block_no_otherwise: open_curly && statements && close_curly
    result.insert("block_no_otherwise", create_and_rule_once( vec!["open_curly", "statements", "close_curly"]));
    // block: block_no_otherwise && optional_otherwise
    result.insert("block", create_and_rule_once( vec!["block_no_otherwise", "optional_otherwise"]));
    // optional_param_value: (equal && literal)?
    result.insert("optional_param_value", create_and_rule(RuleRepeats::ZeroOrOne, vec!["equal", "literal"]));
    // function_params: (identifier && colon && data_type && optional_param_value && optional_comma)*
    result.insert("function_params", create_and_rule(RuleRepeats::ZeroOrMore, vec!["identifier", "colon", "data_type", "optional_param_value", "optional_comma"]));
    // function_params_group: open_paren && function_params && close_paren
    result.insert("function_params_group", create_and_rule_once( vec!["open_paren", "function_params", "close_paren"]));
    // optional_entry_point_decl: (app || test || lib || service || ui)?
    result.insert("optional_entry_point_decl", create_or_rule(RuleRepeats::ZeroOrOne, vec!["app", "test", "lib", "service", "ui"]));
    // function_name: identifier && optional_generics
    result.insert("function_name", create_and_rule_once( vec!["identifier", "optional_generics"]));
    // entry_or_function_decl: optional_attr_tags && optional_entry_point_decl && function && function_name && function_params_group && block_no_otherwise
    result.insert("entry_or_function_decl", create_and_rule_once( vec!["optional_attr_tags", "optional_entry_point_decl", "function", "function_name", "function_params_group", "block_no_otherwise"]));
    // function_signature_decl: optional_attr_tags && function && function_name && function_params_group
    result.insert("function_signature_decl", create_and_rule_once( vec!["optional_attr_tags", "function", "function_name", "function_params_group"]));
    // function_decl: function_signature_decl && block_no_otherwise
    result.insert("function_decl", create_and_rule_once( vec!["function_signature_decl", "block_no_otherwise"]));
    // attr_base_data_type: integer || float || boolean || character || identifier
    result.insert("attr_base_data_type", create_or_rule_once( vec!["integer", "float", "boolean", "character", "identifier"]));
    // attr_array_type: open_bracket && attr_base_data_type && close_bracket
    result.insert("attr_array_type", create_and_rule_once( vec!["open_bracket", "attr_base_data_type", "close_bracket"]));
    // attr_data_type: attr_base_data_type || attr_array_type
    result.insert("attr_data_type", create_or_rule_once( vec!["attr_base_data_type", "attr_array_type"]));
    // attr_body: (identifier && colon && attr_data_type && optional_semicolon)*
    result.insert("attr_body", create_and_rule(RuleRepeats::ZeroOrMore, vec!["identifier", "colon", "attr_data_type", "optional_semicolon"]));
    // attr_type: module || struct || impl || trait || function || enum || app || ui || service || lib
    result.insert("attr_type", create_or_rule_once( vec!["module", "struct", "impl", "trait", "function", "enum", "app", "ui", "service", "lib"]));
    // attr_types: (attr_type && optional_comma)+
    result.insert("attr_types", create_and_rule(RuleRepeats::OneOrMore, vec!["attr_type", "optional_comma"]));
    // optional_attr_generic_of_decl: (colon && attr_types)?
    result.insert("optional_attr_generic_of_decl", create_and_rule(RuleRepeats::ZeroOrOne, vec!["colon", "attr_types"]));
    // optional_attr_generic_decl: (less && identifier && optional_attr_generic_of_decl && greater)?
    result.insert("optional_attr_generic_decl", create_and_rule(RuleRepeats::ZeroOrOne, vec!["less", "identifier", "optional_attr_generic_of_decl", "greater"]));
    // attr_decl: attribute && identifier && optional_attr_generic_decl && use_when_config_matches_props && open_culry && attr_body && close_curly
    result.insert("attr_decl", create_and_rule_once( vec!["attribute", "identifier", "optional_attr_generic_decl", "use_when_config_matches_props", "open_culry", "attr_body", "close_curly"]));
    // mod_body_decls: (entry_or_function_decl || struct_decl || trait_decl || impl_decl || enum_decl || mod_decl || attr_decl)*
    result.insert("mod_body_decls", create_or_rule(RuleRepeats::ZeroOrMore, vec!["entry_or_function_decl", "struct_decl", "trait_decl", "impl_decl", "enum_decl", "mod_decl", "attr_decl"]));
    // optional_test: test?
    result.insert("optional_test", create_and_rule(RuleRepeats::ZeroOrOne, vec!["test"]));
    // mod_body: use_decls && mod_body_decls
    result.insert("mod_body", create_and_rule_once( vec!["use_decls", "mod_body_decls"]));
    // mod_decl: optional_attr_tags && optional_test && module && identifier && use_when_config_matches_props && open_curly && mod_body && close_curly
    result.insert("mod_decl", create_and_rule_once( vec!["optional_attr_tags", "optional_test", "module", "identifier", "use_when_config_matches_props", "open_curly", "mod_body", "close_curly"]));
    // use_group_part_alias: (as && alias)?
    result.insert("use_group_part_alias", create_and_rule(RuleRepeats::ZeroOrOne, vec!["as", "alias"]));
    // use_group_part_decl: (identifier && use_group_part_alias && optional_comma)+
    result.insert("use_group_part_decl", create_and_rule(RuleRepeats::OneOrMore, vec!["identifier", "use_group_part_alias", "optional_comma"]));
    // use_group_decl: double_colon && open_curly && use_group_part_decl && close_curly
    result.insert("use_group_decl", create_and_rule_once( vec!["double_colon", "open_curly", "use_group_part_decl", "close_curly"]));
    // use_decl_next_part: (double_colon && identifier)*
    result.insert("use_decl_next_part", create_and_rule(RuleRepeats::ZeroOrMore, vec!["double_colon", "identifier"]));
    // use_decl_form_2: use && identifier && use_decl_next_part && use_group_decl && optional_semicolon
    result.insert("use_decl_form_2", create_and_rule_once( vec!["use", "identifier", "use_decl_next_part", "use_group_decl", "optional_semicolon"]));
    // use_decl_form_1: use && identifier && use_decl_next_part && use_group_part_alias && optional_semicolon
    result.insert("use_decl_form_1", create_and_rule_once( vec!["use", "identifier", "use_decl_next_part", "use_group_part_alias", "optional_semicolon"]));
    // use_decls: (use_decl_form_1 || use_decl_form_2)*
    result.insert("use_decls", create_or_rule(RuleRepeats::ZeroOrMore, vec!["use_decl_form_1", "use_decl_form_2"]));
    // use_when_config_matches_prop: (identifier && colon && literal_or_identifier && optional_comma)+
    result.insert("use_when_config_matches_prop", create_and_rule(RuleRepeats::OneOrMore, vec!["identifier", "colon", "literal_or_identifier", "optional_comma"]));
    // use_when_config_matches_props: (open_bracket && use_when_config_matches_prop && close_bracket)?
    result.insert("use_when_config_matches_props", create_and_rule(RuleRepeats::ZeroOrOne, vec!["open_bracket", "use_when_config_matches_prop", "close_bracket"]));
    // mod_decl_next_part: (double_colon && identifier)*
    result.insert("mod_decl_next_part", create_and_rule(RuleRepeats::ZeroOrMore, vec!["double_colon", "identifier"]));
    // mod_name_decl: module && identifier && mod_decl_next_part && use_when_config_matches_props && optional_semicolon
    result.insert("mod_name_decl", create_and_rule_once( vec!["module", "identifier", "mod_decl_next_part", "use_when_config_matches_props", "optional_semicolon"]));
    // optional_mod_name_decl: mod_name_decl?
    result.insert("optional_mod_name_decl", create_and_rule(RuleRepeats::ZeroOrOne, vec!["mod_name_decl"]));
    // module_document: optional_mod_name_decl && mod_body
    result.insert("module_document", create_and_rule_once( vec!["optional_mod_name_decl", "mod_body"]));
    // config_value: literal || config_map || config_list
    result.insert("config_value", create_or_rule_once( vec!["literal", "config_map", "config_list"]));
    // config_list_entry: config_value && optional_comma
    result.insert("config_list_entry", create_and_rule_once( vec!["config_value", "optional_comma"]));
    // config_list_entries: config_list_entry*
    result.insert("config_list_entries", create_and_rule(RuleRepeats::ZeroOrMore, vec!["config_list_entry"]));
    // config_list: open_bracket && config_list_entries && close_bracket
    result.insert("config_list", create_and_rule_once( vec!["open_bracket", "config_list_entries", "close_bracket"]));
    // config_map_entry: identifier && colon && config_value && optional_comma
    result.insert("config_map_entry", create_and_rule_once( vec!["identifier", "colon", "config_value", "optional_comma"]));
    // config_map_entries: config_map_entry*
    result.insert("config_map_entries", create_and_rule(RuleRepeats::ZeroOrMore, vec!["config_map_entry"]));
    // config_map: open_curly && config_map_entries && close_curly
    result.insert("config_map", create_and_rule_once( vec!["open_curly", "config_map_entries", "close_curly"]));
    // optional_config_extention: (colon && identifier)?
    result.insert("optional_config_extention", create_and_rule(RuleRepeats::ZeroOrOne, vec!["colon", "identifier"]));
    // config_decl: config && identifier && optional_config_extention && config_map
    result.insert("config_decl", create_and_rule_once( vec!["config", "identifier", "optional_config_extention", "config_map"]));
    // config_document: config_decl+
    result.insert("config_document", create_and_rule(RuleRepeats::OneOrMore, vec!["config_decl"]));
    // document: config_document || module_document
    result.insert("document", create_or_rule_once( vec!["config_document", "module_document"]));

    return Some(result);
}

fn create_and_rule_once(children: Vec<&'static str>) -> Box<RuleStruct> {
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

