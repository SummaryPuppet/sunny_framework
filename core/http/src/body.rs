/// Body of http
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Body {
    /// Json format
    JSON(String),

    /// XML format
    XML(String),

    /// HTML format
    HTML(String),

    /// For other formats
    NONE,
}

/// Convert a string in Body enum
pub fn from_string_to_body(string: String) -> Body {
    let string = string.trim().to_string();

    if string.starts_with("<html>") {
        Body::HTML(string)
    } else if string.starts_with("{") {
        Body::JSON(string)
    } else if string.starts_with("<") {
        Body::XML(string)
    } else {
        Body::NONE
    }
}
