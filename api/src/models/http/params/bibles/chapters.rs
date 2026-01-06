use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(ToSchema, IntoParams, Deserialize, Serialize, Validate)]
pub struct ChaptersPath {
    #[validate(range(min = 1, message = "Bible ID must be at least 1"))]
    pub bible_id: i32,
    #[validate(range(min = 1, message = "Book number must be at least 1"))]
    pub book_num: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chapters_path_valid() {
        let path = ChaptersPath {
            bible_id: 1,
            book_num: 1,
        };
        assert!(path.validate().is_ok());
    }

    #[test]
    fn test_chapters_path_invalid_bible_id_zero() {
        let path = ChaptersPath {
            bible_id: 0,
            book_num: 1,
        };
        let result = path.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("bible_id"));
    }

    #[test]
    fn test_chapters_path_invalid_book_num_zero() {
        let path = ChaptersPath {
            bible_id: 1,
            book_num: 0,
        };
        let result = path.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("book_num"));
    }

    #[test]
    fn test_chapters_path_invalid_both_zero() {
        let path = ChaptersPath {
            bible_id: 0,
            book_num: 0,
        };
        let result = path.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("bible_id"));
        assert!(errors.field_errors().contains_key("book_num"));
    }

    #[test]
    fn test_chapters_path_invalid_negative() {
        let path = ChaptersPath {
            bible_id: -1,
            book_num: -5,
        };
        let result = path.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("bible_id"));
        assert!(errors.field_errors().contains_key("book_num"));
    }
}
