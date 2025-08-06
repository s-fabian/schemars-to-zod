use schemars::schema::{SchemaObject, SingleOrVec};

use crate::{Error, ParserInner, ParserResult};

impl ParserInner {
    /// Parse an array
    pub fn parse_array(&self, object: &SchemaObject) -> ParserResult {
        let options_default = Default::default();
        let options = object.array.as_ref().unwrap_or(&options_default);

        if options.contains.is_some() || options.unique_items.is_some_and(|b| b) {
            return Err(Error::Unimplemented(
                "Array: contains and unique_items are unimplemented",
            ));
        }

        let array_parsed = if let Some(items) = &options.items {
            match items {
                SingleOrVec::Single(schema) => {
                    let mut res = format!("z.array({})", self.parse_schema(&*schema)?);
                    let mut checks = Vec::new();

                    if let Some(min_items) = options.min_items {
                        checks.push(format!("z.minLength({min_items})"));
                    }

                    if let Some(max_items) = options.max_items {
                        checks.push(format!("z.maxLength({max_items})"));
                    }

                    if !checks.is_empty() {
                        res.push_str(&format!(".check({})", checks.join(", ")));
                    }

                    res
                },
                SingleOrVec::Vec(schemas) => {
                    let mut schemas_parsed = Vec::with_capacity(schemas.len());

                    for schema in schemas {
                        schemas_parsed.push(self.parse_schema(&schema)?);
                    }

                    if let Some(additional) = &options.additional_items {
                        let rest = self.parse_schema(&*additional)?;

                        format!("z.tuple([{}], {})", schemas_parsed.join(", "), rest)
                    } else {
                        format!("z.tuple([{}])", schemas_parsed.join(", "))
                    }
                },
            }
        } else {
            String::from("z.array(z.never())")
        };

        Ok(array_parsed)
    }
}

#[cfg(test)]
mod tests {
    use schemars::{JsonSchema, schema::Schema};

    use crate::{Parser, test_helpers::generator};

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    struct TestSchema {
        admin: bool,
        age: Option<i32>,
    }

    #[allow(dead_code)]
    type TestType = Vec<TestSchema>;

    #[test]
    fn test_array() {
        let schema = generator().into_root_schema_for::<TestType>();
        let schema = Schema::Object(schema.schema);

        let parser = Parser::default();
        let result = parser.parse_pretty_default(&schema).unwrap();

        // std::fs::write("tests/array.js",
        // result).expect("Could not save
        // result");
        assert_eq!(include_str!("../../tests/array.js"), &result);
        crate::parsers::check(result);
    }
}
