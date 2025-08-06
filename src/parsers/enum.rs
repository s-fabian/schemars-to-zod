use schemars::schema::SchemaObject;

use crate::{Error, ParserInner, ParserResult};

impl ParserInner {
    /// Parse an enum
    pub fn parse_enum(&self, object: &SchemaObject) -> ParserResult {
        let Some(enum_values) = &object.enum_values else {
            return Err(Error::ForgotCheck(
                "Enum: parse_enum requires property enum_values",
            ));
        };

        Ok(if let [only] = enum_values.as_slice() {
            format!("z.literal({})", serde_json::to_string(only)?)
        } else if enum_values.len() != 0 {
            let mut converted = Vec::with_capacity(enum_values.len());

            for value in enum_values {
                converted.push(serde_json::to_string(value)?);
            }

            if !converted.is_empty() {
                format!("z.enum([{}])", converted.join(", "))
            } else {
                String::from("z.string()")
            }
        } else {
            String::from("z.never()")
        })
    }
}

#[cfg(test)]
mod tests {
    use schemars::{JsonSchema, schema::Schema};

    use crate::{Parser, test_helpers::generator};

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    #[serde(rename_all = "camelCase")]
    enum TestEnum {
        Message,
        Leave,
        Walk,
        BuyATV,
    }

    #[test]
    fn test_enum() {
        let schema = generator().into_root_schema_for::<TestEnum>();
        let schema = Schema::Object(schema.schema);

        let parser = Parser::default();
        let result = parser.parse_pretty_default(&schema).unwrap();

        // std::fs::write("tests/enum.js",
        // result).expect("Could not save
        // result");
        assert_eq!(include_str!("../../tests/enum.js"), &result);
        crate::parsers::check(result);
    }
}
