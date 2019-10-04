use std::fs::File;
use std::io::{BufRead, BufReader};

use super::ParsedMessagesResource;

pub fn parse_messages_resource(
    messages_count: usize,
    reader: BufReader<File>,
) -> Result<ParsedMessagesResource, String> {
    let mut parsed = ParsedMessagesResource::new();
    for (i, line) in reader.lines().enumerate() {
        let trimmed = line
            .map(|l| l.trim().to_string())
            .map_err(|err| format!("reading error at line {}: {}", i + 1, err))?;
        if trimmed.is_empty() || &trimmed[..] == "x" {
            continue;
        }
        parsed.push(trimmed);
    }
    if parsed.len() == messages_count {
        Ok(parsed)
    } else {
        Err(format!(
            "expected {} message items, found {}",
            messages_count,
            parsed.len()
        ))
    }
}
