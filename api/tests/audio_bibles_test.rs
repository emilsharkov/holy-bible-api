mod common;

use common::TestServer;

#[tokio::test]
async fn test_get_audio_bibles() {
    let server = TestServer::start()
        .await
        .expect("Failed to start test server");
    let client = server.client();

    let response = client
        .get(&format!("{}/audio_bibles", server.base_url))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(
        response.status(),
        200,
        "GET /audio_bibles should return 200"
    );

    let audio_bibles: Vec<serde_json::Value> = response
        .json()
        .await
        .expect("Failed to parse JSON response");

    // Verify the structure of at least one audio bible
    if let Some(audio_bible) = audio_bibles.first() {
        assert!(
            audio_bible.get("audio_bible_id").is_some(),
            "AudioBible should have audio_bible_id"
        );
        assert!(
            audio_bible.get("language").is_some(),
            "AudioBible should have language"
        );
    }
}

#[tokio::test]
async fn test_get_audio_bibles_with_query_params() {
    let server = TestServer::start()
        .await
        .expect("Failed to start test server");
    let client = server.client();

    // Test with language filter
    let response = client
        .get(&format!("{}/audio_bibles?language=en", server.base_url))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(
        response.status(),
        200,
        "GET /audio_bibles?language=en should return 200"
    );

    let _audio_bibles: Vec<serde_json::Value> = response
        .json()
        .await
        .expect("Failed to parse JSON response");
}

#[tokio::test]
async fn test_get_audio_bible_books() {
    let server = TestServer::start()
        .await
        .expect("Failed to start test server");
    let client = server.client();

    // First, get an audio_bible_id to use
    let audio_bibles_response = client
        .get(&format!("{}/audio_bibles", server.base_url))
        .send()
        .await
        .expect("Failed to get audio bibles");

    let audio_bibles: Vec<serde_json::Value> = audio_bibles_response
        .json()
        .await
        .expect("Failed to parse audio bibles");

    if let Some(audio_bible) = audio_bibles.first() {
        let audio_bible_id = audio_bible
            .get("audio_bible_id")
            .and_then(|v| v.as_i64())
            .expect("audio_bible_id should be a number");

        let response = client
            .get(&format!(
                "{}/audio_bibles/{}/books",
                server.base_url, audio_bible_id
            ))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            response.status(),
            200,
            "GET /audio_bibles/{{audio_bible_id}}/books should return 200"
        );

        let result: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse JSON response");

        assert!(
            result.get("num_books").is_some(),
            "Response should have num_books"
        );
        let num_books = result
            .get("num_books")
            .and_then(|v| v.as_i64())
            .expect("num_books should be a number");
        assert!(num_books > 0, "num_books should be positive");
    }
}

#[tokio::test]
async fn test_get_audio_bible_chapters() {
    let server = TestServer::start()
        .await
        .expect("Failed to start test server");
    let client = server.client();

    // Get an audio_bible_id and book_num
    let audio_bibles_response = client
        .get(&format!("{}/audio_bibles", server.base_url))
        .send()
        .await
        .expect("Failed to get audio bibles");

    let audio_bibles: Vec<serde_json::Value> = audio_bibles_response
        .json()
        .await
        .expect("Failed to parse audio bibles");

    if let Some(audio_bible) = audio_bibles.first() {
        let audio_bible_id = audio_bible
            .get("audio_bible_id")
            .and_then(|v| v.as_i64())
            .expect("audio_bible_id should be a number");

        // Get books first
        let books_response = client
            .get(&format!(
                "{}/audio_bibles/{}/books",
                server.base_url, audio_bible_id
            ))
            .send()
            .await
            .expect("Failed to get books");

        let books_result: serde_json::Value = books_response
            .json()
            .await
            .expect("Failed to parse books response");

        let num_books = books_result
            .get("num_books")
            .and_then(|v| v.as_i64())
            .expect("num_books should be a number");

        if num_books > 0 {
            // Test with book_num = 1 (typically Genesis)
            let response = client
                .get(&format!(
                    "{}/audio_bibles/{}/books/1/chapters",
                    server.base_url, audio_bible_id
                ))
                .send()
                .await
                .expect("Failed to send request");

            assert_eq!(
                response.status(),
                200,
                "GET /audio_bibles/{{id}}/books/{{num}}/chapters should return 200"
            );

            let result: serde_json::Value = response
                .json()
                .await
                .expect("Failed to parse JSON response");

            assert!(
                result.get("num_chapters").is_some(),
                "Response should have num_chapters"
            );
            let num_chapters = result
                .get("num_chapters")
                .and_then(|v| v.as_i64())
                .expect("num_chapters should be a number");
            assert!(num_chapters > 0, "num_chapters should be positive");
        }
    }
}

#[tokio::test]
async fn test_get_audio_chapter() {
    let server = TestServer::start()
        .await
        .expect("Failed to start test server");
    let client = server.client();

    // Get an audio_bible_id
    let audio_bibles_response = client
        .get(&format!("{}/audio_bibles", server.base_url))
        .send()
        .await
        .expect("Failed to get audio bibles");

    let audio_bibles: Vec<serde_json::Value> = audio_bibles_response
        .json()
        .await
        .expect("Failed to parse audio bibles");

    if let Some(audio_bible) = audio_bibles.first() {
        let audio_bible_id = audio_bible
            .get("audio_bible_id")
            .and_then(|v| v.as_i64())
            .expect("audio_bible_id should be a number");

        // Test getting audio chapter from Genesis 1 (book 1, chapter 1)
        let response = client
            .get(&format!(
                "{}/audio_bibles/{}/books/1/chapters/1",
                server.base_url, audio_bible_id
            ))
            .send()
            .await
            .expect("Failed to send request");

        // This endpoint might return 404 if the audio file doesn't exist, or 200 if it does
        // So we check for either status
        assert!(
            response.status() == 200 || response.status() == 404,
            "GET /audio_bibles/{{id}}/books/{{num}}/chapters/{{num}} should return 200 or 404"
        );

        if response.status() == 200 {
            // Verify it's an audio file
            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");

            assert!(
                content_type.contains("audio"),
                "Response should have audio content-type, got: {}",
                content_type
            );
        }
    }
}

// Validation tests - verify that invalid parameters return appropriate error status codes
// Note: axum-valid returns 400 Bad Request for validation errors, not 422
mod validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_audio_bibles_invalid_language_too_long() {
        let server = TestServer::start()
            .await
            .expect("Failed to start test server");
        let client = server.client();

        let long_language = "a".repeat(101);
        let response = client
            .get(&format!(
                "{}/audio_bibles?language={}",
                server.base_url, long_language
            ))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            response.status(),
            400,
            "GET /audio_bibles with language > 100 chars should return 400"
        );
    }

    #[tokio::test]
    async fn test_get_audio_bibles_invalid_version_too_long() {
        let server = TestServer::start()
            .await
            .expect("Failed to start test server");
        let client = server.client();

        let long_version = "a".repeat(101);
        let response = client
            .get(&format!(
                "{}/audio_bibles?version={}",
                server.base_url, long_version
            ))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            response.status(),
            400,
            "GET /audio_bibles with version > 100 chars should return 400"
        );
    }

    #[tokio::test]
    async fn test_get_audio_bible_books_invalid_audio_bible_id_zero() {
        let server = TestServer::start()
            .await
            .expect("Failed to start test server");
        let client = server.client();

        let response = client
            .get(&format!("{}/audio_bibles/0/books", server.base_url))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            response.status(),
            400,
            "GET /audio_bibles/0/books should return 400 for invalid audio_bible_id"
        );
    }

    #[tokio::test]
    async fn test_get_audio_bible_books_invalid_audio_bible_id_negative() {
        let server = TestServer::start()
            .await
            .expect("Failed to start test server");
        let client = server.client();

        let response = client
            .get(&format!("{}/audio_bibles/-1/books", server.base_url))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            response.status(),
            400,
            "GET /audio_bibles/-1/books should return 400 for negative audio_bible_id"
        );
    }

    #[tokio::test]
    async fn test_get_audio_bible_chapters_invalid_book_num_zero() {
        let server = TestServer::start()
            .await
            .expect("Failed to start test server");
        let client = server.client();

        // First get a valid audio_bible_id
        let audio_bibles_response = client
            .get(&format!("{}/audio_bibles", server.base_url))
            .send()
            .await
            .expect("Failed to get audio bibles");

        let audio_bibles: Vec<serde_json::Value> = audio_bibles_response
            .json()
            .await
            .expect("Failed to parse audio bibles");

        if let Some(audio_bible) = audio_bibles.first() {
            let audio_bible_id = audio_bible
                .get("audio_bible_id")
                .and_then(|v| v.as_i64())
                .expect("audio_bible_id should be a number");

            let response = client
                .get(&format!(
                    "{}/audio_bibles/{}/books/0/chapters",
                    server.base_url, audio_bible_id
                ))
                .send()
                .await
                .expect("Failed to send request");

            assert_eq!(
                response.status(),
                400,
                "GET /audio_bibles/{{id}}/books/0/chapters should return 400 for invalid book_num"
            );
        }
    }

    #[tokio::test]
    async fn test_get_audio_bible_chapters_invalid_audio_bible_id_zero() {
        let server = TestServer::start()
            .await
            .expect("Failed to start test server");
        let client = server.client();

        let response = client
            .get(&format!(
                "{}/audio_bibles/0/books/1/chapters",
                server.base_url
            ))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            response.status(),
            400,
            "GET /audio_bibles/0/books/1/chapters should return 400 for invalid audio_bible_id"
        );
    }

    #[tokio::test]
    async fn test_get_audio_chapter_invalid_chapter_num_zero() {
        let server = TestServer::start()
            .await
            .expect("Failed to start test server");
        let client = server.client();

        // First get a valid audio_bible_id
        let audio_bibles_response = client
            .get(&format!("{}/audio_bibles", server.base_url))
            .send()
            .await
            .expect("Failed to get audio bibles");

        let audio_bibles: Vec<serde_json::Value> = audio_bibles_response
            .json()
            .await
            .expect("Failed to parse audio bibles");

        if let Some(audio_bible) = audio_bibles.first() {
            let audio_bible_id = audio_bible
                .get("audio_bible_id")
                .and_then(|v| v.as_i64())
                .expect("audio_bible_id should be a number");

            let response = client
                .get(&format!(
                    "{}/audio_bibles/{}/books/1/chapters/0",
                    server.base_url, audio_bible_id
                ))
                .send()
                .await
                .expect("Failed to send request");

            assert_eq!(
                response.status(),
                400,
                "GET /audio_bibles/{{id}}/books/1/chapters/0 should return 400"
            );
        }
    }

    #[tokio::test]
    async fn test_get_audio_chapter_invalid_audio_bible_id_zero() {
        let server = TestServer::start()
            .await
            .expect("Failed to start test server");
        let client = server.client();

        let response = client
            .get(&format!(
                "{}/audio_bibles/0/books/1/chapters/1",
                server.base_url
            ))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            response.status(),
            400,
            "GET /audio_bibles/0/books/1/chapters/1 should return 400 for invalid audio_bible_id"
        );
    }

    #[tokio::test]
    async fn test_get_audio_chapter_invalid_book_num_zero() {
        let server = TestServer::start()
            .await
            .expect("Failed to start test server");
        let client = server.client();

        // First get a valid audio_bible_id
        let audio_bibles_response = client
            .get(&format!("{}/audio_bibles", server.base_url))
            .send()
            .await
            .expect("Failed to get audio bibles");

        let audio_bibles: Vec<serde_json::Value> = audio_bibles_response
            .json()
            .await
            .expect("Failed to parse audio bibles");

        if let Some(audio_bible) = audio_bibles.first() {
            let audio_bible_id = audio_bible
                .get("audio_bible_id")
                .and_then(|v| v.as_i64())
                .expect("audio_bible_id should be a number");

            let response = client
                .get(&format!(
                    "{}/audio_bibles/{}/books/0/chapters/1",
                    server.base_url, audio_bible_id
                ))
                .send()
                .await
                .expect("Failed to send request");

            assert_eq!(
                response.status(),
                400,
                "GET /audio_bibles/{{id}}/books/0/chapters/1 should return 400 for invalid book_num"
            );
        }
    }
}
