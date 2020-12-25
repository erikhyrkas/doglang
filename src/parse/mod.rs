//parser
// takes the token stream from the lexer and translates it into a structure that it can give
// to the analyzer

use crate::lex::token_stream::TokenStream;
use crate::parse::parse_model::ParseModel;
use crate::parse::parse_rules::match_document;
use crate::UNKNOWN;

mod parse_model;
mod parse_rules;


// In my first attempt at writing a parser in Rust, I tried an OO model like I did with the
// lexer. I've used this pattern in other languages and it has served me well.
//
// After reading this very good post: https://m-decoster.github.io//2017/01/16/fighting-borrowchk/
//
// I threw up my hands, swore a few times, and then started anew.
//
// Here we bump into one of the areas of Rust that is frustrating: circular references
// Because of the way that Rust does memory management, it will do everything in its
// power to prevent a circular reference, and it is very good at it. The problem we
// have is that parsing has some very recursive ideas in it:
//
// A block has statements and statements have blocks.
//
// Of course, there are more situations just like this in parsing. If this were C, we'd have
// functions recursing. In C++ or Java, we'd make an object hierarchy that held the logic.
// In Rust, an object hierarchy won't work without bending the language really far. Instead,
// it is easier to more or less do the C method.
//
// There is a second issue here with globals initialization. Rust does not allow code to
// run before main, which makes it very explict and easy to interpret. This might be fine
// if there was an easy way of declaring a global that was not initialized and then initializing
// it later. Yes, you will google and see the `lazy_static` crate and the crate `phf`, both of
// which look great, however: lazy static didn't solve my issues with trying to use a dynamically
// sized value of Box<dyn Rule>. phf didn't help since my values were logically generated.
//
// This is a great example of why rust is not easy to learn. The design goals were
// (in my words) to create a more-or-less memory safe low-level language with a good
// tool chain. And, for the most part, they did this. However, if you've used C/C++/Java
// those goals are super inconvenient. In this case, we are making a static global list of rules.
// The memory shouldn't get cleared until the program exits, so we don't need to worry
// about whether the reference checker decrements. We also have a good logic model that
// fits the problem, but a language restriction that does not. Yes, circular references
// are super dangerous in C/C++ but you can do them if you are careful. Being careful
// with Rust in this case requires diving into the bowls of the language to break the
// entire language design of being based on reference counters that don't handle circular
// references.
//
// In the end, rather than having rules hold rules, I'll have rules hold the names of other
// rules and look them up in a hashmap. This is not efficient, but spending hours trying to
// figure out how to make the Rc<RefCell<dyn Rule>> solution work properly (it's insane to me)
// is not worth it. The recursive nature of this problem doesn't suit Rust well in a way
// that also performs well and doesn't require unreusable code. The table lookup method is more
// or less creating our own, slower, weak reference.

pub fn parse(mut token_stream: Box<TokenStream>, file_name: Option<&str>, file_path: Option<&str>) -> Option<Box<ParseModel>> {
    let real_file_name = String::from(file_name.unwrap_or(UNKNOWN));
    let real_file_path = String::from(file_path.unwrap_or(UNKNOWN));
    println!("Parsing: {} ({})", real_file_name, real_file_path);

    println!("{:?}", token_stream);

    if let Some(result) = match_document(&mut token_stream) {
        if !token_stream.has_next() {
            return Some(Box::new(result));
        }
        if let Some(last_consumed) = token_stream.last_consumed() {
            println!("Syntax error at token {} \"{}\" found at line {} offset {}.",
                     last_consumed.label, last_consumed.value,
                     last_consumed.line_number, last_consumed.line_offset);
        } else {
            println!("Unknown parsing error."); // improve this message with diagnostic info.
        }
    }

    println!("Parsing failed for: {} ({})", real_file_name, real_file_path);
    return None;
}