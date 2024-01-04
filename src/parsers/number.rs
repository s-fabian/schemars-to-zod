use schemars::schema::SchemaObject;

use crate::{ParserInner, ParserResult};

impl ParserInner {
    /// Parse a number / integer
    pub fn parse_number(&self, is_int: bool, object: &SchemaObject) -> ParserResult {
        let mut res = if is_int {
            String::from("z.number().int()")
        } else {
            String::from("z.number()")
        };

        let options_default = Default::default();
        let options = object.number.as_ref().unwrap_or(&options_default);

        if let Some(multiple_of) = options.multiple_of {
            res.push_str(&format!(".multipleOf({})", multiple_of));
        }

        if let Some(val) = options.minimum {
            if self.config.explicit_min_max {
                res.push_str(&format!(".gte({})", val));
            } else {
                res.push_str(&format!(".min({})", val));
            }
        }

        if let Some(val) = options.exclusive_minimum {
            res.push_str(&format!(".gt({})", val));
        }

        if let Some(val) = options.maximum {
            if self.config.explicit_min_max {
                res.push_str(&format!(".lte({})", val));
            } else {
                res.push_str(&format!(".max({})", val));
            }
        }

        if let Some(val) = options.exclusive_maximum {
            res.push_str(&format!(".lt({})", val));
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use schemars::{schema::Schema, JsonSchema};

    use crate::{test_helpers::generator, Parser};

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    struct TestSchema {
        distance: f64,
        age: i32,
    }

    #[test]
    fn test_number() {
        let schema = generator().into_root_schema_for::<TestSchema>();
        let schema = Schema::Object(schema.schema);

        let parser = Parser::default();
        let result = parser.parse_pretty_default(&schema).unwrap();

        // std::fs::write("tests/number.js",
        // result).expect("Could not save
        // result");
        assert_eq!(&result, include_str!("../../tests/number.js"));
    }
}
