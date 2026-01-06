use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(ToSchema, IntoParams, Deserialize, Serialize, Validate)]
pub struct ChaptersPath {
    #[validate(range(min = 1, message = "Audio Bible ID must be at least 1"))]
    pub audio_bible_id: i32,
    #[validate(range(min = 1, message = "Book number must be at least 1"))]
    pub book_num: i32,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize, Validate)]
pub struct AudioChapterPath {
    #[validate(range(min = 1, message = "Audio Bible ID must be at least 1"))]
    pub audio_bible_id: i32,
    #[validate(range(min = 1, message = "Book number must be at least 1"))]
    pub book_num: i32,
    #[validate(range(min = 1, message = "Chapter number must be at least 1"))]
    pub chapter_num: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod chapters_path_tests {
        use super::*;

        #[test]
        fn test_audio_chapters_path_valid() {
            let path = ChaptersPath {
                audio_bible_id: 1,
                book_num: 1,
            };
            assert!(path.validate().is_ok());
        }

        #[test]
        fn test_audio_chapters_path_invalid_audio_bible_id_zero() {
            let path = ChaptersPath {
                audio_bible_id: 0,
                book_num: 1,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("audio_bible_id"));
        }

        #[test]
        fn test_audio_chapters_path_invalid_book_num_zero() {
            let path = ChaptersPath {
                audio_bible_id: 1,
                book_num: 0,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("book_num"));
        }

        #[test]
        fn test_audio_chapters_path_invalid_both_zero() {
            let path = ChaptersPath {
                audio_bible_id: 0,
                book_num: 0,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("audio_bible_id"));
            assert!(errors.field_errors().contains_key("book_num"));
        }

        #[test]
        fn test_audio_chapters_path_invalid_negative() {
            let path = ChaptersPath {
                audio_bible_id: -1,
                book_num: -5,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("audio_bible_id"));
            assert!(errors.field_errors().contains_key("book_num"));
        }
    }

    mod audio_chapter_path_tests {
        use super::*;

        #[test]
        fn test_audio_chapter_path_valid() {
            let path = AudioChapterPath {
                audio_bible_id: 1,
                book_num: 1,
                chapter_num: 1,
            };
            assert!(path.validate().is_ok());
        }

        #[test]
        fn test_audio_chapter_path_invalid_audio_bible_id_zero() {
            let path = AudioChapterPath {
                audio_bible_id: 0,
                book_num: 1,
                chapter_num: 1,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("audio_bible_id"));
        }

        #[test]
        fn test_audio_chapter_path_invalid_book_num_zero() {
            let path = AudioChapterPath {
                audio_bible_id: 1,
                book_num: 0,
                chapter_num: 1,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("book_num"));
        }

        #[test]
        fn test_audio_chapter_path_invalid_chapter_num_zero() {
            let path = AudioChapterPath {
                audio_bible_id: 1,
                book_num: 1,
                chapter_num: 0,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("chapter_num"));
        }

        #[test]
        fn test_audio_chapter_path_invalid_all_zero() {
            let path = AudioChapterPath {
                audio_bible_id: 0,
                book_num: 0,
                chapter_num: 0,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("audio_bible_id"));
            assert!(errors.field_errors().contains_key("book_num"));
            assert!(errors.field_errors().contains_key("chapter_num"));
        }

        #[test]
        fn test_audio_chapter_path_invalid_negative() {
            let path = AudioChapterPath {
                audio_bible_id: -1,
                book_num: -5,
                chapter_num: -10,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("audio_bible_id"));
            assert!(errors.field_errors().contains_key("book_num"));
            assert!(errors.field_errors().contains_key("chapter_num"));
        }
    }
}
