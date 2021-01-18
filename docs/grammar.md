# Grammar

A rust grammar in a simple format that is easy to read.

* Lexer rules are defined with leading underscores.
* Parser rules are defined without leading underscores.
* C-style comments (// and /* */)
* [] text within a bracket reprents the valid characters in this position, so ["] reprents a quote. Where [0-9] reprents
  the range of 0 to 9 inclusive. [a-zA-Z0-9] covers upper and lower case letters plus digits
* && is an "and" rule
* || is an "or" rule
* ? is optional
* * is 0 or more
* + is 1 or more
* parenthesis are for clarity when repeats happen, since we can't combine the logic ands/ors they don't do anything else

Because this parser implementation doesn't support an OO approach and relies on lookups to work around Rust limits, you
cannot combin && and || in a single declaration, so they are being split up. This is annoying, but what we are working
with for now. It's probably too much to hope that we'll change this prior to dog self-compiling and being able to
represent this in a cleaner fashion.

Parser note: Because the or/and/repeat parser features only work on parser tokens, you'll see the lexer tokens be mapped
to parser matchers. This is functionally fine, but seems redundant to the reader and possibly a little annoying to the
person transforming the parser if they need to pull out the original token text, which really only happens for literals
-- so it mostly doesn't matter.

Rust Parser Rules

```
// Rust Parser Rules 0.0.1

open_curly: _open_curly
close_curly: _close_curly
comma: _comma
equal: _equal
greater: _greater
less: _less
plus: _plus
minus: _minus
star: _star
period: _period
slash: _slash
hash: _hash
open_paren: _open_paren
close_paren: _close_paren
open_bracket: _open_bracket
close_bracket: _close_bracket
exclamation: _exclamation
question_mark: _question_mark
colon: _colon
pipe: _pipe

let: _let
return: _return
if: _if
else: _else
switch: _switch
case: _case
default: _default
fail: _fail
otherwise: _otherwise
for: _for
in: _in
while: _while
break: _break
continue: _continue
with: _with 
isa: _isa
app: _app
lib: _lib
ui: _ui 
service: _service
test: _test
log: _log
config: _config
function: _function
struct: _struct
enum: _enum
trait: _trait
impl: _impl
attribute: _attribute
self: _self
public: _public
mutable: _mutable
constant: _constant
once: _once
//reference: _reference
unsafe: _unsafe
use: _use
as: _as
module: _module
unsigned_integer: _unsigned_integer
integer: _integer
float: _float
boolean: _boolean
character: _character
void: _void
false: _false
true: _true
null: _null
f32: _f32
f64: _f64
i8: _i8
i16: _i16
i32: _i32
i64: _i64
u8: _u8
u16: _u16
u32: _u32
u64: _u64

sql: _sql
bool_literal: true || false
string_literal: _string_literal
number_literal: _number_literal
identifier: _word
null: _null
literal: string_literal || number_literal || bool_literal || null

external_identifier_tail: (double_colon && identifier)*
external_identifier: identifier && external_identifier_tail
identifier_part: external_identifier || config || string_literal || number_literal || bool_literal
additional_identifier_part: (period && identifier)*
qualified_identifier: identifier_part && additional_identifier_part
literal_or_identifier: literal || qualified_identifier

optional_generic_of_decl: (colon && data_type)?
generics: (external_identifier && optional_generic_of_decl && optional_comma)+
optional_generics: (less && generics && greater)?
user_type_or_generic: external_identifier && optional_generics
base_data_type: integer || float || boolean || character || user_type_or_generic
array_type: open_bracket && data_type && close_bracket
data_type: base_data_type || array_type
optional_data_type: (colon && data_type)?

alias: _word

double_colon: colon && colon

optional_comma: comma?
optional_semicolon: semicolon*

boolean_equals: equal && equal
boolean_less: less
boolean_greater: greater
boolean_not_equal: exclamation && equal
boolean_greater_or_equal: greater && equal
boolean_less_or_equal: less && equal
comparison: boolean_equals || boolean_less || boolean_greater || boolean_not_equal || boolean_greater_or_equal ||boolean_less_or_equal
multiply: star
divide: slash
dereference_instance_member: period
dereference_const_member: double_colon
binary_operator: plus || minus || multiply || divide || dereference_instance_member || dereference_const_member || comparison

not_operator: exclamation
minus_operator: minus
unary_operator: exclamation || minus

log_decl: log && open_paren && string_literal && close_paren && optional_semicolon

attr_metadata: identifier && colon && literal
optional_attr_metadata_next: (optional_comma && attr_metadata)*
optional_attr_metadata: (attr_metadata && optional_attr_metadata_next)?
optional_attr_metadata_group: (open_curly && optional_attr_metadata && close_curly)*
attr_tag: hash && external_identifier && optional_attr_metadata_group
optional_attr_tags: (attr_tag)* 

enum_member: identifier
enum_members: (enum_member && optional_comma)*
enum_decl: optional_attr_tags && enum && open_curly && enum_members && close_culry

impl_statement: function_decl
impl_body: (optional_const && impl_statement)*
on_optional_trait: (on && identifier && optional_generics)?
impl_decl: optional_attr_tags && impl && identifier && on_optional_trait && open_curly && impl_body && close_curly

optional_const: const?
trait_statement: function_signature_decl || function_decl
trait_body: (optional_const && trait_statement)*
trait_decl: optional_attr_tags && identifier && optional_generics && open_curly && trait_body && close_curly 

struct_member: identifier && optional_data_type 
struct_body: (struct_member && optional_semicolon)*
struct_decl: optional_attr_tags && struct && identifier && optional_generics && open_curly && struct_body && close_curly

optional_param_qualifier: (identifier && colon)?
params: (optional_param_qualifier && expression && optional_comma)*
function_invocation: qualified_identifier && open_paren && params && close_paren

fail_invocation: fail && open_paren && params && close_paren

variable_literal_invocation: function_invocation || literal_or_identifier

struct_constructor_list_entry: literal_or_identifier && optional_comma
struct_constructor_list_entries: struct_constructor_list_entry*
struct_constructor_list: open_bracket && struct_constructor_list_entries && close_bracket
struct_constructor_map_entry: identifier && colon && literal_or_identifier && optional_comma
struct_constructor_map_entries: struct_constructor_map_entry*
struct_constructor_map: open_curly && struct_constructor_map_entries && close_curly
struct_constructor: identifier && struct_constructor_map

optional_config_extention: (colon && identifier)? 
config_decl:  config && identifier && optional_config_extention && config_map
config_document: config_decl*

optional_range_inclusive: equal?
range_expression: open_bracket && literal_or_identifier && period && period && optional_range_inclusive && literal_or_identifier && close_bracket

binary_operation: variable_literal_invocation && binary_operator && expression
unary_operation: unary_operator && expression
cast_operation: variable_literal_invocation && as && data_type 
expression_group: open_paren && expression && close_paren
expression_part: function_invocation || struct_constructor || expression_group || binary_operation || unary_operation || variable_literal_invocation || range_expression 
trailing_binary_expression_part: (binary_operator && expression)*
expression: expression_part && trailing_binary_expression_part
optional_expression: expression?

variable_declaration: let && identifier && optional_data_type 
variable_declaration_statement: variable_declaration && optional_semicolon

variable_or_variable_declaration: qualified_identifier || variable_declaration
assignment: variable_or_variable_declaration && equal && expression && optional_semicolon

simple_statement: assignment || expression || variable_declaration_statement

while_loop_statement: while && optional_expression && block 

for_loop_statement: for && identifier && optional_data_type && in && expression && block

return_statement: return && expression && optional_semicolon

if_statement: if && expression && block

otherwise_action: (block || expression || fail_invocation)
optional_otherwise: (otherwise && otherwise_action)?

any_statement: block || return_statement || for_loop_statement || while_loop_statement || simple_statement || if_statement || fail_invocation
statements: (any_statement && optional_otherwise)*
block_no_otherwise: open_curly && statements && close_curly
block: block_no_otherwise && optional_otherwise

optional_param_value: (equal && literal)?
function_params: (identifier && colon && data_type && optional_param_value && optional_comma)* 
function_params_group: open_paren && function_params && close_paren

optional_entry_point_decl: (app || test || lib || service || ui)?  
function_name: identifier && optional_generics
entry_or_function_decl: optional_attr_tags && optional_entry_point_decl && function && function_name && function_params_group && block_no_otherwise
function_signature_decl: optional_attr_tags && function && function_name && function_params_group
function_decl: function_signature_decl && block_no_otherwise

attr_base_data_type: integer || float || boolean || character || identifier
attr_array_type: open_bracket && attr_base_data_type && close_bracket
attr_data_type: attr_base_data_type || attr_array_type
attr_body: (identifier && colon && attr_data_type && optional_semicolon)*
attr_type: module || struct || impl || trait || function || enum || app || ui || service || lib
attr_types: (attr_type && optional_comma)+
optional_attr_generic_of_decl: (colon && attr_types)?
optional_attr_generic_decl: (less && identifier && optional_attr_generic_of_decl && greater)?

attr_decl: attribute && identifier && optional_attr_generic_decl && use_when_config_matches_props && open_culry && attr_body && close_curly

mod_body_decls: (entry_or_function_decl || struct_decl || trait_decl || impl_decl || enum_decl || mod_decl || attr_decl)*

optional_test: test?
mod_body: use_decls && mod_body_decls 
mod_decl: optional_attr_tags && optional_test && module && identifier && use_when_config_matches_props && open_curly && mod_body && close_curly

use_group_part_alias: (as && alias)?
use_group_part_decl: (identifier && use_group_part_alias && optional_comma)+
use_group_decl: double_colon && open_curly && use_group_part_decl && close_curly
use_decl_next_part: (double_colon && identifier)*
use_decl_form_2: use && identifier && use_decl_next_part && use_group_decl && optional_semicolon
use_decl_form_1: use && identifier && use_decl_next_part && use_group_part_alias && optional_semicolon
use_decls: (use_decl_form_1 || use_decl_form_2)*

use_when_config_matches_prop: (identifier && colon && literal_or_identifier && optional_comma)+
use_when_config_matches_props: (open_bracket && use_when_config_matches_prop && close_bracket)?

mod_decl_next_part: (double_colon && identifier)*
mod_name_decl: module && identifier && mod_decl_next_part && use_when_config_matches_props && optional_semicolon
optional_mod_name_decl: mod_name_decl?

module_document: optional_mod_name_decl && mod_body

config_value: literal || config_map || config_list
config_list_entry: config_value && optional_comma
config_list_entries: config_list_entry*
config_list: open_bracket && config_list_entries && close_bracket
config_map_entry: identifier && colon && config_value && optional_comma
config_map_entries: config_map_entry*
config_map: open_curly && config_map_entries && close_curly
optional_config_extention: (colon && identifier)? 
config_decl: config && identifier && optional_config_extention && config_map
config_document: config_decl+

document: config_document || module_document 

```

Rust Lexer Rules

```
// Rust Lexer Rules 0.0.1

_let: 'let'
_return: 'return'
_if: 'if'
_else: 'else'
_switch: 'switch'
_case: 'case'
_default: 'default'
_fail: 'fail'
_otherwise: 'otherwise'
_for: 'for'
_in: 'in'
_while: 'while'
_break: 'break'
_continue: 'continue'
_with: 'with' 
_isa: 'isa'
_app: 'app'
_lib: 'lib'
_ui: 'ui' 
_service: 'service'
_test: 'test'
_log: 'log'
_config: 'config'
_function: 'fn'
_struct: 'struct'
_enum: 'enum'
_trait: 'trait'
_impl: 'impl'
_attribute: 'attr'
_self: 'self'
_public: 'pub'
_mutable: 'mut'
_constant: 'const'
_once: 'once'
//_reference: 'ref'
_unsafe: 'unsafe'
_use: 'use'
_as: 'as'
_module: 'mod'
_unsigned_integer: 'uint'
_integer: 'int'
_float: 'float'
_boolean: 'bool'
_character: 'char'
_void: 'void'
_false: 'false'
_true: 'true'
_null: 'null'
_f32: 'f32'
_f64: 'f64'
_i8: 'i8'
_i16: 'i16'
_i32: 'i32'
_i64: 'i64'
_u8: 'u8'
_u16: 'u16'
_u32: 'u32'
_u64: 'u64'

_sql: ([`] && (.* || [\] && [`])* && [`]) 
_comment: ( '//' && .* && '\n') || ('/*' && .* && '*/') -> skip
_string_literal: (["] && (.* || [\] && ["])* && ["]) || (['] && (.* || [\] && ['])* && ['])
_number_literal: ('0x' && [0-9a-z-A-Z]+) || ([0-9]+ && '.' && [0-9]+) || ([0-9]+)
_word: [a-zA-Z] && [a-zA-Z0-9_]*

_open_curly: '{'
_close_curly: '}'
_comma: ','
_equal: '='
_greater: '>'
_less: '<'
_plus: '+'
_minus: '-'
_star: '*'
_period: '.'
_slash: '/'
_hash: '#'
_open_paren: '('
_close_paren: ')'
_open_bracket: '['
_close_bracket: ']'
_exclamation: '!'
_question_mark: '?'
_colon: ':'
_pipe: '|'
_end_of_line: '\n' -> skip
_whitespace: (' ' || '\t' || '\r')+ -> skip

```