#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct User {
    id: uuid::Uuid,
    #[builder(setter(into))]
    first_name: String,
    #[builder(setter(into))] // Into<String> が実装されてるものもいれられる
    last_name: String,
    #[builder(setter(into), default)]
    // defaultつけておけば指定されなければdefault::Defaultがよばれる
    age: Option<u16>,
}
