
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Literal<T> {
    pub value: T
}
