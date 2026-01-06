mod common;

use common::TestServer;

#[tokio::test]
async fn test_get_bibles() {
    let server = TestServer::start().await.expect("Failed to start test server");
    let client = server.client();
    
    let response = client
        .get(&format!("{}/bibles", server.base_url))
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(response.status(), 200, "GET /bibles should return 200");
    
    let bibles: Vec<serde_json::Value> = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    
    // Verify the structure of at least one bible
    if let Some(bible) = bibles.first() {
        assert!(bible.get("bible_id").is_some(), "Bible should have bible_id");
        assert!(bible.get("language").is_some(), "Bible should have language");
        // version is optional, so we don't assert it
    }
}

#[tokio::test]
async fn test_get_bibles_with_query_params() {
    let server = TestServer::start().await.expect("Failed to start test server");
    let client = server.client();
    
    // Test with language filter
    let response = client
        .get(&format!("{}/bibles?language=en", server.base_url))
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(response.status(), 200, "GET /bibles?language=en should return 200");
    
    let _bibles: Vec<serde_json::Value> = response
        .json()
        .await
        .expect("Failed to parse JSON response");
}

#[tokio::test]
async fn test_get_bible_books() {
    let server = TestServer::start().await.expect("Failed to start test server");
    let client = server.client();
    
    // First, get a bible_id to use
    let bibles_response = client
        .get(&format!("{}/bibles", server.base_url))
        .send()
        .await
        .expect("Failed to get bibles");
    
    let bibles: Vec<serde_json::Value> = bibles_response
        .json()
        .await
        .expect("Failed to parse bibles");
    
    if let Some(bible) = bibles.first() {
        let bible_id = bible.get("bible_id")
            .and_then(|v| v.as_i64())
            .expect("bible_id should be a number");
        
        let response = client
            .get(&format!("{}/bibles/{}/books", server.base_url, bible_id))
            .send()
            .await
            .expect("Failed to send request");
        
        assert_eq!(response.status(), 200, "GET /bibles/{{bible_id}}/books should return 200");
        
        let result: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse JSON response");
        
        assert!(result.get("num_books").is_some(), "Response should have num_books");
        let num_books = result.get("num_books")
            .and_then(|v| v.as_i64())
            .expect("num_books should be a number");
        assert!(num_books > 0, "num_books should be positive");
    }
}

#[tokio::test]
async fn test_get_bible_chapters() {
    let server = TestServer::start().await.expect("Failed to start test server");
    let client = server.client();
    
    // Get a bible_id and book_num
    let bibles_response = client
        .get(&format!("{}/bibles", server.base_url))
        .send()
        .await
        .expect("Failed to get bibles");
    
    let bibles: Vec<serde_json::Value> = bibles_response
        .json()
        .await
        .expect("Failed to parse bibles");
    
    if let Some(bible) = bibles.first() {
        let bible_id = bible.get("bible_id")
            .and_then(|v| v.as_i64())
            .expect("bible_id should be a number");
        
        // Get books first
        let books_response = client
            .get(&format!("{}/bibles/{}/books", server.base_url, bible_id))
            .send()
            .await
            .expect("Failed to get books");
        
        let books_result: serde_json::Value = books_response
            .json()
            .await
            .expect("Failed to parse books response");
        
        let num_books = books_result.get("num_books")
            .and_then(|v| v.as_i64())
            .expect("num_books should be a number");
        
        if num_books > 0 {
            // Test with book_num = 1 (typically Genesis)
            let response = client
                .get(&format!("{}/bibles/{}/books/1/chapters", server.base_url, bible_id))
                .send()
                .await
                .expect("Failed to send request");
            
            assert_eq!(response.status(), 200, "GET /bibles/{{id}}/books/{{num}}/chapters endpoint should return 200");
            
            let result: serde_json::Value = response
                .json()
                .await
                .expect("Failed to parse JSON response");
            
            assert!(result.get("num_chapters").is_some(), "Response should have num_chapters");
            let num_chapters = result.get("num_chapters")
                .and_then(|v| v.as_i64())
                .expect("num_chapters should be a number");
            assert!(num_chapters > 0, "num_chapters should be positive");
        }
    }
}

#[tokio::test]
async fn test_get_bible_verses() {
    let server = TestServer::start().await.expect("Failed to start test server");
    let client = server.client();
    
    // Get a bible_id
    let bibles_response = client
        .get(&format!("{}/bibles", server.base_url))
        .send()
        .await
        .expect("Failed to get bibles");
    
    let bibles: Vec<serde_json::Value> = bibles_response
        .json()
        .await
        .expect("Failed to parse bibles");
    
    if let Some(bible) = bibles.first() {
        let bible_id = bible.get("bible_id")
            .and_then(|v| v.as_i64())
            .expect("bible_id should be a number");
        
        // Test getting verses from Genesis 1 (book 1, chapter 1)
        let response = client
            .get(&format!("{}/bibles/{}/books/1/chapters/1/verses", server.base_url, bible_id))
            .send()
            .await
            .expect("Failed to send request");
        
        assert_eq!(response.status(), 200, "GET /bibles/{{id}}/books/{{num}}/chapters/{{num}}/verses endpoint should return 200");
        
        let verses: Vec<serde_json::Value> = response
            .json()
            .await
            .expect("Failed to parse JSON response");
        
        // Verify verse structure
        if let Some(verse) = verses.first() {
            assert!(verse.get("verse").is_some(), "Verse should have verse");
            assert!(verse.get("text").is_some(), "Verse should have text");
        }
    }
}

#[tokio::test]
async fn test_get_bible_verse_by_number() {
    let server = TestServer::start().await.expect("Failed to start test server");
    let client = server.client();
    
    // Get a bible_id
    let bibles_response = client
        .get(&format!("{}/bibles", server.base_url))
        .send()
        .await
        .expect("Failed to get bibles");
    
    let bibles: Vec<serde_json::Value> = bibles_response
        .json()
        .await
        .expect("Failed to parse bibles");
    
    if let Some(bible) = bibles.first() {
        let bible_id = bible.get("bible_id")
            .and_then(|v| v.as_i64())
            .expect("bible_id should be a number");
        
        // Test getting verse 1 from Genesis 1:1
        let response = client
            .get(&format!("{}/bibles/{}/books/1/chapters/1/verses/1", server.base_url, bible_id))
            .send()
            .await
            .expect("Failed to send request");
        
        assert_eq!(response.status(), 200, "GET /bibles/{{id}}/books/{{num}}/chapters/{{num}}/verses/{{num}} endpoint should return 200");
        
        let verse: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse JSON response");
        
        assert!(verse.get("verse").is_some(), "Verse should have verse");
        assert!(verse.get("text").is_some(), "Verse should have text");
    }
}

#[tokio::test]
async fn test_get_random_bible_verse() {
    let server = TestServer::start().await.expect("Failed to start test server");
    let client = server.client();
    
    // Get a bible_id
    let bibles_response = client
        .get(&format!("{}/bibles", server.base_url))
        .send()
        .await
        .expect("Failed to get bibles");
    
    let bibles: Vec<serde_json::Value> = bibles_response
        .json()
        .await
        .expect("Failed to parse bibles");
    
    if let Some(bible) = bibles.first() {
        let bible_id = bible.get("bible_id")
            .and_then(|v| v.as_i64())
            .expect("bible_id should be a number");
        
        let response = client
            .get(&format!("{}/bibles/{}/random", server.base_url, bible_id))
            .send()
            .await
            .expect("Failed to send request");
        
        assert_eq!(response.status(), 200, "GET /bibles/{{id}}/random endpoint should return 200");
        
        let verse: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse JSON response");
        
        assert!(verse.get("verse").is_some(), "Verse should have verse");
        assert!(verse.get("text").is_some(), "Verse should have text");
    }
}

#[tokio::test]
async fn test_get_verse_of_the_day() {
    let server = TestServer::start().await.expect("Failed to start test server");
    let client = server.client();
    
    // Get a bible_id
    let bibles_response = client
        .get(&format!("{}/bibles", server.base_url))
        .send()
        .await
        .expect("Failed to get bibles");
    
    let bibles: Vec<serde_json::Value> = bibles_response
        .json()
        .await
        .expect("Failed to parse bibles");
    
    if let Some(bible) = bibles.first() {
        let bible_id = bible.get("bible_id")
            .and_then(|v| v.as_i64())
            .expect("bible_id should be a number");
        
        let response = client
            .get(&format!("{}/bibles/{}/verse-of-the-day", server.base_url, bible_id))
            .send()
            .await
            .expect("Failed to send request");
        
        assert_eq!(response.status(), 200, "GET /bibles/{{id}}/verse-of-the-day endpoint should return 200");
        
        let verse: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse JSON response");
        
        assert!(verse.get("verse").is_some(), "Verse should have verse");
        assert!(verse.get("text").is_some(), "Verse should have text");
        
        // Verify that calling it twice on the same day returns the same verse
        let response2 = client
            .get(&format!("{}/bibles/{}/verse-of-the-day", server.base_url, bible_id))
            .send()
            .await
            .expect("Failed to send second request");
        
        let verse2: serde_json::Value = response2
            .json()
            .await
            .expect("Failed to parse second JSON response");
        
        // They should be the same verse (same day)
        assert_eq!(
            verse.get("verse"),
            verse2.get("verse"),
            "Verse of the day should be consistent within the same day"
        );
    }
}

