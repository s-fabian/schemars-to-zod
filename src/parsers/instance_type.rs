use schemars::schema::{InstanceType, SchemaObject};

use crate::{ParserInner, ParserResult};

impl ParserInner {
    /// Find the correct parser for an instance
    /// type
    pub fn match_instance_type(
        &self,
        instance_type: InstanceType,
        object: &SchemaObject,
    ) -> ParserResult {
        Ok(match instance_type {
            InstanceType::Null => String::from("z.null()"),
            InstanceType::Boolean => String::from("z.boolean()"),
            InstanceType::Number => self.parse_number(false, object)?,
            InstanceType::Integer => self.parse_number(true, object)?,
            InstanceType::String => self.parse_string(object)?,
            InstanceType::Object => self.parse_object(object)?,
            InstanceType::Array => self.parse_array(object)?,
        })
    }
}
