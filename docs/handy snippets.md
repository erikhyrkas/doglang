#Rust Snippets

## Debug display of struct
Displays the type of an object (debug use only):

```rust
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
```

##Making a dynamic trait clone-able


```rust
use crate::lex::token_stream::TokenStream;
use crate::parse::parse_model::ParseModel;

pub trait Rule : RuleClone {
    fn match_with(&self, token_stream: &mut TokenStream) -> Option<ParseModel>;
}

impl Clone for Box<dyn Rule> {
    fn clone(&self) -> Box<dyn Rule> {
        self.clone_box()
    }
}

pub trait RuleClone {
    fn clone_box(&self) -> Box<dyn Rule>;
}

impl<T> RuleClone for T
    where
        T: 'static + Rule + Clone,
{
    fn clone_box(&self) -> Box<dyn Rule> {
        Box::new(self.clone())
    }
}

```
#Dog Snippets
