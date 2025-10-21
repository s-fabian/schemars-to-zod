use schemars::schema::Schema;

use crate::{ParserInner, ParserResult};

impl ParserInner {
    /// Parse a `Schema`
    pub fn parse_schema(&self, schema: &Schema) -> ParserResult {
        Ok(match schema {
            Schema::Bool(bool) if *bool =>
                if self.config.prefer_unknown {
                    String::from("z.unknown()")
                } else {
                    String::from("z.any()")
                },
            Schema::Bool(bool) if !*bool => String::from("z.never()"),
            Schema::Object(object) => self.parse_schema_object(object)?,
            _ => unreachable!(),
        })
    }
}
