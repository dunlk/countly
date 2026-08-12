use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Student {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}
