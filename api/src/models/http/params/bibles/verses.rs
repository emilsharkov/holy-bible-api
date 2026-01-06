use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(ToSchema, IntoParams, Deserialize, Serialize, Validate)]
#[into_params(parameter_in = Query)]
pub struct VersesQuery {
    #[validate(range(min = 1, message = "Start verse must be at least 1"))]
    pub start: Option<i32>,
    #[validate(range(min = 1, message = "End verse must be at least 1"))]
    pub end: Option<i32>,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize, Validate)]
pub struct VersesPath {
    #[validate(range(min = 1, message = "Bible ID must be at least 1"))]
    pub bible_id: i32,
    #[validate(range(min = 1, message = "Book number must be at least 1"))]
    pub book_num: i32,
    #[validate(range(min = 1, message = "Chapter number must be at least 1"))]
    pub chapter_num: i32,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize, Validate)]
pub struct VerseByNumberPath {
    #[validate(range(min = 1, message = "Bible ID must be at least 1"))]
    pub bible_id: i32,
    #[validate(range(min = 1, message = "Book number must be at least 1"))]
    pub book_num: i32,
    #[validate(range(min = 1, message = "Chapter number must be at least 1"))]
    pub chapter_num: i32,
    #[validate(range(min = 1, message = "Verse number must be at least 1"))]
    pub verse_num: i32,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize, Validate)]
pub struct RandomBibleVersePath {
    #[validate(range(min = 1, message = "Bible ID must be at least 1"))]
    pub bible_id: i32,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize, Validate)]
pub struct VerseOfTheDayPath {
    #[validate(range(min = 1, message = "Bible ID must be at least 1"))]
    pub bible_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod verses_query_tests {
        use super::*;

        #[test]
        fn test_verses_query_valid_empty() {
            let query = VersesQuery {
                start: None,
                end: None,
            };
            assert!(query.validate().is_ok());
        }

        #[test]
        fn test_verses_query_valid_start_only() {
            let query = VersesQuery {
                start: Some(1),
                end: None,
            };
            assert!(query.validate().is_ok());
        }

        #[test]
        fn test_verses_query_valid_end_only() {
            let query = VersesQuery {
                start: None,
                end: Some(10),
            };
            assert!(query.validate().is_ok());
        }

        #[test]
        fn test_verses_query_valid_both() {
            let query = VersesQuery {
                start: Some(1),
                end: Some(10),
            };
            assert!(query.validate().is_ok());
        }

        #[test]
        fn test_verses_query_invalid_start_zero() {
            let query = VersesQuery {
                start: Some(0),
                end: None,
            };
            let result = query.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("start"));
        }

        #[test]
        fn test_verses_query_invalid_start_negative() {
            let query = VersesQuery {
                start: Some(-1),
                end: None,
            };
            let result = query.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("start"));
        }

        #[test]
        fn test_verses_query_invalid_end_zero() {
            let query = VersesQuery {
                start: None,
                end: Some(0),
            };
            let result = query.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("end"));
        }

        #[test]
        fn test_verses_query_invalid_end_negative() {
            let query = VersesQuery {
                start: None,
                end: Some(-5),
            };
            let result = query.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("end"));
        }
    }

    mod verses_path_tests {
        use super::*;

        #[test]
        fn test_verses_path_valid() {
            let path = VersesPath {
                bible_id: 1,
                book_num: 1,
                chapter_num: 1,
            };
            assert!(path.validate().is_ok());
        }

        #[test]
        fn test_verses_path_invalid_bible_id_zero() {
            let path = VersesPath {
                bible_id: 0,
                book_num: 1,
                chapter_num: 1,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("bible_id"));
        }

        #[test]
        fn test_verses_path_invalid_book_num_zero() {
            let path = VersesPath {
                bible_id: 1,
                book_num: 0,
                chapter_num: 1,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("book_num"));
        }

        #[test]
        fn test_verses_path_invalid_chapter_num_zero() {
            let path = VersesPath {
                bible_id: 1,
                book_num: 1,
                chapter_num: 0,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("chapter_num"));
        }

        #[test]
        fn test_verses_path_invalid_all_zero() {
            let path = VersesPath {
                bible_id: 0,
                book_num: 0,
                chapter_num: 0,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("bible_id"));
            assert!(errors.field_errors().contains_key("book_num"));
            assert!(errors.field_errors().contains_key("chapter_num"));
        }
    }

    mod verse_by_number_path_tests {
        use super::*;

        #[test]
        fn test_verse_by_number_path_valid() {
            let path = VerseByNumberPath {
                bible_id: 1,
                book_num: 1,
                chapter_num: 1,
                verse_num: 1,
            };
            assert!(path.validate().is_ok());
        }

        #[test]
        fn test_verse_by_number_path_invalid_verse_num_zero() {
            let path = VerseByNumberPath {
                bible_id: 1,
                book_num: 1,
                chapter_num: 1,
                verse_num: 0,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("verse_num"));
        }

        #[test]
        fn test_verse_by_number_path_invalid_all_fields() {
            let path = VerseByNumberPath {
                bible_id: 0,
                book_num: 0,
                chapter_num: 0,
                verse_num: 0,
            };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("bible_id"));
            assert!(errors.field_errors().contains_key("book_num"));
            assert!(errors.field_errors().contains_key("chapter_num"));
            assert!(errors.field_errors().contains_key("verse_num"));
        }
    }

    mod random_bible_verse_path_tests {
        use super::*;

        #[test]
        fn test_random_bible_verse_path_valid() {
            let path = RandomBibleVersePath { bible_id: 1 };
            assert!(path.validate().is_ok());
        }

        #[test]
        fn test_random_bible_verse_path_invalid_zero() {
            let path = RandomBibleVersePath { bible_id: 0 };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("bible_id"));
        }

        #[test]
        fn test_random_bible_verse_path_invalid_negative() {
            let path = RandomBibleVersePath { bible_id: -1 };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("bible_id"));
        }
    }

    mod verse_of_the_day_path_tests {
        use super::*;

        #[test]
        fn test_verse_of_the_day_path_valid() {
            let path = VerseOfTheDayPath { bible_id: 1 };
            assert!(path.validate().is_ok());
        }

        #[test]
        fn test_verse_of_the_day_path_invalid_zero() {
            let path = VerseOfTheDayPath { bible_id: 0 };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("bible_id"));
        }

        #[test]
        fn test_verse_of_the_day_path_invalid_negative() {
            let path = VerseOfTheDayPath { bible_id: -1 };
            let result = path.validate();
            assert!(result.is_err());
            let errors = result.unwrap_err();
            assert!(errors.field_errors().contains_key("bible_id"));
        }
    }
}
