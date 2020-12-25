use crate::lex::token_stream::Match;

pub struct ParseModel {
    pub label: String,
    pub tokens: Vec<Box<Match>>,
    pub children: Vec<Box<ParseModel>>,
}

impl ParseModel {
    #[allow(dead_code)]
    fn get_children(&self, name: &str) -> Vec<&Box<ParseModel>> {
        let result = self.children.iter()
            .filter(|model| model.label.eq(name))
            .collect();

        return result;
    }

    #[allow(dead_code)]
    fn get_child(&self, name: &str) -> Option<&ParseModel> {
        if let Some(result) = self.get_children(name).first() {
            return Some(*result);
        }

        return None;
    }

    #[allow(dead_code)]
    fn as_text(&self) -> Option<String> {
        if self.children.is_empty() || self.tokens.is_empty() {
            return None;
        }
        let mut result: String = String::new();
        let mut line_number = 0;
        let mut line_offset = 0;
        for token in &self.tokens {
            if token.line_number > line_number {
                line_number = token.line_number;
                line_offset = token.line_offset;
            } else {
                let target = token.line_offset;
                if line_offset < target {
                    result += " ";
                }
                line_offset = target;
            }
            result += token.value.as_str();
            line_offset += token.length;
        }
        for child in &self.children {
            if let Some(child_text) = child.as_text() {
                result += " ";
                result += child_text.as_str();
            }
        }
        return Some(result);
    }
}

#[cfg(test)]
mod parse_model_tests {
    use crate::lex::lex;
    use crate::parse::parse_model::ParseModel;

    #[test]
    fn test_children_1() {
        let code = lex("Hello there,", None, None).unwrap();
        let code2 = lex("world!", None, None).unwrap();

        let pm = ParseModel {
            label: "my label".to_string(),
            tokens: code.matches,
            children: vec![
                Box::new(ParseModel {
                    label: "other label".to_string(),
                    tokens: code2.matches,
                    children: vec![
                        Box::new(ParseModel {
                            label: "nothing".to_string(),
                            tokens: vec![],
                            children: vec![],
                        })
                    ],
                }),
                Box::new(ParseModel {
                    label: "other label".to_string(),
                    tokens: vec![],
                    children: vec![],
                }),
                Box::new(ParseModel {
                    label: "other label".to_string(),
                    tokens: vec![],
                    children: vec![],
                })
            ],
        };

        // ParseModel isn't realistic, so the output is just as unrealistic. We're just proving that
        // it does what we expect.
        assert_eq!("my label", pm.label);
        assert_eq!("Hello there, world!", pm.as_text().unwrap());
        let child = pm.get_child("other label").unwrap();
        assert_eq!("other label", child.label);
        assert_eq!("world!", child.as_text().unwrap());
        assert!(pm.get_child("garbage").is_none());
        assert_eq!(3, pm.get_children("other label").len());
    }
}