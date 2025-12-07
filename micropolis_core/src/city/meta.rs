use quick_xml::{events::Event, Reader};
use std::str;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CityMetadata {
    title: String,
    description: String,
    save_file_name: String,
    is_read_only: bool,
}

const XML_REQUIRED_PROPERTIES_NAMES: [&str; 4] =
    ["title", "description", "saveFileName", "readOnly"];

impl CityMetadata {
    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn get_description(&self) -> &String {
        &self.description
    }
    pub fn get_save_file_name(&self) -> &String {
        &self.save_file_name
    }
    pub fn get_is_read_only(&self) -> bool {
        self.is_read_only
    }

    /// Try and decode an XML-encoded city metadata string.
    ///
    /// We manually use quick-xml for quick & dirty parsing since serde
    /// has no first-party support for XML and the third-party crate
    /// based on xml-rs apparently would need a nested struct declaration
    /// for every property since they are value-encoded in micropolis.
    ///
    /// TODO: now quick-xml integrates with serde's Serialize/Deserialize, can something be done here?
    pub fn decode_from_xml(raw: &str) -> Result<Self, String> {
        let mut reader = Reader::from_str(raw);
        let mut in_container_tag = false;
        let mut current_property = Option::<String>::None;
        let mut missing_properties = XML_REQUIRED_PROPERTIES_NAMES.to_vec();
        let mut parsed = CityMetadata::default();

        // reader.trim_text(true);
        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) => {
                    let name = str::from_utf8(e.name().into_inner())
                        .map_err(|err| format!("from_utf8 error: {}", err))?;
                    println!("name {}", name);
                    match name {
                        "metaCity" => {
                            in_container_tag = match in_container_tag {
                                true => return Err("duplicate 'metaCity' container tag".into()),
                                false => true,
                            }
                        }
                        property if missing_properties.contains(&name) => {
                            current_property = Some(property.into());
                            missing_properties.retain(|p| p != &property);
                        }
                        duplicate if XML_REQUIRED_PROPERTIES_NAMES.contains(&name) => {
                            return Err(format!("duplicate property tag '{}'", duplicate))
                        }
                        _ => return Err(format!("unknown property tag '{}'", name)),
                    }
                }
                Ok(Event::Text(e)) => {
                    let value_cow = e
                        .decode()
                        .map_err(|err| format!("text event decoding error: {}", err))?;
                    let value = value_cow.trim();
                    if value.is_empty() {
                        continue;
                    }
                    println!("value {}", value);
                    if let Some(property) = current_property.as_ref() {
                        match property.as_str() {
                            "title" => parsed.title = value.to_string(),
                            "description" => parsed.description = value.to_string(),
                            "saveFileName" => parsed.save_file_name = value.to_string(),
                            "readOnly" if value == "true" => parsed.is_read_only = true,
                            "readOnly" if value == "false" => parsed.is_read_only = false,
                            "readOnly" => return Err(format!("invalid boolean value '{}'", value)),
                            _ => unreachable!(),
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(why) => {
                    return Err(format!(
                        "parsing error at position {}: {:?}",
                        reader.buffer_position(),
                        why
                    ))
                }
                _ => {}
            };
        }

        match missing_properties.len() {
            0 => Ok(parsed),
            n => Err(format!(
                "missing {} properties): {}",
                n,
                missing_properties.join(", ")
            )),
        }
    }

    // Encode the current city metadata state as XML.
    //
    // Given the simplicity of the excepted data structure and the need
    // for proper indentation, we can do it manually here with basic
    // string formatting.
    fn encode_to_xml(&self) -> String {
        // TODO: fix ugly as hell formatting due to indentation
        format!(
            r#"<metaCity>
    <title>{}</title>
    <description>{}</description>
    <saveFileName>{}</saveFileName>
    <readOnly>{}</readOnly>
</metaCity>"#,
            self.title,
            self.description,
            self.save_file_name,
            if self.is_read_only { "true" } else { "false" }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CityMetadata;

    #[test]
    fn test_city_metadata_deserialization() {
        let meta_xml = r#"
            <metaCity>
                <title>Wet City</title>
                <description>The city of "Wet City".</description>
                <saveFileName>wetcity.cty</saveFileName>
                <readOnly>true</readOnly>
            </metaCity>
        "#;
        let result = CityMetadata::decode_from_xml(meta_xml);
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            CityMetadata {
                title: "Wet City".into(),
                description: "The city of \"Wet City\".".into(),
                save_file_name: "wetcity.cty".into(),
                is_read_only: true,
            }
        )
    }

    #[test]
    fn test_city_metadata_serialization() {
        let metadata = CityMetadata {
            title: "Happy Isle".into(),
            description: "The city of \"Happy Isle\".".into(),
            save_file_name: "happisle.cty".into(),
            is_read_only: true,
        };
        let expected = r#"<metaCity>
    <title>Happy Isle</title>
    <description>The city of "Happy Isle".</description>
    <saveFileName>happisle.cty</saveFileName>
    <readOnly>true</readOnly>
</metaCity>"#;
        assert_eq!(metadata.encode_to_xml(), expected);
    }
}
