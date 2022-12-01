use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CardDefinition {
    display_name: String,
    icon: Option<String>,

    #[serde(default)]
    color: [u8; 3],
}
