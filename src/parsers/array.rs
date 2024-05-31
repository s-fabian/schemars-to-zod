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

        let mut array_parsed = if let Some(items) = &options.items {
            match items {
                SingleOrVec::Single(schema) =>
                    if self.config.array_wrapper {
                        format!("z.array({})", self.parse_schema(&*schema)?)
                    } else {
                        format!("{}.array()", self.parse_schema(&*schema)?)
                    },
                SingleOrVec::Vec(schemas) => {
                    let mut schemas_parsed = Vec::with_capacity(schemas.len());

                    for schema in schemas {
                        schemas_parsed.push(self.parse_schema(&schema)?);
                    }

                    let mut tuple_parsed =
                        format!("z.tuple([{}])", schemas_parsed.join(", "));

                    if let Some(additional) = &options.additional_items {
                        let additional_parsed = self.parse_schema(&*additional)?;

                        tuple_parsed.push_str(&format!(".rest({})", additional_parsed))
                    }

                    tuple_parsed
                },
            }
        } else {
            if self.config.array_wrapper {
                String::from("z.array(z.never())")
            } else {
                String::from("z.never().array()")
            }
        };

        if !matches!(options.items, Some(SingleOrVec::Vec(_))) {
            if let Some(val) = options.min_items {
                array_parsed.push_str(&format!(".min({})", val));
            }

            if let Some(val) = options.max_items {
                array_parsed.push_str(&format!(".max({})", val));
            }
        }

        Ok(array_parsed)
    }
}

#[cfg(test)]
mod tests {
    use schemars::{schema::Schema, JsonSchema};

    use crate::{test_helpers::generator, Parser};

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
        assert_eq!(&result, include_str!("../../tests/array.js"));
    }
}
