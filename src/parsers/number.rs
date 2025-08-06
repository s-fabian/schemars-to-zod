use schemars::schema::SchemaObject;

use crate::{ParserInner, ParserResult};

impl ParserInner {
    /// Parse a number / integer
    pub fn parse_number(&self, is_int: bool, object: &SchemaObject) -> ParserResult {
        let options_default = Default::default();
        let options = object.number.as_ref().unwrap_or(&options_default);

        let mut res = if is_int {
            if options.minimum.is_some_and(|val| val == 0.) {
                String::from("z.uint32()")
            } else {
                String::from("z.int32()")
            }
        } else {
            String::from("z.float64()")
        };

        let mut checks = Vec::new();

        if let Some(multiple_of) = options.multiple_of {
            checks.push(format!("z.step({multiple_of})"));
        }

        if let Some(val) = options.minimum
            && (val != 0. || !is_int)
        {
            checks.push(format!("z.minimum({val})"));
        }

        if let Some(val) = options.exclusive_minimum {
            checks.push(format!("z.gt({val})"));
        }

        if let Some(val) = options.maximum {
            checks.push(format!("z.maximum({val})"));
        }

        if let Some(val) = options.exclusive_maximum {
            checks.push(format!("z.lt({val})"));
        }

        if !checks.is_empty() {
            res.push_str(&format!(".check({})", checks.join(", ")));
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use schemars::{JsonSchema, schema::Schema};

    use crate::{Parser, test_helpers::generator};

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
        assert_eq!(include_str!("../../tests/number.js"), &result);
        crate::parsers::check(result);
    }
}
