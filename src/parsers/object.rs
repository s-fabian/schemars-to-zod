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
            let schema_parsed = self.parse_schema(schema)?;

            let default = if let Schema::Object(schema) = schema {
                if let Some(metadata) = &schema.metadata {
                    metadata.default.as_ref()
                } else {
                    None
                }
            } else {
                None
            };

            let schema_parsed = if let Some(default) = default {
                format!(
                    "z.default({}, {})",
                    schema_parsed,
                    serde_json::to_string(default)?
                )
            } else if !options.required.contains(key) && !self.config.ignore_undefined {
                if schema_parsed.starts_with("z.nullable(")
                    && schema_parsed.ends_with(")")
                {
                    let schema_parsed = schema_parsed
                        .strip_prefix("z.nullable(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap();
                    format!("z.nullish({})", schema_parsed)
                } else {
                    format!("z.optional({})", schema_parsed)
                }
            } else {
                schema_parsed
            };

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

        let object_parsed = if options
            .additional_properties
            .as_ref()
            .is_some_and(|p| p.as_ref() == &Schema::Bool(true))
        {
            object_inner_parsed.map(|p| format!("z.strictObject({{ {} }})", p))
        } else {
            object_inner_parsed.map(|p| format!("z.object({{ {} }})", p))
        };

        let object_parsed = if let Some(additional) = &options.additional_properties {
            if additional.as_ref() != &Schema::Bool(false) {
                let additional_parsed = self.parse_schema(additional)?;

                if let Some(object_parsed) = object_parsed {
                    format!("z.catchall({}, {})", object_parsed, additional_parsed)
                } else {
                    format!("z.record({})", additional_parsed)
                }
            } else if let Some(object_parsed) = object_parsed {
                object_parsed
            } else {
                return Err(Error::Unimplemented(
                    "Object: additional_properties are false, and there are no \
                     properties given",
                ));
            }
        } else {
            object_parsed.unwrap_or_else(|| String::from("z.looseObject({})"))
        };

        Ok(object_parsed)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::NaiveDate;
    use schemars::{JsonSchema, schema::Schema};
    use uuid::Uuid;

    use crate::{Parser, test_helpers::generator};

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
        assert_eq!(include_str!("../../tests/object.js"), &result);
        crate::parsers::check(result);
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
        assert_eq!(include_str!("../../tests/record.js"), &result);
        crate::parsers::check(result);
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
        assert_eq!(include_str!("../../tests/flatten.js"), &result);
        crate::parsers::check(result);
    }
}
