# Holy Bible API - TypeScript Client

A clean, well-typed TypeScript client for the Holy Bible API.

### Installation

```bash
npm install holy-bible-api-typescript
```

## 🚀 Quick Start

```typescript
import { createBibleApi } from 'holy-bible-api-typescript';

// Create API instance
const api = createBibleApi();

// Get all available Bibles
const bibles = await api.getBibles();
console.log(bibles);
```

## 📚 Available Methods

### Text Bible Methods

#### `getBibles(options?)` - Get all available Bibles
```typescript
// Get all Bibles
const bibles = await api.getBibles();
// Response: { bibles: Array<{ bibleId: number, language: string, version?: string }> }

// Filter by language
const englishBibles = await api.getBibles({ language: "en" });

// Filter by version
const kjvBibles = await api.getBibles({ version: "KJV" });
```

#### `getBibleBooks({ bibleId })` - Get number of books in a Bible
```typescript
const books = await api.getBibleBooks({ bibleId: 1 });
// Response: { numBooks: number }
console.log(`Bible has ${books.numBooks} books`);
```

#### `getBibleChapters({ bibleId, bookNum })` - Get number of chapters in a book
```typescript
const chapters = await api.getBibleChapters({ bibleId: 1, bookNum: 1 });
// Response: { numChapters: number }
console.log(`Book 1 has ${chapters.numChapters} chapters`);
```

#### `getBibleVerses({ bibleId, bookNum, chapterNum, start?, end? })` - Get verses from a chapter
```typescript
// Get all verses from Genesis 1
const verses = await api.getBibleVerses({ 
  bibleId: 1, 
  bookNum: 1, 
  chapterNum: 1 
});
// Response: { verses: Array<{ bibleId: number, book: number, chapter: number, verse: number, text: string }> }

// Get specific verse range (verses 1-5)
const firstFiveVerses = await api.getBibleVerses({ 
  bibleId: 1, 
  bookNum: 1, 
  chapterNum: 1,
  start: 1,
  end: 5
});
```

#### `getBibleVerseByNumber({ bibleId, bookNum, chapterNum, verseNum })` - Get a specific verse
```typescript
const verse = await api.getBibleVerseByNumber({ 
  bibleId: 1, 
  bookNum: 1, 
  chapterNum: 1, 
  verseNum: 1 
});
// Response: { bibleId: number, book: number, chapter: number, verse: number, text: string }
console.log(verse.text); // "In the beginning God created the heaven and the earth."
```

### Audio Bible Methods

#### `getAudioBibles(options?)` - Get all available audio Bibles
```typescript
// Get all audio Bibles
const audioBibles = await api.getAudioBibles();
// Response: { audioBibles: Array<{ audioBibleId: number, language: string, version?: string }> }

// Filter by language
const englishAudioBibles = await api.getAudioBibles({ language: "en" });
```

#### `getAudioBibleBooks({ audioBibleId })` - Get number of books in an audio Bible
```typescript
const audioBooks = await api.getAudioBibleBooks({ audioBibleId: 1 });
// Response: { numBooks: number }
console.log(`Audio Bible has ${audioBooks.numBooks} books`);
```

#### `getAudioBibleChapters({ audioBibleId, bookNum })` - Get number of chapters in an audio book
```typescript
const audioChapters = await api.getAudioBibleChapters({ audioBibleId: 1, bookNum: 1 });
// Response: { numChapters: number }
console.log(`Audio Book 1 has ${audioChapters.numChapters} chapters`);
```

#### `getAudioChapter({ audioBibleId, bookNum, chapterNum })` - Stream audio for a chapter
```typescript
// This method streams audio data (returns void)
await api.getAudioChapter({ 
  audioBibleId: 1, 
  bookNum: 1, 
  chapterNum: 1 
});
```

### Health Check

#### `getHealth()` - Check API health
```typescript
const health = await api.getHealth();
// Response: string (e.g., "OK")
console.log(`API Status: ${health}`);
```

## 🔧 Configuration

```typescript
import { createBibleApi } from 'holy-bible-api-typescript';

// Use default endpoint
const api = createBibleApi();

// Use custom endpoint
const api = createBibleApi('https://your-api-endpoint.com');
```

## 📝 Complete Example

```typescript
import { createBibleApi } from 'holy-bible-api-typescript';

async function demonstrateAPI() {
  const api = createBibleApi();
  
  try {
    // Check API health
    const health = await api.getHealth();
    console.log('API Status:', health);
    
    // Get available Bibles
    const bibles = await api.getBibles();
    console.log('Available Bibles:', bibles.bibles.length);
    
    // Get books in first Bible
    const books = await api.getBibleBooks({ bibleId: 1 });
    console.log(`Bible has ${books.numBooks} books`);
    
    // Get chapters in first book
    const chapters = await api.getBibleChapters({ bibleId: 1, bookNum: 1 });
    console.log(`First book has ${chapters.numChapters} chapters`);
    
    // Get verses from first chapter
    const verses = await api.getBibleVerses({ bibleId: 1, bookNum: 1, chapterNum: 1 });
    console.log(`First chapter has ${verses.verses.length} verses`);
    
    // Get a specific verse
    const verse = await api.getBibleVerseByNumber({ 
      bibleId: 1, 
      bookNum: 1, 
      chapterNum: 1, 
      verseNum: 1 
    });
    console.log('First verse:', verse.text);
    
    // Get audio Bibles
    const audioBibles = await api.getAudioBibles();
    console.log('Available Audio Bibles:', audioBibles.audioBibles.length);
    
  } catch (error) {
    console.error('Error:', error);
  }
}

demonstrateAPI();
```