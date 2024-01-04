# schemars-zod

## Example

```rust
use schemars::{JsonSchema, schema_for, schema::Schema};
use schemars_zod::{Config, Parser};

#[derive(JsonSchema)]
struct MyStruct {
    name: String,
    age: u8,
}

fn main() {
    let schema = schema_for!(MyStruct);
    let schema = Schema::Object(schema.schema);

    let parser = Parser::new(Config {
        // useful when generating a js client
        // where a date would otherwise get parsed as a string
        use_coerce_date: true,
        array_wrapper: false,
        explicit_min_max: false,
        add_descriptions: false,
        union_first: true,
        add_default: false,
    });
    // with the feature pretty
    let result = parser.parse_pretty_default(&schema).unwrap();
    // without the feature pretty
    let result = parser.parse(&schema).unwrap();
    
    println!("{}", result);
}
```

## NOTE

After I made this library, I realised there is already a library with the same name.
You can find the original library [here](https://github.com/audiocloud/schemars-zod).
This library is __not__ [the one on crates.io](https://crates.io/crates/schemars-zod).

Also, this library is heavily inspired by the [node package](https://github.com/StefanTerdell/json-schema-to-zod).
