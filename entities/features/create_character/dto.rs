use std::borrow::Cow;

#[derive(Clone, serde::Deserialize)]
pub struct CreateCharacterRequest<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub age: i32,
    #[serde(borrow)]
    pub gender: Cow<'a, str>,
    #[serde(borrow)]
    pub description: Cow<'a, str>,
    pub entity_type_id: u64,
    pub project_id: u64,
}
