pub fn json_to_value<T: serde::Serialize>(
    data: Option<T>,
) -> Result<Option<serde_json::Value>, serde_json::Error> {
    data.map(|d| serde_json::to_value(d)).transpose()
}

pub fn value_to_json<T: serde::de::DeserializeOwned>(
    value: Option<serde_json::Value>,
) -> Result<Option<T>, serde_json::Error> {
    value.map(|v| serde_json::from_value(v)).transpose()
}

pub fn option_string_to_option_datetime(
    input: Option<String>,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, chrono::ParseError> {
    input
        .map(|s| s.parse::<chrono::DateTime<chrono::Utc>>())
        .transpose()
}

pub fn option_u32_to_option_i32(input: Option<u32>) -> Option<i32> {
    input.and_then(|u| i32::try_from(u).ok())
}
