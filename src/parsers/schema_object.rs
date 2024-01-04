use schemars::schema::{InstanceType, SchemaObject, SingleOrVec};

use crate::{Error, ParserInner, ParserResult};

impl ParserInner {
    /// Parse a `SchemaObject`
    pub fn parse_schema_object(&self, object: &SchemaObject) -> ParserResult {
        if self.is_union(&object) {
            return self.parse_union(&object);
        } else if self.is_literal(object) {
            return self.parse_literal(&object);
        }

        let Some(r#type) = &object.instance_type else {
            #[cfg(test)]
            dbg!(object);
            return Err(Error::Unimplemented(
                "Schema: a schema has to be a union, a literal or have an instance_type",
            ));
        };

        let description = if self.config.add_descriptions {
            object
                .metadata
                .as_ref()
                .and_then(|m| m.description.as_ref())
        } else {
            None
        };

        Ok(match r#type {
            SingleOrVec::Single(instance_type) =>
                if let Some(description) = description {
                    format!(
                        "{}.describe({})",
                        self.match_instance_type(**instance_type, object)?,
                        serde_json::to_string(&description)?
                    )
                } else {
                    self.match_instance_type(**instance_type, object)?
                },
            SingleOrVec::Vec(instance_types) => {
                let null_filtered: Vec<&InstanceType> = instance_types
                    .iter()
                    .filter(|t| !matches!(*t, InstanceType::Null))
                    .collect();
                let is_nullable = instance_types.len() > null_filtered.len();

                if let [instance_type] = instance_types.as_slice() {
                    self.match_instance_type(*instance_type, object)?
                } else if is_nullable && null_filtered.len() == 1 {
                    let [instance_type] = null_filtered.as_slice() else {
                        unreachable!()
                    };

                    if let Some(description) = description {
                        format!(
                            "{}.nullable().describe({})",
                            self.match_instance_type(**instance_type, object)?,
                            serde_json::to_string(&description)?
                        )
                    } else {
                        format!(
                            "{}.nullable()",
                            self.match_instance_type(**instance_type, object)?
                        )
                    }
                } else {
                    let mut parsed = Vec::with_capacity(null_filtered.len());
                    for instance_type in null_filtered {
                        parsed.push(self.match_instance_type(*instance_type, object)?);
                    }

                    if let Some(description) = description {
                        format!(
                            "z.union([{}]){}.describe({})",
                            parsed.join(", "),
                            if is_nullable { ".nullable()" } else { "" },
                            serde_json::to_string(description)?
                        )
                    } else {
                        format!(
                            "z.union([{}]){}",
                            parsed.join(", "),
                            if is_nullable { ".nullable()" } else { "" }
                        )
                    }
                }
            },
        })
    }
}
