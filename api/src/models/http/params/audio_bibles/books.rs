use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(ToSchema, IntoParams, Deserialize, Serialize, Validate)]
pub struct BookPath {
    #[validate(range(min = 1, message = "Audio Bible ID must be at least 1"))]
    pub audio_bible_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_book_path_valid() {
        let path = BookPath { audio_bible_id: 1 };
        assert!(path.validate().is_ok());
    }

    #[test]
    fn test_audio_book_path_valid_large_number() {
        let path = BookPath {
            audio_bible_id: 1000,
        };
        assert!(path.validate().is_ok());
    }

    #[test]
    fn test_audio_book_path_invalid_zero() {
        let path = BookPath { audio_bible_id: 0 };
        let result = path.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("audio_bible_id"));
    }

    #[test]
    fn test_audio_book_path_invalid_negative() {
        let path = BookPath { audio_bible_id: -1 };
        let result = path.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("audio_bible_id"));
    }
}
