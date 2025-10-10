import { createBibleApi, Bible, BibleVerse } from 'holy-bible-api';

async function main() {
  console.log('🚀 Testing Holy Bible API Package\n');
  
  const api = createBibleApi();
  
  try {
    console.log("Bibles", await api.getBibles());
    console.log("Bible Books", await api.getBibleBooks({ bibleId: 1 }));
    console.log("Bible Chapters", await api.getBibleChapters({ bibleId: 1, bookNum: 1 }));
    console.log("Bible Verses", await api.getBibleVerses({ bibleId: 1, bookNum: 1, chapterNum: 1 }));
    console.log("Audio Bibles", await api.getAudioBibles());
    console.log("Audio Bible Books", await api.getAudioBibleBooks({ audioBibleId: 1 }));
    console.log("Audio Bible Chapters", await api.getAudioBibleChapters({ audioBibleId: 1, bookNum: 1 }));
    console.log("Audio Chapter", await api.getAudioChapter({ audioBibleId: 1, bookNum: 1, chapterNum: 1 }));
  } catch (error) {
    console.error('❌ Error:', error);
  }
}

main();
