use crate::ZOD_IMPORT;

mod array;
mod r#enum;
mod instance_type;
mod literal;
mod number;
mod object;
mod schema;
mod schema_object;
mod string;
mod union;

#[cfg(test)]
fn check(schema: String) {
    let schema = format!(r#"{ZOD_IMPORT} {schema}"#);

    let output = std::process::Command::new("node")
        .args(["-e", &schema])
        .output()
        .expect("failed to execute process");

    assert!(
        output.status.success(),
        "Failed to create schema in node:\n{}",
        String::from_utf8_lossy(&output.stderr)
    )
}
