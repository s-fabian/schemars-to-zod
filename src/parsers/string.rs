use schemars::schema::SchemaObject;

use crate::{ParserInner, ParserResult};

impl ParserInner {
    /// Parse a string, or an enum
    pub fn parse_string(&self, object: &SchemaObject) -> ParserResult {
        if object.enum_values.is_some() {
            return self.parse_enum(object);
        }

        let mut res = String::from("z.string()");

        if let Some(format) = object.format.as_ref().map(|s| s.as_str()) {
            let zod_function = match format {
                "email" => "email",
                "uri" => "url",
                "uuid" => "uuid",
                "date-time" | "partial-date-time" | "date"
                    if self.config.use_coerce_date =>
                    return Ok(String::from("z.coerce.date()")),
                "date-time" | "partial-date-time" | "date"
                    if !self.config.use_coerce_date =>
                    return Ok(String::from("z.date()")),
                _ => return Ok(res),
            };

            res.push_str(&format!(".{}()", zod_function));
        }

        let options_default = Default::default();
        let options = object.string.as_ref().unwrap_or(&options_default);

        if options
            .min_length
            .is_some_and(|min| options.max_length.is_some_and(|max| max == min))
        {
            res.push_str(&format!(".length({})", options.min_length.unwrap()));
        } else {
            if let Some(val) = options.min_length {
                res.push_str(&format!(".min({})", val));
            }

            if let Some(val) = options.max_length {
                res.push_str(&format!(".max({})", val));
            }
        }

        if let Some(pattern) = &options.pattern {
            res.push_str(&format!(
                ".regex(new RegExp({}))",
                serde_json::to_string(&pattern)?
            ));
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime};
    use schemars::{schema::Schema, JsonSchema};
    use uuid::Uuid;

    use crate::{test_helpers::generator, Parser};

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    #[serde(rename_all = "camelCase")]
    struct TestSchema {
        user_id: Uuid,
        created_at: NaiveDateTime,
        birthday: NaiveDate,
        name: String,
    }

    #[test]
    fn test_string() {
        let schema = generator().into_root_schema_for::<TestSchema>();
        let schema = Schema::Object(schema.schema);

        let parser = Parser::default();
        let result = parser.parse_pretty_default(&schema).unwrap();

        // std::fs::write("tests/string.js",
        // result).expect("Could not save
        // result");
        assert_eq!(&result, include_str!("../../tests/string.js"));
    }
}
