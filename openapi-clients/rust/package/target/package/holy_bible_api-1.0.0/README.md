# Holy Bible API - Rust Client

A clean, well-typed Rust client for the Holy Bible API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
holy_bible_api = "1.0.0"
tokio = { version = "1", features = ["full"] }
```

## 🚀 Quick Start

```rust
use holy_bible_api::{create_bible_api, apis::default_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create API client (optionally pass custom base URL)
    let config = create_bible_api(Some("https://holy-bible-api.com".to_string()));
    
    // Get all available Bibles
    let bibles = default_api::get_bibles(&config, None, None).await?;
    println!("Found {} bibles", bibles.bibles.len());
    
    Ok(())
}
```

## 📚 Available Methods

### Text Bible Methods

#### `get_bibles` - Get all available Bibles

```rust
use holy_bible_api::{create_bible_api, apis::default_api};

// Get all Bibles
let bibles = default_api::get_bibles(&config, None, None).await?;
// Response: GetBiblesRes{ bibles: Vec<Bible> }

// Filter by language
let english_bibles = default_api::get_bibles(&config, Some("en"), None).await?;

// Filter by version
let kjv_bibles = default_api::get_bibles(&config, None, Some("KJV")).await?;
```

#### `get_bible_books` - Get number of books in a Bible

```rust
let books = default_api::get_bible_books(&config, 1).await?;
// Response: GetBibleBooksRes{ num_books: i32 }
println!("Bible has {} books", books.num_books);
```

#### `get_bible_chapters` - Get number of chapters in a book

```rust
let chapters = default_api::get_bible_chapters(&config, 1, 1).await?;
// Response: GetBibleChaptersRes{ num_chapters: i32 }
println!("Book 1 has {} chapters", chapters.num_chapters);
```

#### `get_bible_verses` - Get verses from a chapter

```rust
// Get all verses from Genesis 1
let verses = default_api::get_bible_verses(&config, 1, 1, 1, None, None).await?;
// Response: GetBibleVersesRes{ verses: Vec<BibleVerse> }

// Get specific verse range (verses 1-5)
let first_five_verses = default_api::get_bible_verses(&config, 1, 1, 1, Some(1), Some(5)).await?;
```

#### `get_bible_verse_by_number` - Get a specific verse

```rust
let verse = default_api::get_bible_verse_by_number(&config, 1, 1, 1, 1).await?;
// Response: BibleVerse{ bible_id, book, chapter, verse, text }
println!("{}", verse.text); // "In the beginning God created the heaven and the earth."
```

### Audio Bible Methods

#### `get_audio_bibles` - Get all available audio Bibles

```rust
// Get all audio Bibles
let audio_bibles = default_api::get_audio_bibles(&config, None, None).await?;
// Response: GetAudioBiblesRes{ audio_bibles: Vec<AudioBible> }

// Filter by language
let english_audio_bibles = default_api::get_audio_bibles(&config, Some("en"), None).await?;
```

#### `get_audio_bible_books` - Get number of books in an audio Bible

```rust
let audio_books = default_api::get_audio_bible_books(&config, 1).await?;
// Response: GetAudioBooksRes{ num_books: i32 }
println!("Audio Bible has {} books", audio_books.num_books);
```

#### `get_audio_bible_chapters` - Get number of chapters in an audio book

```rust
let audio_chapters = default_api::get_audio_bible_chapters(&config, 1, 1).await?;
// Response: GetAudioChaptersRes{ num_chapters: i32 }
println!("Audio Book 1 has {} chapters", audio_chapters.num_chapters);
```

#### `get_audio_chapter` - Stream audio for a chapter

```rust
use reqwest;

// This method streams audio data
let audio_data = default_api::get_audio_chapter(&config, 1, 1, 1).await?;
// Returns bytes (Vec<u8>) that can be saved to a file or streamed
```

### Health Check

#### `get_health` - Check API health

```rust
let health = default_api::get_health(&config).await?;
// Response: String (e.g., "OK")
println!("API Status: {}", health);
```

## 🔧 Configuration

### Basic Configuration

```rust
use holy_bible_api::create_bible_api;

// Use default endpoint
let config = create_bible_api(None);

// Use custom endpoint
let config = create_bible_api(Some("https://your-api-endpoint.com".to_string()));
```

### TLS Features

By default, the crate uses native TLS. You can opt for rustls instead:

```toml
[dependencies]
holy_bible_api = { version = "1.0.0", default-features = false, features = ["rustls-tls"] }
```

## 📝 Complete Example

```rust
use holy_bible_api::{create_bible_api, apis::default_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create API client
    let config = create_bible_api(Some("https://holy-bible-api.com".to_string()));
    
    // Check API health
    let health = default_api::get_health(&config).await?;
    println!("API Status: {}", health);
    
    // Get available Bibles
    let bibles = default_api::get_bibles(&config, None, None).await?;
    println!("Available Bibles: {}", bibles.bibles.len());
    
    // Get books in first Bible
    let books = default_api::get_bible_books(&config, 1).await?;
    println!("Bible has {} books", books.num_books);
    
    // Get chapters in first book
    let chapters = default_api::get_bible_chapters(&config, 1, 1).await?;
    println!("First book has {} chapters", chapters.num_chapters);
    
    // Get verses from first chapter
    let verses = default_api::get_bible_verses(&config, 1, 1, 1, None, None).await?;
    println!("First chapter has {} verses", verses.verses.len());
    
    // Get a specific verse
    let verse = default_api::get_bible_verse_by_number(&config, 1, 1, 1, 1).await?;
    println!("First verse: {}", verse.text);
    
    // Get audio Bibles
    let audio_bibles = default_api::get_audio_bibles(&config, None, None).await?;
    println!("Available Audio Bibles: {}", audio_bibles.audio_bibles.len());
    
    Ok(())
}
```

## 🔄 Error Handling

All methods return a `Result` type. Always handle errors appropriately:

```rust
match default_api::get_bibles(&config, None, None).await {
    Ok(bibles) => {
        println!("Found {} bibles", bibles.bibles.len());
    }
    Err(e) => {
        eprintln!("Error: {:?}", e);
    }
}
```

## 🧪 Testing

Run tests with:

```bash
cargo test
```

## 📦 Package Structure

```
holy-bible-api/
├── src/
│   └── lib.rs         # Main library wrapper with convenience methods
├── generated/         # Auto-generated OpenAPI client
│   ├── src/
│   │   ├── apis/
│   │   ├── models/
│   │   └── lib.rs
│   └── Cargo.toml
├── Cargo.toml         # Package definition
└── README.md          # This file
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🔗 Links

- [API Documentation](https://holy-bible-api.com)
- [GitHub Repository](https://github.com/emilsharkov/holy-bible-api)
- [OpenAPI Specification](https://holy-bible-api.com/openapi.json)

