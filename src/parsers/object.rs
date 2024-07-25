use std::collections::BTreeMap;

use schemars::schema::{Schema, SchemaObject};

use crate::{Error, ParserInner, ParserResult};

impl ParserInner {
    /// Parse a object
    pub fn parse_object(&self, object: &SchemaObject) -> ParserResult {
        let options_default = Default::default();
        let options = object.object.as_ref().unwrap_or(&options_default);

        let mut properties_parsed = BTreeMap::new();

        if options.min_properties.is_some()
            || options.max_properties.is_some()
            || !options.pattern_properties.is_empty()
            || options.property_names.is_some()
        {
            return Err(Error::Unimplemented(
                "Object: keys min_properties, max_properties, pattern_properties and \
                 property_names are not supported",
            ));
        }

        for (key, schema) in &options.properties {
            let mut schema_parsed = self.parse_schema(schema)?;

            let default = if let Schema::Object(schema) = schema {
                if let Some(metadata) = &schema.metadata {
                    metadata.default.as_ref()
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(default) = default {
                schema_parsed
                    .push_str(&format!(".default({})", serde_json::to_string(default)?));
            } else if !options.required.contains(key) && !self.config.ignore_undefined {
                schema_parsed.push_str(".optional()");
            }

            properties_parsed.insert(key.to_owned(), schema_parsed);
        }

        let mut object_inner = Vec::with_capacity(properties_parsed.len());

        for (k, v) in properties_parsed {
            let k = serde_json::to_string(&k)?;

            object_inner.push(format!("{k}: {v}"));
        }

        let object_inner_parsed = if !object_inner.is_empty() {
            Some(object_inner.join(", "))
        } else {
            None
        };

        let object_parsed = object_inner_parsed.map(|p| format!("z.object({{ {} }})", p));

        let object_parsed = if let Some(additional) = &options.additional_properties {
            if additional.as_ref() != &Schema::Bool(false) {
                let additional_parsed = self.parse_schema(additional)?;

                if let Some(mut object_parsed) = object_parsed {
                    object_parsed.push_str(&format!(".catchall({})", additional_parsed));

                    object_parsed
                } else {
                    format!("z.record({})", additional_parsed)
                }
            } else if let Some(object_parsed) = object_parsed {
                format!("{}.strict()", object_parsed)
            } else {
                return Err(Error::Unimplemented(
                    "Object: additional_properties are false, and there are no \
                     properties given",
                ));
            }
        } else {
            object_parsed.unwrap_or_else(|| String::from("z.object({})"))
        };

        Ok(object_parsed)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::NaiveDate;
    use schemars::{schema::Schema, JsonSchema};
    use uuid::Uuid;

    use crate::{test_helpers::generator, Parser};

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    #[serde(rename_all = "camelCase")]
    struct TestSchema {
        user_id: Uuid,
        #[serde(flatten)]
        other: HashMap<String, String>,
    }

    #[test]
    fn test_object() {
        let schema = generator().into_root_schema_for::<TestSchema>();
        let schema = Schema::Object(schema.schema);

        let parser = Parser::default();
        let result = parser.parse_pretty_default(&schema).unwrap();

        // std::fs::write("tests/object.js",
        // result).expect("Could not save
        // result");
        assert_eq!(&result, include_str!("../../tests/object.js"));
    }

    type TestType = HashMap<String, TestSchema>;

    #[test]
    fn test_object_2() {
        let schema = generator().into_root_schema_for::<TestType>();
        let schema = Schema::Object(schema.schema);

        let parser = Parser::default();
        let result = parser.parse_pretty_default(&schema).unwrap();

        // std::fs::write("tests/record.js",
        // result).expect("Could not save
        // result");
        assert_eq!(&result, include_str!("../../tests/record.js"));
    }

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    struct TestSchema2 {
        max: Option<u8>,
        date: Option<NaiveDate>,
        #[serde(default)]
        #[serde(flatten)]
        range: TestSchema3,
        #[serde(flatten)]
        range3: TestSchema5,
    }

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    struct TestSchema3 {
        a: i32,
    }

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    #[serde(tag = "kind")]
    enum TestSchema5 {
        Option1 { key: String },
        Option2 { key2: u8 },
    }

    #[test]
    fn test_object_3() {
        let schema = generator().into_root_schema_for::<TestSchema2>();
        let schema = Schema::Object(schema.schema);

        let parser = Parser::default();
        let result = parser.parse_pretty_default(&schema).unwrap();

        // eprintln!("{}",
        // serde_json::to_string(&schema).
        // unwrap());

        // std::fs::write("tests/flatten.js",
        // result).expect("Could not save
        // result");
        assert_eq!(&result, include_str!("../../tests/flatten.js"));
    }
}
