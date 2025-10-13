package main

import (
	"context"
	"fmt"
	"log"

	holybibleapi "github.com/emilsharkov/holy-bible-api/openapi-clients/go/package"
)

func main() {
	// Create API client (optionally pass a custom base URL)
	client := holybibleapi.CreateBibleApi("https://holy-bible-api.com")
	ctx := context.Background()

	fmt.Println("=== Holy Bible API Demo ===")
	fmt.Println()

	// Check API health
	fmt.Println("1. Checking API health...")
	health, _, err := client.DefaultAPI.GetHealth(ctx).Execute()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("   API Status: %v\n\n", health)

	// Get available Bibles
	fmt.Println("2. Fetching available Bibles...")
	bibles, _, err := client.DefaultAPI.GetBibles(ctx).Execute()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("   Found %d Bibles\n", len(bibles.Bibles))
	if len(bibles.Bibles) > 0 {
		fmt.Printf("   First Bible: ID=%d, Language=%s\n",
			bibles.Bibles[0].BibleId,
			bibles.Bibles[0].Language)
	}
	fmt.Println()

	// Get English Bibles only
	fmt.Println("3. Fetching English Bibles...")
	language := "en"
	englishBibles, _, err := client.DefaultAPI.GetBibles(ctx).Language(language).Execute()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("   Found %d English Bibles\n\n", len(englishBibles.Bibles))

	// Get books in first Bible
	fmt.Println("4. Getting books in Bible #1...")
	books, _, err := client.DefaultAPI.GetBibleBooks(ctx, 1).Execute()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("   Bible has %d books\n\n", books.NumBooks)

	// Get chapters in first book (Genesis)
	fmt.Println("5. Getting chapters in Genesis (Book #1)...")
	chapters, _, err := client.DefaultAPI.GetBibleChapters(ctx, 1, 1).Execute()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("   Genesis has %d chapters\n\n", chapters.NumChapters)

	// Get all verses from Genesis 1
	fmt.Println("6. Fetching all verses from Genesis 1...")
	verses, _, err := client.DefaultAPI.GetBibleVerses(ctx, 1, 1, 1).Execute()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("   Genesis 1 has %d verses\n\n", len(verses.Verses))

	// Get the first verse
	fmt.Println("7. Getting Genesis 1:1...")
	verse, _, err := client.DefaultAPI.GetBibleVerseByNumber(ctx, 1, 1, 1, 1).Execute()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("   Genesis 1:1 - \"%s\"\n\n", verse.Text)

	// Get verses 1-5
	fmt.Println("8. Fetching Genesis 1:1-5...")
	start := int32(1)
	end := int32(5)
	verseRange, _, err := client.DefaultAPI.GetBibleVerses(ctx, 1, 1, 1).Start(start).End(end).Execute()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("   Retrieved %d verses:\n", len(verseRange.Verses))
	for _, v := range verseRange.Verses {
		fmt.Printf("   - Verse %d: %s\n", v.Verse, v.Text[:min(50, len(v.Text))]+"...")
	}
	fmt.Println()

	// Get audio Bibles
	fmt.Println("9. Fetching available audio Bibles...")
	audioBibles, _, err := client.DefaultAPI.GetAudioBibles(ctx).Execute()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("   Found %d audio Bibles\n", len(audioBibles.AudioBibles))
	if len(audioBibles.AudioBibles) > 0 {
		fmt.Printf("   First Audio Bible: ID=%d, Language=%s\n",
			audioBibles.AudioBibles[0].AudioBibleId,
			audioBibles.AudioBibles[0].Language)
	}
	fmt.Println()

	fmt.Println("=== Demo Complete! ===")
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
