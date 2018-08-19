#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: Option<i32>,
    pub given_name: String,
    pub family_name: String
}
