# Holy Bible API - Go Client

A clean, well-typed Go client for the Holy Bible API.

## Installation

```bash
go get github.com/emilsharkov/holy-bible-api/openapi-clients/go/package
```

## 🚀 Quick Start

```go
package main

import (
    "context"
    "fmt"
    "log"

    holybibleapi "github.com/emilsharkov/holy-bible-api/openapi-clients/go/package"
)

func main() {
    // Create API client (optionally pass custom base URL)
    client := holybibleapi.CreateBibleApi("https://holy-bible-api.com")
    ctx := context.Background()

    // Get all available Bibles
    bibles, _, err := client.DefaultAPI.GetBibles(ctx).Execute()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Found %d bibles\n", len(bibles.Bibles))
}
```

## 📚 Available Methods

### Text Bible Methods

#### `GetBibles(ctx)` - Get all available Bibles

```go
// Get all Bibles
bibles, resp, err := client.DefaultAPI.GetBibles(ctx).Execute()
if err != nil {
    log.Fatal(err)
}
// Response: GetBiblesRes{ Bibles: []Bible }

// Filter by language
language := "en"
englishBibles, _, err := client.DefaultAPI.GetBibles(ctx).Language(language).Execute()

// Filter by version
version := "KJV"
kjvBibles, _, err := client.DefaultAPI.GetBibles(ctx).Version(version).Execute()
```

#### `GetBibleBooks(ctx, bibleID)` - Get number of books in a Bible

```go
books, _, err := client.DefaultAPI.GetBibleBooks(ctx, 1).Execute()
if err != nil {
    log.Fatal(err)
}
// Response: GetBibleBooksRes{ NumBooks: int32 }
fmt.Printf("Bible has %d books\n", books.NumBooks)
```

#### `GetBibleChapters(ctx, bibleID, bookNum)` - Get number of chapters in a book

```go
chapters, _, err := client.DefaultAPI.GetBibleChapters(ctx, 1, 1).Execute()
if err != nil {
    log.Fatal(err)
}
// Response: GetBibleChaptersRes{ NumChapters: int32 }
fmt.Printf("Book 1 has %d chapters\n", chapters.NumChapters)
```

#### `GetBibleVerses(ctx, bibleID, bookNum, chapterNum)` - Get verses from a chapter

```go
// Get all verses from Genesis 1
verses, _, err := client.DefaultAPI.GetBibleVerses(ctx, 1, 1, 1).Execute()
if err != nil {
    log.Fatal(err)
}
// Response: GetBibleVersesRes{ Verses: []BibleVerse }

// Get specific verse range (verses 1-5)
start := int32(1)
end := int32(5)
firstFiveVerses, _, err := client.DefaultAPI.GetBibleVerses(ctx, 1, 1, 1).Start(start).End(end).Execute()
```

#### `GetBibleVerseByNumber(ctx, bibleID, bookNum, chapterNum, verseNum)` - Get a specific verse

```go
verse, _, err := client.DefaultAPI.GetBibleVerseByNumber(ctx, 1, 1, 1, 1).Execute()
if err != nil {
    log.Fatal(err)
}
// Response: BibleVerse{ BibleId, Book, Chapter, Verse, Text }
fmt.Println(verse.Text) // "In the beginning God created the heaven and the earth."
```

### Audio Bible Methods

#### `GetAudioBibles(ctx)` - Get all available audio Bibles

```go
// Get all audio Bibles
audioBibles, _, err := client.DefaultAPI.GetAudioBibles(ctx).Execute()
if err != nil {
    log.Fatal(err)
}
// Response: GetAudioBiblesRes{ AudioBibles: []AudioBible }

// Filter by language
language := "en"
englishAudioBibles, _, err := client.DefaultAPI.GetAudioBibles(ctx).Language(language).Execute()
```

#### `GetAudioBibleBooks(ctx, audioBibleID)` - Get number of books in an audio Bible

```go
audioBooks, _, err := client.DefaultAPI.GetAudioBibleBooks(ctx, 1).Execute()
if err != nil {
    log.Fatal(err)
}
// Response: GetAudioBooksRes{ NumBooks: int32 }
fmt.Printf("Audio Bible has %d books\n", audioBooks.NumBooks)
```

#### `GetAudioBibleChapters(ctx, audioBibleID, bookNum)` - Get number of chapters in an audio book

```go
audioChapters, _, err := client.DefaultAPI.GetAudioBibleChapters(ctx, 1, 1).Execute()
if err != nil {
    log.Fatal(err)
}
// Response: GetAudioChaptersRes{ NumChapters: int32 }
fmt.Printf("Audio Book 1 has %d chapters\n", audioChapters.NumChapters)
```

#### `GetAudioChapter(ctx, audioBibleID, bookNum, chapterNum)` - Stream audio for a chapter

```go
// This method streams audio data
resp, err := client.DefaultAPI.GetAudioChapter(ctx, 1, 1, 1).Execute()
if err != nil {
    log.Fatal(err)
}
defer resp.Body.Close()

// Read audio data from resp.Body
data, err := io.ReadAll(resp.Body)
if err != nil {
    log.Fatal(err)
}
// Process audio data...
```

### Health Check

#### `GetHealth(ctx)` - Check API health

```go
health, _, err := client.DefaultAPI.GetHealth(ctx).Execute()
if err != nil {
    log.Fatal(err)
}
// Response: string (e.g., "OK")
fmt.Printf("API Status: %v\n", health)
```

## 🔧 Configuration

### Basic Configuration

```go
// Use default endpoint (empty string)
client := holybibleapi.CreateBibleApi()

// Use custom endpoint
client := holybibleapi.CreateBibleApi("https://holy-bible-api.com")
```

## 📝 Complete Example

```go
package main

import (
    "context"
    "fmt"
    "log"

    holybibleapi "github.com/emilsharkov/holy-bible-api/openapi-clients/go/package"
)

func main() {
    // Create API client
    client := holybibleapi.CreateBibleApi("https://holy-bible-api.com")
    ctx := context.Background()

    // Check API health
    health, _, err := client.DefaultAPI.GetHealth(ctx).Execute()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("API Status: %v\n", health)

    // Get available Bibles
    bibles, _, err := client.DefaultAPI.GetBibles(ctx).Execute()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Available Bibles: %d\n", len(bibles.Bibles))

    // Get books in first Bible
    books, _, err := client.DefaultAPI.GetBibleBooks(ctx, 1).Execute()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Bible has %d books\n", books.NumBooks)

    // Get chapters in first book
    chapters, _, err := client.DefaultAPI.GetBibleChapters(ctx, 1, 1).Execute()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("First book has %d chapters\n", chapters.NumChapters)

    // Get verses from first chapter
    verses, _, err := client.DefaultAPI.GetBibleVerses(ctx, 1, 1, 1).Execute()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("First chapter has %d verses\n", len(verses.Verses))

    // Get a specific verse
    verse, _, err := client.DefaultAPI.GetBibleVerseByNumber(ctx, 1, 1, 1, 1).Execute()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("First verse: %s\n", verse.Text)

    // Get audio Bibles
    audioBibles, _, err := client.DefaultAPI.GetAudioBibles(ctx).Execute()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Available Audio Bibles: %d\n", len(audioBibles.AudioBibles))
}
```

## 🔄 Error Handling

All methods return three values: the result, the HTTP response, and an error. Always check for errors:

```go
result, resp, err := client.DefaultAPI.GetBibles(ctx).Execute()
if err != nil {
    // Handle error
    log.Printf("Error: %v\n", err)
    if resp != nil {
        log.Printf("Status Code: %d\n", resp.StatusCode)
    }
    return
}
// Use result
```

## 🧪 Testing

The package includes generated test files. Run tests with:

```bash
go test ./...
```

## 📦 Package Structure

```
holy-bible-api-go/
├── holybible.go       # Main client wrapper with convenience methods
├── go.mod             # Go module definition
├── README.md          # This file
└── generated/         # Auto-generated OpenAPI client
    ├── api_default.go
    ├── client.go
    ├── configuration.go
    ├── model_*.go
    └── ...
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🔗 Links

- [API Documentation](https://holy-bible-api.com)
- [GitHub Repository](https://github.com/emilsharkov/holy-bible-api)
- [OpenAPI Specification](https://holy-bible-api.com/openapi.json)

