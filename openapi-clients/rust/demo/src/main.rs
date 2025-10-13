use holy_bible_api::{create_bible_api, apis::default_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Holy Bible API Demo ===\n");

    // Create API client (optionally pass a custom base URL)
    let config = create_bible_api(Some("https://holy-bible-api.com".to_string()));

    // 1. Check API health
    println!("1. Checking API health...");
    match default_api::get_health(&config).await {
        Ok(health) => println!("   API Status: {}\n", health),
        Err(e) => eprintln!("   Error: {:?}\n", e),
    }

    // 2. Get available Bibles
    println!("2. Fetching available Bibles...");
    match default_api::get_bibles(&config, None, None).await {
        Ok(bibles) => {
            println!("   Found {} Bibles", bibles.bibles.len());
            if let Some(first) = bibles.bibles.first() {
                println!(
                    "   First Bible: ID={}, Language={}",
                    first.bible_id, first.language
                );
            }
            println!();
        }
        Err(e) => eprintln!("   Error: {:?}\n", e),
    }

    // 3. Get English Bibles only
    println!("3. Fetching English Bibles...");
    match default_api::get_bibles(&config, Some("en"), None).await {
        Ok(english_bibles) => {
            println!("   Found {} English Bibles\n", english_bibles.bibles.len());
        }
        Err(e) => eprintln!("   Error: {:?}\n", e),
    }

    // 4. Get books in first Bible
    println!("4. Getting books in Bible #1...");
    match default_api::get_bible_books(&config, 1).await {
        Ok(books) => {
            println!("   Bible has {} books\n", books.num_books);
        }
        Err(e) => eprintln!("   Error: {:?}\n", e),
    }

    // 5. Get chapters in first book (Genesis)
    println!("5. Getting chapters in Genesis (Book #1)...");
    match default_api::get_bible_chapters(&config, 1, 1).await {
        Ok(chapters) => {
            println!("   Genesis has {} chapters\n", chapters.num_chapters);
        }
        Err(e) => eprintln!("   Error: {:?}\n", e),
    }

    // 6. Get all verses from Genesis 1
    println!("6. Fetching all verses from Genesis 1...");
    match default_api::get_bible_verses(&config, 1, 1, 1, None, None).await {
        Ok(verses) => {
            println!("   Genesis 1 has {} verses\n", verses.verses.len());
        }
        Err(e) => eprintln!("   Error: {:?}\n", e),
    }

    // 7. Get the first verse
    println!("7. Getting Genesis 1:1...");
    match default_api::get_bible_verse_by_number(&config, 1, 1, 1, 1).await {
        Ok(verse) => {
            println!("   Genesis 1:1 - \"{}\"\n", verse.text);
        }
        Err(e) => eprintln!("   Error: {:?}\n", e),
    }

    // 8. Get verses 1-5
    println!("8. Fetching Genesis 1:1-5...");
    match default_api::get_bible_verses(&config, 1, 1, 1, Some(1), Some(5)).await {
        Ok(verse_range) => {
            println!("   Retrieved {} verses:", verse_range.verses.len());
            for v in &verse_range.verses {
                let preview = if v.text.len() > 50 {
                    format!("{}...", &v.text[..50])
                } else {
                    v.text.clone()
                };
                println!("   - Verse {}: {}", v.verse, preview);
            }
            println!();
        }
        Err(e) => eprintln!("   Error: {:?}\n", e),
    }

    // 9. Get audio Bibles
    println!("9. Fetching available audio Bibles...");
    match default_api::get_audio_bibles(&config, None, None).await {
        Ok(audio_bibles) => {
            println!("   Found {} audio Bibles", audio_bibles.audio_bibles.len());
            if let Some(first) = audio_bibles.audio_bibles.first() {
                println!(
                    "   First Audio Bible: ID={}, Language={}",
                    first.audio_bible_id, first.language
                );
            }
            println!();
        }
        Err(e) => eprintln!("   Error: {:?}\n", e),
    }

    println!("=== Demo Complete! ===");
    Ok(())
}

