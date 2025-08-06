use schemars::{
    schema::{InstanceType, Schema, SchemaObject, SingleOrVec},
    Set,
};

use crate::{Error, ParserInner, ParserResult};

impl ParserInner {
    /// Check if the union has one shared literal
    /// key
    pub fn has_discriminated(&self, variants: &Vec<Schema>) -> Option<String> {
        let variants_object = variants
            .iter()
            .filter_map(|schema| {
                if let Schema::Object(schema) = schema {
                    if let Some(SingleOrVec::Single(instance_type)) =
                        &schema.instance_type
                    {
                        if *instance_type.as_ref() == InstanceType::Object {
                            if let Some(schema) = &schema.object {
                                return Some(schema);
                            }
                        }
                    }
                }

                None
            })
            .collect::<Vec<_>>();

        if !variants_object.is_empty() && variants_object.len() == variants.len() {
            let literal_keys = variants_object
                .iter()
                .map(|v| {
                    v.properties
                        .iter()
                        .filter(|(_, schema)| {
                            if let Schema::Object(schema) = schema {
                                self.is_literal(&schema)
                            } else {
                                false
                            }
                        })
                        .map(|(key, _)| key.as_str())
                        .collect::<Set<&str>>()
                })
                .reduce(|mut keys1, keys2| {
                    keys1.retain(|key| keys2.contains(key));
                    keys1
                });

            if let Some(keys) = literal_keys {
                if keys.len() == 1 {
                    return Some(keys.first().unwrap().to_string());
                }
            }
        };
        None
    }

    /// Check if the object is a union and
    /// `parse_union` is safe to call
    pub fn is_union(&self, object: &SchemaObject) -> bool { object.subschemas.is_some() }

    /// Parse a union
    pub fn parse_union(&self, object: &SchemaObject) -> ParserResult {
        let subschemas = object.subschemas.as_ref().unwrap();

        let variants = subschemas.one_of.as_ref().map(|v| Ok(v)).unwrap_or_else(
            || match &subschemas.any_of {
                None => {
                    #[cfg(test)]
                    dbg!(object);
                    Err(Error::Unimplemented(
                        "Union: subschemas are only supported with any_of or one_of",
                    ))
                },
                Some(v) => Ok(v),
            },
        )?;

        if variants.is_empty() {
            return Ok(String::from("z.any()"));
        }
        if let [only] = variants.as_slice() {
            return self.parse_schema(&only);
        }

        let discriminated_key = self.has_discriminated(variants);

        let mut union_values = Vec::with_capacity(variants.len());
        for schema in variants {
            union_values.push(self.parse_schema(schema)?);
        }

        let mut union_parsed = match discriminated_key {
            Some(key) => format!(
                "z.discriminatedUnion({}, [{}])",
                serde_json::to_string(&key)?,
                union_values.join(", "),
            ),
            None => format!("z.union([{}])", union_values.join(", ")),
        };

        if object.object.is_some() {
            let and = self.parse_object(object)?;

            union_parsed = format!("z.intersection({union_parsed}, {and})")
        }

        Ok(union_parsed)
    }
}

#[cfg(test)]
mod tests {
    use schemars::{schema::Schema, JsonSchema};

    use crate::{test_helpers::generator, Parser};

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "kind")]
    enum TestSchema2 {
        JustTheName,
        NameAndSingleValue(i32),
        NameAndObject { prop: String, int: i32 },
    }

    #[test]
    fn test_tagged_union() {
        let schema = generator().into_root_schema_for::<TestSchema2>();
        let schema = Schema::Object(schema.schema);

        let parser = Parser::default();
        let result = parser.parse_pretty_default(&schema).unwrap();

        // std::fs::write("tests/tagged-union.js",
        // result).expect("Could not save
        // result");
        assert_eq!(include_str!("../../tests/tagged-union.js"), &result);
        crate::parsers::check(result);
    }

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "kind", content = "value")]
    enum TestSchema3 {
        JustTheName,
        NameAndSingleValue(i32),
        NameAndTuple(String, String, String),
        NameAndObject { prop: String, int: i32 },
    }

    #[test]
    fn test_double_tagged_union() {
        let schema = generator().into_root_schema_for::<TestSchema3>();
        let schema = Schema::Object(schema.schema);

        let parser = Parser::default();
        let result = parser.parse_pretty_default(&schema).unwrap();

        assert_eq!(include_str!("../../tests/double-tagged-union.js"), &result);
        crate::parsers::check(result);
        // std::fs::write("tests/
        // double-tagged-union.js",
        // result).expect("Could not save
        // result");
    }

    #[derive(JsonSchema)]
    #[allow(dead_code)]
    #[serde(rename_all = "camelCase")]
    #[serde(untagged)]
    enum TestSchema {
        Option1 { prop: String, int: i32 },
        Option2 { prop: i32, name: String },
    }

    #[test]
    fn test_untagged_union() {
        let schema = generator().into_root_schema_for::<TestSchema>();
        let schema = Schema::Object(schema.schema);

        let parser = Parser::default();
        let result = parser.parse_pretty_default(&schema).unwrap();

        assert_eq!(include_str!("../../tests/untagged-union.js"), &result);
        crate::parsers::check(result);
        // std::fs::write("tests/untagged-union.
        // js", result).expect("Could not save
        // result");
    }
}
