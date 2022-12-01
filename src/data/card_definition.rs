use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CardDefinition {
    display_name: String,

    #[serde(default)]
    description: Option<String>,

    /// Informations "IRL", pas utilisées dans le jeu
    #[serde(default)]
    info: Option<String>,

    icon: Option<String>,

    #[serde(default)]
    category: CardCategory,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardCategory {
    #[default]
    Uncategorized,
    Building,
    Prevention,
    TreatmentRecipe,
    Treatment,
}
