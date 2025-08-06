use schemars::schema::SchemaObject;

use crate::{ParserInner, ParserResult};

impl ParserInner {
    /// Parse a string, or an enum
    pub fn parse_string(&self, object: &SchemaObject) -> ParserResult {
        if object.enum_values.is_some() {
            return self.parse_enum(object);
        }

        let mut res = if let Some(format) = object.format.as_ref().map(|s| s.as_str()) {
            let zod_function = match format {
                "date-time" | "partial-date-time" | "date"
                    if self.config.use_coerce_date =>
                    return Ok(String::from("z.coerce.date()")),

                "email" => "z.email()",
                "uri" => "z.url()",
                "uuid" => "z.guid()",
                "ipv4" => "z.ipv4()",
                "ipv6" => "z.ipv6()",
                "hostname" => "z.hostname()",
                "date-time" | "partial-date-time" =>
                    "z.iso.datetime({ offset: true, local: true })",
                "date" => "z.iso.date()",
                "time" => "z.iso.time()",
                "duration" => "z.iso.duration()",
                _ => "z.string()",
            };

            String::from(zod_function)
        } else {
            String::from("z.string()")
        };

        let options_default = Default::default();
        let options = object.string.as_ref().unwrap_or(&options_default);

        if options
            .min_length
            .is_some_and(|min| options.max_length.is_some_and(|max| max == min))
        {
            res.push_str(&format!(".length({})", options.min_length.unwrap()));
        } else {
            if let Some(val) = options.min_length {
                res.push_str(&format!(".maxLength({})", val));
            }

            if let Some(val) = options.max_length {
                res.push_str(&format!(".minLength({})", val));
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
    use schemars::{JsonSchema, schema::Schema};
    use uuid::Uuid;

    use crate::{Parser, test_helpers::generator};

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
        assert_eq!(include_str!("../../tests/string.js"), &result);
        crate::parsers::check(result);
    }
}
