use schemars::schema::SchemaObject;

use crate::{Error, ParserInner, ParserResult};

impl ParserInner {
    /// Check if the object is a literal and
    /// `parse_literal` is safe to call
    pub fn is_literal(&self, object: &SchemaObject) -> bool {
        object.const_value.is_some()
            || object.enum_values.as_ref().is_some_and(|v| v.len() == 1)
    }

    /// Parse a literal
    pub fn parse_literal(&self, object: &SchemaObject) -> ParserResult {
        Ok(if let Some(literal) = &object.const_value {
            format!("z.literal({})", serde_json::to_string(literal)?)
        } else if let Some([only]) = object.enum_values.as_ref().map(|v| v.as_slice()) {
            format!("z.literal({})", serde_json::to_string(only)?)
        } else {
            return Err(Error::ForgotCheck(
                "Literal: has to have the const_value property or the enum_values \
                 property with only one value",
            ));
        })
    }
}
