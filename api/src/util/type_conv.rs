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

/// Parses an RFC 3339 timestamp into the shape the timestamp columns use.
///
/// The input carries an offset, the column does not, so the instant is
/// normalised to UTC before the offset is dropped. Storing the wall clock time
/// as written would otherwise silently shift anything sent from a non UTC
/// offset.
pub fn option_string_to_option_datetime(
    input: Option<String>,
) -> Result<Option<jiff::civil::DateTime>, jiff::Error> {
    input
        .map(|s| {
            s.parse::<jiff::Timestamp>()
                .map(|ts| ts.to_zoned(jiff::tz::TimeZone::UTC).datetime())
        })
        .transpose()
}

pub fn option_u32_to_option_i32(input: Option<u32>) -> Option<i32> {
    input.and_then(|u| i32::try_from(u).ok())
}

/// Renders a stored timestamp as RFC 3339, which is what the API hands back.
///
/// The column holds no offset because every write normalises to UTC first, so
/// reattaching UTC here recovers the instant rather than inventing one.
pub fn datetime_to_rfc3339(value: jiff::civil::DateTime) -> String {
    value
        .to_zoned(jiff::tz::TimeZone::UTC)
        .map(|zoned| zoned.timestamp().to_string())
        .unwrap_or_default()
}

/// Unwraps a stored JSON column into the plain value the DTOs carry.
pub fn json_column_to_value(
    value: Option<toasty::stmt::Json<serde_json::Value>>,
) -> Option<serde_json::Value> {
    value.map(|json| json.0)
}
