from holy_bible_api import create_bible_api

def main():
    print('🚀 Testing Holy Bible API Package\n')
    
    api = create_bible_api()
    
    try:
        print("Bibles:", api.get_bibles())
        print("Bible Books:", api.get_bible_books(bible_id=1))
        print("Bible Chapters:", api.get_bible_chapters(bible_id=1, book_num=1))
        print("Bible Verses:", api.get_bible_verses(bible_id=1, book_num=1, chapter_num=1))
        print("Audio Bibles:", api.get_audio_bibles())
        print("Audio Bible Books:", api.get_audio_bible_books(audio_bible_id=1))
        print("Audio Bible Chapters:", api.get_audio_bible_chapters(audio_bible_id=1, book_num=1))
        print("Audio Chapter:", api.get_audio_chapter(audio_bible_id=1, book_num=1, chapter_num=1))
    except Exception as error:
        print(f'❌ Error: {error}')

if __name__ == "__main__":
    main()