use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(ToSchema, IntoParams, Deserialize, Serialize, Validate)]
#[into_params(parameter_in = Query)]
pub struct BibleQuery {
    #[validate(length(max = 100, message = "Language must be at most 100 characters"))]
    pub language: Option<String>,
    #[validate(length(max = 100, message = "Version must be at most 100 characters"))]
    pub version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bible_query_valid_empty() {
        let query = BibleQuery {
            language: None,
            version: None,
        };
        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_bible_query_valid_short_strings() {
        let query = BibleQuery {
            language: Some("en".to_string()),
            version: Some("KJV".to_string()),
        };
        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_bible_query_valid_max_length() {
        let query = BibleQuery {
            language: Some("a".repeat(100)),
            version: Some("b".repeat(100)),
        };
        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_bible_query_invalid_language_too_long() {
        let query = BibleQuery {
            language: Some("a".repeat(101)),
            version: None,
        };
        let result = query.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("language"));
    }

    #[test]
    fn test_bible_query_invalid_version_too_long() {
        let query = BibleQuery {
            language: None,
            version: Some("a".repeat(101)),
        };
        let result = query.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("version"));
    }
}

