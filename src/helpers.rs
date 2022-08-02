use mungos::Serialize;

pub fn enum_as_string<T: Serialize>(e: &T) -> String {
    serde_json::to_string(e).unwrap().replace("\"", "")
}
