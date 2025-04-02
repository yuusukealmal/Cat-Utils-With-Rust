use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};

pub fn indent_json(
    data: &serde_json::Map<String, serde_json::Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    let formatter = PrettyFormatter::with_indent(b"    ");

    let mut buf = Vec::new();

    let mut ser = Serializer::with_formatter(&mut buf, formatter);

    data.serialize(&mut ser)?;

    Ok(String::from_utf8(buf)?)
}
