use crate::dto::error::ValidationError;

pub fn validate_url(url: &str, field_name: Option<&str>) -> Result<(), ValidationError> {
    // Basic validation for URL format. Not RFC compliant but serves as a simple check.
    if !url.starts_with("http") || url.split(":").count() != 2 || url.split("//").count() != 2 {
        return Err(ValidationError {
            field: String::from(field_name.unwrap_or("url")),
            message: String::from("Invalid URL format"),
        });
    }
    Ok(())
}
